use anyhow::Result;
use chrono::Utc;
use serde_json::{to_value, Map, Value};
use sqlx::PgPool;
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::models::did::{CreateDidRequest, DidDocumentRecord};
use ssi::dids::document::verification_method::ValueOrReference;
use ssi::dids::document::{DIDVerificationMethod, VerificationRelationships};
use ssi::dids::{DIDMethod, DIDResolver, DIDURLBuf, Document, DID, DIDJWK};
use ssi::jwk::{Params as JWKParams, JWK};
use ssi::verification_methods::ssi_core::OneOrMany;

pub async fn create_did_document(
    pool: &PgPool,
    request: CreateDidRequest,
) -> Result<(), anyhow::Error> {

    let jwk: JWK = JWK::generate_ed25519().expect("Failed to generate Ed25519 JWK");

    // 2. Generate DID from the JWK
    let did = DIDJWK::generate(&jwk);
    let did_url_str = format!("{did}#0");
    let did_url = DIDURLBuf::try_from(did_url_str.clone())?;

    // 3. Resolve the base document
    let resolver = DIDJWK;
    let result = resolver.resolve(&did).await?;

    let repr = result.document.document();

    let mut doc: Document = repr.clone();

    let main_vm = DIDVerificationMethod {
        id: did_url,
        type_: String::from("JsonWebKey2020"),
        controller: did.clone().into(),
        properties: {
            let mut props = BTreeMap::new();
            props.insert(
                String::from("publicKeyJwk"),
                to_value(&jwk).expect("Failed to serialize JWK"),
            );
            props
        },
    };
    let key_ref = ValueOrReference::Reference(main_vm.id.clone().into());
    let refs = vec![key_ref];
    doc.verification_method = vec![main_vm];

    doc.verification_relationships = VerificationRelationships {
        authentication: refs.clone(),
        assertion_method: refs.clone(),
        key_agreement: refs.clone(),
        capability_invocation: refs.clone(),
        capability_delegation: refs,
    };

    doc.controller = Some(OneOrMany::One(did.clone()));
    
    println!("{:?}", doc);

    Ok(())
}

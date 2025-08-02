use anyhow::{anyhow, Result};
use chrono::Utc;
use deadpool_redis::Pool as RedisPool;
use redis::AsyncCommands;
use serde_json::{to_value, Map, Value};
use sqlx::PgPool;
use std::collections::BTreeMap;
use uuid::Uuid;

use crate::models::did::{CreateDidRequest, DidDocumentRecord};
use base64_url::decode;
use ssi::claims::jws::Header;
use ssi::claims::jwt::{
    decode_unverified, decode_verify, Issuer, JWTClaims, Nonce, StringOrURI, VerifiableCredential,
};
use ssi::dids::document::verification_method::ValueOrReference;
use ssi::dids::document::{DIDVerificationMethod, VerificationRelationships};
use ssi::dids::{AnyDidMethod, DIDMethod, DIDResolver, DIDURLBuf, Document, DID, DIDJWK};
use ssi::jwk::{Params as JWKParams, JWK};
use ssi::verification_methods::ssi_core::OneOrMany;

pub async fn create_did_document(
    pool: &PgPool,
    request: CreateDidRequest,
) -> Result<DidDocumentRecord> {
    let jwk: JWK = JWK::generate_ed25519().expect("Failed to generate Ed25519 JWK");
    println!("{}", jwk);
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

    let did_document_record = DidDocumentRecord {
        id: Uuid::new_v4(),
        controller: did.to_string(),
        did: did.to_string(),
        document: to_value(&doc)?,
        metadata: None,
        created_at: Utc::now(),
    };

    Ok(did_document_record)
}

pub async fn verify_did_jwt(jwt: &str, pg_pool: &PgPool, redis_pool: &RedisPool) -> Result<()> {
    // Decode JWT without verification
    let claims: JWTClaims<VerifiableCredential> = decode_unverified(jwt)?;
    println!("✅ Claims: {:?}", claims);

    // Extract the DID from the issuer
    let did = extract_issuer_did(&claims)?;
    println!("✅ DID: {did}");

    // Extract JWK from did:jwk
    let public_jwk = extract_jwk_from_did(&did)?;
    println!(
        "🔑 Extracted Public JWK: {}",
        serde_json::to_string_pretty(&public_jwk)?
    );

    // Verify the JWT signature using the public key
    let verified_claims: JWTClaims<VerifiableCredential> = decode_verify(jwt, &public_jwk)?;

    // ✅ Validate audience (aud)
    let expected_audience = "https://your.api";
    let audience = claims
        .registered
        .get::<ssi::claims::jwt::Audience>()
        .ok_or_else(|| anyhow!("❌ Missing audience"))?;

    match &audience.0 {
        ssi::one_or_many::OneOrMany::One(uri) => match uri {
            StringOrURI::String(s) => {
                if s != expected_audience {
                    return Err(anyhow!("❌ Invalid audience"));
                }
            }
            StringOrURI::URI(uri_obj) => {
                if uri_obj.to_string() != expected_audience {
                    return Err(anyhow!("❌ Invalid audience"));
                }
            }
        },
        _ => return Err(anyhow!("❌ Unsupported audience format")),
    }

    let nonce = claims
        .registered
        .get::<Nonce>()
        .ok_or_else(|| anyhow!("❌ Missing nonce in JWT"))?
        .0
        .clone();

    let mut redis_conn = redis_pool.get().await?;

    // Check if nonce has already been used
    if redis_conn.exists(&nonce).await? {
        return Err(anyhow!("⚠️ Nonce has already been used"));
    }

    // Store nonce with expiration (e.g., 5 minutes)
    redis_conn.set_ex(&nonce, "used", 300).await?;

    println!("🔓 Nonce: {}", nonce);
    println!("✅ Signature Verified!");
    println!("🎯 Verified Claims: {:?}", verified_claims);

    Ok(())
}

fn extract_issuer_did(claims: &JWTClaims<VerifiableCredential>) -> Result<String> {
    match claims.registered.get::<Issuer>() {
        Some(issuer) => match &issuer.0 {
            StringOrURI::String(s) => Ok(s.clone()),
            StringOrURI::URI(uri) => Ok(uri.to_string()),
        },
        None => Err(anyhow!("❌ Issuer not found")),
    }
}

fn extract_jwk_from_did(did: &str) -> anyhow::Result<JWK> {
    let prefix = "did:jwk:";
    if !did.starts_with(prefix) {
        anyhow::bail!("Invalid did:jwk format");
    }

    let b64_jwk = &did[prefix.len()..];
    let jwk_bytes = decode(b64_jwk)?;
    let jwk: JWK = serde_json::from_slice(&jwk_bytes)?;

    Ok(jwk)
}

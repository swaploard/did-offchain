use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Represents a Decentralized Identifier (DID) record stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct DID {
    pub id: Uuid,

    /// The W3C-compliant DID string (e.g. "did:jwk:...")
    #[validate(length(min = 10, message = "DID must be at least 10 characters"))]
    pub did: String,

    /// Public keys associated with this DID, stored as JSON
    #[validate(length(min = 1))]
    pub public_keys: String, // You may serialize BTreeMap as JSON string

    /// List of controller DIDs (optional, stored as JSON array)
    pub controllers: Option<String>, // Could be Vec<String>, but easier to serialize manually

    /// Metadata (application-defined, stored as JSON)
    pub metadata: Option<String>,

    /// CID or IPFS hash of off-chain DID Document (optional)
    pub document_cid: Option<String>,

    /// Timestamp when this DID was registered
    pub created_at: DateTime<Utc>,

    /// Timestamp when this DID was last updated
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct CreateDidRequest {
    #[validate(length(min = 3))]
    pub controller: String,

    pub jwk: serde_json::Value,

    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DidDocumentRecord {
    pub id: Uuid,
    pub controller: String,
    pub did: String,
    pub document: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

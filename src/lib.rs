// ContactList - CTCL 2023-2024
// File: src/lib.rs
// Purpose: Commonly used code
// Created: March 22, 2024
// Modified: May 1, 2024

use couch_rs::CouchDocument;
use couch_rs::types::find::FindQuery;
use couch_rs::document::{DocumentCollection, TypedCouchDocument};
use couch_rs::types::document::DocumentId;
use std::error::Error;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, CouchDocument)]
pub struct Contact {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _id: DocumentId,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub _rev: String,
}
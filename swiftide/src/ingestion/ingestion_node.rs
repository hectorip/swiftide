//! This module defines the `IngestionNode` struct and its associated methods.
//!
//! `IngestionNode` represents a unit of data in the ingestion process, containing metadata,
//! the data chunk itself, and an optional vector representation.
//!
//! # Overview
//!
//! The `IngestionNode` struct is designed to encapsulate all necessary information for a single
//! unit of data being processed in the ingestion pipeline. It includes fields for an identifier,
//! file path, data chunk, optional vector representation, and metadata.
//!
//! The struct provides methods to convert the node into an embeddable string format and to
//! calculate a hash value for the node based on its path and chunk.
//!
//! # Usage
//!
//! The `IngestionNode` struct is used throughout the ingestion pipeline to represent and process
//! individual units of data. It is particularly useful in scenarios where metadata and data chunks
//! need to be processed together.
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    path::PathBuf,
};

/// Represents a unit of data in the ingestion process.
///
/// `IngestionNode` encapsulates all necessary information for a single unit of data being processed
/// in the ingestion pipeline. It includes fields for an identifier, file path, data chunk, optional
/// vector representation, and metadata.
#[derive(Debug, Default, Clone)]
pub struct IngestionNode {
    /// Optional identifier for the node.
    pub id: Option<u64>,
    /// File path associated with the node.
    pub path: PathBuf,
    /// Data chunk contained in the node.
    pub chunk: String,
    /// Optional vector representation of the data chunk.
    pub vector: Option<Vec<f32>>,
    /// Metadata associated with the node.
    pub metadata: HashMap<String, String>,
}

impl IngestionNode {
    /// Converts the node into an embeddable string format.
    ///
    /// The embeddable format consists of the metadata formatted as key-value pairs, each on a new line,
    /// followed by the data chunk.
    ///
    /// # Returns
    ///
    /// A string representing the embeddable format of the node.
    pub fn as_embeddable(&self) -> String {
        // Metadata formatted by newlines joined with the chunk
        let metadata = self
            .metadata
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<String>>()
            .join("\n");

        format!("{}\n{}", metadata, self.chunk)
    }

    /// Calculates a hash value for the node based on its path and chunk.
    ///
    /// The hash value is calculated using the default hasher provided by the standard library.
    ///
    /// # Returns
    ///
    /// A 64-bit hash value representing the node.
    pub fn calculate_hash(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for IngestionNode {
    /// Hashes the node based on its path and chunk.
    ///
    /// This method is used by the `calculate_hash` method to generate a hash value for the node.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
        self.chunk.hash(state);
    }
}

//! This module provides integration with the Qdrant vector database.
//! It includes functionalities to interact with Qdrant, such as creating and managing vector collections,
//! storing data, and ensuring proper indexing for efficient searches.

mod ingestion_node;
mod persist;

use anyhow::Result;
use derive_builder::Builder;
use qdrant_client::client::QdrantClient;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{VectorParams, VectorsConfig};

const DEFAULT_COLLECTION_NAME: &str = "swiftide";

/// A struct representing a Qdrant client with configuration options.
///
/// This struct is used to interact with the Qdrant vector database, providing methods to create and manage
/// vector collections, store data, and ensure proper indexing for efficient searches.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Qdrant {
    /// The Qdrant client used to interact with the Qdrant vector database.
    client: QdrantClient,
    /// The name of the collection to be used in Qdrant. Defaults to "swiftide".
    #[builder(default = "DEFAULT_COLLECTION_NAME.to_string()")]
    collection_name: String,
    /// The size of the vectors to be stored in the collection.
    vector_size: usize,
    /// The batch size for operations. Optional.
    #[builder(default, setter(strip_option))]
    batch_size: Option<usize>,
}

impl Qdrant {
    /// Returns a new `QdrantBuilder` for constructing a `Qdrant` instance.
    pub fn builder() -> QdrantBuilder {
        QdrantBuilder::default()
    }

    /// Tries to create a `QdrantBuilder` from a given URL.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that holds the URL for the Qdrant client.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `QdrantBuilder` if successful, or an error otherwise.
    pub fn try_from_url(url: impl AsRef<str>) -> Result<QdrantBuilder> {
        Ok(QdrantBuilder::default().client(QdrantClient::from_url(url.as_ref()).build()?))
    }

    /// Creates an index in the Qdrant collection if it does not already exist.
    ///
    /// This method checks if the specified collection exists in Qdrant. If it does not exist, it creates a new collection
    /// with the specified vector size and cosine distance metric.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub async fn create_index_if_not_exists(&self) -> Result<()> {
        tracing::info!("Checking if collection {} exists", self.collection_name);
        if self.client.collection_exists(&self.collection_name).await? {
            tracing::warn!("Collection {} exists", self.collection_name);
            return Ok(());
        }

        tracing::warn!("Creating collection {}", self.collection_name);
        self.client
            .create_collection(&CreateCollection {
                collection_name: self.collection_name.to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: self.vector_size as u64,
                        distance: Distance::Cosine.into(),
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await?;
        Ok(())
    }
}

//! This module provides an implementation of the `Storage` trait for the `Qdrant` struct.
//! It includes methods for setting up the storage, storing a single node, and storing a batch of nodes.
//! This integration allows the Swiftide project to use Qdrant as a storage backend.

use anyhow::Result;
use async_trait::async_trait;

use crate::traits::Storage;

use super::Qdrant;

#[async_trait]
impl Storage for Qdrant {
    /// Returns the batch size for the Qdrant storage.
    ///
    /// # Returns
    ///
    /// An `Option<usize>` representing the batch size if set, otherwise `None`.
    fn batch_size(&self) -> Option<usize> {
        self.batch_size
    }

    /// Sets up the Qdrant storage by creating the necessary index if it does not exist.
    ///
    /// # Returns
    ///
    /// A `Result<()>` which is `Ok` if the setup is successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the index creation fails.
    #[tracing::instrument(skip_all, err)]
    async fn setup(&self) -> Result<()> {
        tracing::debug!("Setting up Qdrant storage");
        self.create_index_if_not_exists().await
    }

    /// Stores a single ingestion node in the Qdrant storage.
    ///
    /// # Parameters
    ///
    /// - `node`: The `IngestionNode` to be stored.
    ///
    /// # Returns
    ///
    /// A `Result<()>` which is `Ok` if the storage is successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if the node conversion or storage operation fails.
    #[tracing::instrument(skip_all, err, name = "storage.qdrant.store")]
    async fn store(&self, node: crate::ingestion::IngestionNode) -> Result<()> {
        self.client
            .upsert_points_blocking(
                self.collection_name.to_string(),
                None,
                vec![node.try_into()?],
                None,
            )
            .await?;
        Ok(())
    }

    /// Stores a batch of ingestion nodes in the Qdrant storage.
    ///
    /// # Parameters
    ///
    /// - `nodes`: A vector of `IngestionNode` to be stored.
    ///
    /// # Returns
    ///
    /// A `Result<()>` which is `Ok` if the storage is successful, otherwise an error.
    ///
    /// # Errors
    ///
    /// This function will return an error if any node conversion or storage operation fails.
    #[tracing::instrument(skip_all, err, name = "storage.qdrant.batch_store")]
    async fn batch_store(&self, nodes: Vec<crate::ingestion::IngestionNode>) -> Result<()> {
        self.client
            .upsert_points_blocking(
                self.collection_name.to_string(),
                None,
                nodes
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<Vec<_>>>()?,
                None,
            )
            .await?;
        Ok(())
    }
}

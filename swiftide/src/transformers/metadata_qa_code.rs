use std::sync::Arc;

use crate::{ingestion::IngestionNode, SimplePrompt, Transformer};
use anyhow::Result;
use async_trait::async_trait;
use indoc::indoc;

/// `MetadataQACode` is responsible for generating questions and answers based on code chunks.
/// This struct integrates with the ingestion pipeline to enhance the metadata of each code chunk
/// by adding relevant questions and answers.
#[derive(Debug)]
pub struct MetadataQACode {
    client: Arc<dyn SimplePrompt>,
    prompt: String,
    num_questions: usize,
    concurrency: Option<usize>,
}

impl MetadataQACode {
    /// Creates a new instance of `MetadataQACode`.
    ///
    /// # Arguments
    ///
    /// * `client` - An implementation of the `SimplePrompt` trait used to generate questions and answers.
    ///
    /// # Returns
    ///
    /// A new instance of `MetadataQACode` with a default prompt and a default number of questions.
    pub fn new(client: impl SimplePrompt + 'static) -> Self {
        Self {
            client: Arc::new(client),
            prompt: default_prompt(),
            num_questions: 5,
            concurrency: None,
        }
    }

    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = Some(concurrency);
        self
    }
}

/// Returns the default prompt template for generating questions and answers.
///
/// This template includes placeholders for the number of questions and the code chunk.
///
/// # Returns
///
/// A string representing the default prompt template.
fn default_prompt() -> String {
    indoc! {r#"

            # Task
            Your task is to generate questions and answers for the given code. 

            Given that somebody else might ask questions about the code, consider things like:
            * What does this code do?
            * What other internal parts does the code use?
            * Does this code have any dependencies?
            * What are some potential use cases for this code?
            * ... and so on

            # Constraints 
            * Generate only {questions} questions and answers.
            * Only respond in the example format
            * Only respond with questions and answers that can be derived from the code.

            # Example
            Respond in the following example format and do not include anything else:

            ```
            Q1: What does this code do?
            A1: It transforms strings into integers.
            Q2: What other internal parts does the code use?
            A2: A hasher to hash the strings.
            ```

            # Code
            ```
            {code}
            ```

        "#}
    .to_string()
}

#[async_trait]
impl Transformer for MetadataQACode {
    /// Asynchronously transforms an `IngestionNode` by generating questions and answers for its code chunk.
    ///
    /// This method uses the `SimplePrompt` client to generate questions and answers based on the code chunk
    /// and adds this information to the node's metadata.
    ///
    /// # Arguments
    ///
    /// * `node` - The `IngestionNode` to be transformed.
    ///
    /// # Returns
    ///
    /// A result containing the transformed `IngestionNode` or an error if the transformation fails.
    ///
    /// # Errors
    ///
    /// This function will return an error if the `SimplePrompt` client fails to generate a response.
    #[tracing::instrument(skip_all, name = "transformers.metadata_qa_code")]
    async fn transform_node(&self, mut node: IngestionNode) -> Result<IngestionNode> {
        let prompt = self
            .prompt
            .replace("{questions}", &self.num_questions.to_string())
            .replace("{code}", &node.chunk);

        let response = self.client.prompt(&prompt).await?;

        node.metadata
            .insert("Questions and Answers".to_string(), response);

        Ok(node)
    }

    fn concurrency(&self) -> Option<usize> {
        self.concurrency
    }
}

use anyhow::Result;
use openai::embeddings::Embedding;
use qdrant_client::{
    prelude::{Payload, QdrantClient},
    qdrant::{
        vectors_config::Config, with_payload_selector::SelectorOptions, CreateCollection, Distance,
        PointStruct, ScoredPoint, SearchPoints, VectorParams, VectorsConfig, WithPayloadSelector,
    },
};
use serde_json::json;

use crate::{contents::File, errors::EmbeddingError};

static COLLECTION: &str = "docs";

pub struct VectorDB {
    client: QdrantClient,
    id: u64,
}

impl VectorDB {
    pub fn new(client: QdrantClient) -> Self {
        Self { client, id: 0 }
    }

    pub async fn reset_collection(&self) -> Result<()> {
        self.client.delete_collection(COLLECTION).await?;

        self.client
            .create_collection(&CreateCollection {
                collection_name: COLLECTION.to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 1536,
                        distance: Distance::Cosine.into(),
                        hnsw_config: None,
                        quantization_config: None,
                        on_disk: None,
                    })),
                }),
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    pub async fn upsert_embedding(&mut self, embedding: Embedding, file: &File) -> Result<()> {
        let payload: Payload = json!({
            "id": file.path.clone(),
        })
        .try_into()
        .map_err(|_| EmbeddingError {})?;

        println!("Embedded: {}", file.path);

        let vec: Vec<f32> = embedding.vec.iter().map(|&x| x as f32).collect();

        let points = vec![PointStruct::new(self.id, vec, payload)];
        self.client
            .upsert_points(COLLECTION, None, points, None)
            .await?;
        self.id += 1;

        Ok(())
    }

    pub async fn search(&self, embedding: Embedding) -> Result<ScoredPoint> {
        let vec: Vec<f32> = embedding.vec.iter().map(|&x| x as f32).collect();

        let payload_selector = WithPayloadSelector {
            selector_options: Some(SelectorOptions::Enable(true)),
        };

        let search_points = SearchPoints {
            collection_name: COLLECTION.to_string(),
            vector: vec,
            limit: 1,
            with_payload: Some(payload_selector),
            ..Default::default()
        };

        let search_result = self.client.search_points(&search_points).await?;
        let result = search_result.result[0].clone();
        Ok(result)
    }
}

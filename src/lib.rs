//! # BenchGecko
//!
//! Official Rust SDK for the [BenchGecko](https://benchgecko.ai) API.
//!
//! Query AI model data, benchmark scores, and run side-by-side
//! model comparisons programmatically.
//!
//! ## Quick Start
//!
//! ```no_run
//! use benchgecko::BenchGecko;
//!
//! fn main() -> Result<(), benchgecko::Error> {
//!     let client = BenchGecko::new();
//!     let models = client.models()?;
//!     println!("Tracking {} models", models.len());
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Errors that can occur when interacting with the BenchGecko API.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// API returned a non-success status code.
    #[error("API error (status {status}): {message}")]
    Api { status: u16, message: String },

    /// Invalid input provided to a client method.
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

/// An AI model tracked by BenchGecko.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Unique identifier.
    pub id: Option<String>,
    /// Display name.
    pub name: Option<String>,
    /// URL slug.
    pub slug: Option<String>,
    /// Provider name (e.g., "OpenAI", "Anthropic").
    pub provider: Option<String>,
    /// Additional fields returned by the API.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A benchmark tracked by BenchGecko.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Benchmark {
    /// Unique identifier.
    pub id: Option<String>,
    /// Benchmark name.
    pub name: Option<String>,
    /// URL slug.
    pub slug: Option<String>,
    /// Category (e.g., "coding", "reasoning").
    pub category: Option<String>,
    /// Additional fields.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Result of comparing two or more models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    /// Per-model comparison data.
    pub models: Option<Vec<serde_json::Value>>,
    /// Additional fields.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Client for the BenchGecko API.
///
/// Provides blocking HTTP methods to query models, benchmarks,
/// and perform model comparisons.
pub struct BenchGecko {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl BenchGecko {
    /// Create a new client with default settings.
    ///
    /// Uses `https://benchgecko.ai` as the base URL.
    pub fn new() -> Self {
        Self::with_base_url("https://benchgecko.ai")
    }

    /// Create a new client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent("benchgecko-rust/0.1.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client,
        }
    }

    /// List all AI models tracked by BenchGecko.
    ///
    /// Returns a vector of [`Model`] structs with metadata,
    /// benchmark scores, and pricing information.
    pub fn models(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/api/v1/models", self.base_url);
        let resp = self.client.get(&url).send()?;
        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().unwrap_or_default(),
            });
        }
        Ok(resp.json()?)
    }

    /// List all benchmarks tracked by BenchGecko.
    ///
    /// Returns a vector of [`Benchmark`] structs with name,
    /// category, and description.
    pub fn benchmarks(&self) -> Result<Vec<Benchmark>, Error> {
        let url = format!("{}/api/v1/benchmarks", self.base_url);
        let resp = self.client.get(&url).send()?;
        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().unwrap_or_default(),
            });
        }
        Ok(resp.json()?)
    }

    /// Compare two or more AI models side by side.
    ///
    /// Accepts a slice of model slugs and returns a structured comparison.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] if fewer than 2 models are provided.
    pub fn compare(&self, models: &[&str]) -> Result<ComparisonResult, Error> {
        if models.len() < 2 {
            return Err(Error::InvalidInput(
                "At least 2 models are required for comparison".to_string(),
            ));
        }
        let url = format!("{}/api/v1/compare", self.base_url);
        let resp = self
            .client
            .get(&url)
            .query(&[("models", models.join(","))])
            .send()?;
        if !resp.status().is_success() {
            return Err(Error::Api {
                status: resp.status().as_u16(),
                message: resp.text().unwrap_or_default(),
            });
        }
        Ok(resp.json()?)
    }
}

impl Default for BenchGecko {
    fn default() -> Self {
        Self::new()
    }
}

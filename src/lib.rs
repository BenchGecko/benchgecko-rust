//! # BenchGecko
//!
//! Rust SDK for exploring AI model benchmarks, comparing performance,
//! and estimating inference costs. Built on data from [BenchGecko](https://benchgecko.ai).
//!
//! ## Quick Start
//!
//! ```rust
//! use benchgecko::{Model, BenchmarkCategory, compare_models, estimate_cost};
//!
//! let gpt4 = Model::new("gpt-4o", "OpenAI")
//!     .with_context_window(128_000)
//!     .with_score(BenchmarkCategory::Reasoning, 92.3)
//!     .with_score(BenchmarkCategory::Coding, 89.1)
//!     .with_pricing(2.50, 10.00);
//!
//! let claude = Model::new("claude-sonnet-4", "Anthropic")
//!     .with_context_window(200_000)
//!     .with_score(BenchmarkCategory::Reasoning, 94.1)
//!     .with_score(BenchmarkCategory::Coding, 93.7)
//!     .with_pricing(3.00, 15.00);
//!
//! let result = compare_models(&gpt4, &claude);
//! assert!(result.winner().name == "claude-sonnet-4");
//!
//! let cost = estimate_cost(&gpt4, 1_000_000, 500_000).unwrap();
//! assert!(cost > 0.0);
//! ```

use std::collections::HashMap;
use std::fmt;

/// Categories of AI benchmarks tracked by BenchGecko.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BenchmarkCategory {
    /// Mathematical and logical reasoning (GSM8K, MATH, ARC)
    Reasoning,
    /// Code generation and understanding (HumanEval, MBPP, SWE-bench)
    Coding,
    /// General knowledge and comprehension (MMLU, HellaSwag)
    Knowledge,
    /// Instruction following and conversational ability
    Instruction,
    /// Multilingual understanding and translation
    Multilingual,
    /// Safety, alignment, and refusal accuracy
    Safety,
    /// Long-context retrieval and comprehension
    LongContext,
    /// Vision and multimodal tasks
    Vision,
    /// Agent and tool-use capabilities
    Agentic,
}

impl fmt::Display for BenchmarkCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reasoning => write!(f, "Reasoning"),
            Self::Coding => write!(f, "Coding"),
            Self::Knowledge => write!(f, "Knowledge"),
            Self::Instruction => write!(f, "Instruction"),
            Self::Multilingual => write!(f, "Multilingual"),
            Self::Safety => write!(f, "Safety"),
            Self::LongContext => write!(f, "Long Context"),
            Self::Vision => write!(f, "Vision"),
            Self::Agentic => write!(f, "Agentic"),
        }
    }
}

/// Performance tier assigned to a model based on aggregate scores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModelTier {
    /// Elite models scoring 90+ across categories
    S,
    /// Strong models scoring 80-89
    A,
    /// Capable models scoring 70-79
    B,
    /// Budget or older models scoring 60-69
    C,
    /// Entry-level or legacy models below 60
    D,
}

impl fmt::Display for ModelTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::S => write!(f, "S-Tier"),
            Self::A => write!(f, "A-Tier"),
            Self::B => write!(f, "B-Tier"),
            Self::C => write!(f, "C-Tier"),
            Self::D => write!(f, "D-Tier"),
        }
    }
}

/// An AI model with benchmark scores and pricing information.
///
/// Use the builder pattern to construct models with scores and pricing:
///
/// ```rust
/// use benchgecko::{Model, BenchmarkCategory};
///
/// let model = Model::new("gpt-4o", "OpenAI")
///     .with_context_window(128_000)
///     .with_score(BenchmarkCategory::Reasoning, 92.3)
///     .with_pricing(2.50, 10.00);
/// ```
#[derive(Debug, Clone)]
pub struct Model {
    /// Model identifier (e.g., "gpt-4o", "claude-sonnet-4")
    pub name: String,
    /// Provider name (e.g., "OpenAI", "Anthropic")
    pub provider: String,
    /// Maximum context window in tokens
    pub context_window: Option<u64>,
    /// Benchmark scores by category (0.0 - 100.0)
    pub scores: HashMap<BenchmarkCategory, f64>,
    /// Input price per million tokens in USD
    pub input_price_per_mtok: Option<f64>,
    /// Output price per million tokens in USD
    pub output_price_per_mtok: Option<f64>,
}

impl Model {
    /// Create a new model with a name and provider.
    pub fn new(name: &str, provider: &str) -> Self {
        Self {
            name: name.to_string(),
            provider: provider.to_string(),
            context_window: None,
            scores: HashMap::new(),
            input_price_per_mtok: None,
            output_price_per_mtok: None,
        }
    }

    /// Set the context window size in tokens.
    pub fn with_context_window(mut self, tokens: u64) -> Self {
        self.context_window = Some(tokens);
        self
    }

    /// Add a benchmark score for a category (0.0 - 100.0).
    ///
    /// Scores are clamped to the valid range.
    pub fn with_score(mut self, category: BenchmarkCategory, score: f64) -> Self {
        self.scores.insert(category, score.clamp(0.0, 100.0));
        self
    }

    /// Set pricing in USD per million tokens.
    pub fn with_pricing(mut self, input_per_mtok: f64, output_per_mtok: f64) -> Self {
        self.input_price_per_mtok = Some(input_per_mtok);
        self.output_price_per_mtok = Some(output_per_mtok);
        self
    }

    /// Calculate the average score across all benchmarked categories.
    ///
    /// Returns `None` if no scores are recorded.
    pub fn average_score(&self) -> Option<f64> {
        if self.scores.is_empty() {
            return None;
        }
        let total: f64 = self.scores.values().sum();
        Some(total / self.scores.len() as f64)
    }

    /// Determine the model's performance tier based on average score.
    ///
    /// Returns `None` if no scores are recorded.
    pub fn tier(&self) -> Option<ModelTier> {
        self.average_score().map(|avg| {
            if avg >= 90.0 {
                ModelTier::S
            } else if avg >= 80.0 {
                ModelTier::A
            } else if avg >= 70.0 {
                ModelTier::B
            } else if avg >= 60.0 {
                ModelTier::C
            } else {
                ModelTier::D
            }
        })
    }

    /// Get the score for a specific benchmark category.
    pub fn score(&self, category: BenchmarkCategory) -> Option<f64> {
        self.scores.get(&category).copied()
    }

    /// Check whether pricing information is available.
    pub fn has_pricing(&self) -> bool {
        self.input_price_per_mtok.is_some() && self.output_price_per_mtok.is_some()
    }

    /// Compute a value score: average benchmark performance per dollar
    /// (based on a blended price of 1M input + 1M output tokens).
    ///
    /// Higher is better. Returns `None` if pricing or scores are missing.
    pub fn value_score(&self) -> Option<f64> {
        let avg = self.average_score()?;
        let input_price = self.input_price_per_mtok?;
        let output_price = self.output_price_per_mtok?;
        let blended = input_price + output_price;
        if blended <= 0.0 {
            return None;
        }
        Some(avg / blended)
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.provider)?;
        if let Some(tier) = self.tier() {
            write!(f, " [{tier}]")?;
        }
        Ok(())
    }
}

/// Result of comparing two models across benchmark categories.
#[derive(Debug, Clone)]
pub struct ComparisonResult {
    /// The first model in the comparison
    pub model_a: Model,
    /// The second model in the comparison
    pub model_b: Model,
    /// Score differences by category (positive = model_a leads)
    pub deltas: HashMap<BenchmarkCategory, f64>,
    /// Categories where model A scores higher
    pub a_wins: Vec<BenchmarkCategory>,
    /// Categories where model B scores higher
    pub b_wins: Vec<BenchmarkCategory>,
    /// Categories with identical scores
    pub ties: Vec<BenchmarkCategory>,
}

impl ComparisonResult {
    /// Return a reference to the model with the higher average score.
    ///
    /// In case of a tie, returns model A.
    pub fn winner(&self) -> &Model {
        let avg_a = self.model_a.average_score().unwrap_or(0.0);
        let avg_b = self.model_b.average_score().unwrap_or(0.0);
        if avg_b > avg_a {
            &self.model_b
        } else {
            &self.model_a
        }
    }

    /// Return the number of categories compared.
    pub fn categories_compared(&self) -> usize {
        self.deltas.len()
    }

    /// Get the score delta for a specific category.
    ///
    /// Positive means model A leads; negative means model B leads.
    pub fn delta(&self, category: BenchmarkCategory) -> Option<f64> {
        self.deltas.get(&category).copied()
    }
}

impl fmt::Display for ComparisonResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} vs {}", self.model_a.name, self.model_b.name)?;
        writeln!(f, "  {} wins: {} categories", self.model_a.name, self.a_wins.len())?;
        writeln!(f, "  {} wins: {} categories", self.model_b.name, self.b_wins.len())?;
        writeln!(f, "  Ties: {}", self.ties.len())?;
        write!(f, "  Overall winner: {}", self.winner().name)
    }
}

/// Compare two models across all mutually-scored benchmark categories.
///
/// Only categories where both models have scores are included in the comparison.
///
/// # Example
///
/// ```rust
/// use benchgecko::{Model, BenchmarkCategory, compare_models};
///
/// let a = Model::new("model-a", "Provider A")
///     .with_score(BenchmarkCategory::Coding, 85.0);
/// let b = Model::new("model-b", "Provider B")
///     .with_score(BenchmarkCategory::Coding, 91.0);
///
/// let result = compare_models(&a, &b);
/// assert_eq!(result.winner().name, "model-b");
/// ```
pub fn compare_models(a: &Model, b: &Model) -> ComparisonResult {
    let mut deltas = HashMap::new();
    let mut a_wins = Vec::new();
    let mut b_wins = Vec::new();
    let mut ties = Vec::new();

    for (category, &score_a) in &a.scores {
        if let Some(&score_b) = b.scores.get(category) {
            let delta = score_a - score_b;
            deltas.insert(*category, delta);

            if (delta).abs() < f64::EPSILON {
                ties.push(*category);
            } else if delta > 0.0 {
                a_wins.push(*category);
            } else {
                b_wins.push(*category);
            }
        }
    }

    ComparisonResult {
        model_a: a.clone(),
        model_b: b.clone(),
        deltas,
        a_wins,
        b_wins,
        ties,
    }
}

/// Estimate the cost of a request in USD given token counts.
///
/// Returns `None` if the model has no pricing information.
///
/// # Example
///
/// ```rust
/// use benchgecko::{Model, estimate_cost};
///
/// let model = Model::new("gpt-4o", "OpenAI")
///     .with_pricing(2.50, 10.00);
///
/// let cost = estimate_cost(&model, 1_000, 500).unwrap();
/// assert!((cost - 0.0075).abs() < 0.0001);
/// ```
pub fn estimate_cost(model: &Model, input_tokens: u64, output_tokens: u64) -> Option<f64> {
    let input_price = model.input_price_per_mtok?;
    let output_price = model.output_price_per_mtok?;

    let input_cost = (input_tokens as f64 / 1_000_000.0) * input_price;
    let output_cost = (output_tokens as f64 / 1_000_000.0) * output_price;

    Some(input_cost + output_cost)
}

/// Filter and rank models by a specific benchmark category.
///
/// Returns models sorted by descending score in the given category.
/// Models without a score in that category are excluded.
pub fn rank_by_category(models: &[Model], category: BenchmarkCategory) -> Vec<(&Model, f64)> {
    let mut ranked: Vec<(&Model, f64)> = models
        .iter()
        .filter_map(|m| m.score(category).map(|s| (m, s)))
        .collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    ranked
}

/// Filter models by performance tier.
pub fn filter_by_tier(models: &[Model], tier: ModelTier) -> Vec<&Model> {
    models.iter().filter(|m| m.tier() == Some(tier)).collect()
}

/// Find the most cost-effective model from a slice, based on value score.
///
/// Returns `None` if no models have both pricing and benchmark data.
pub fn best_value(models: &[Model]) -> Option<&Model> {
    models
        .iter()
        .filter_map(|m| m.value_score().map(|v| (m, v)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(m, _)| m)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_model(name: &str, reasoning: f64, coding: f64) -> Model {
        Model::new(name, "TestProvider")
            .with_score(BenchmarkCategory::Reasoning, reasoning)
            .with_score(BenchmarkCategory::Coding, coding)
            .with_pricing(3.0, 15.0)
            .with_context_window(128_000)
    }

    #[test]
    fn test_average_score() {
        let model = sample_model("test", 90.0, 80.0);
        assert!((model.average_score().unwrap() - 85.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_tier_classification() {
        assert_eq!(sample_model("s", 95.0, 92.0).tier(), Some(ModelTier::S));
        assert_eq!(sample_model("a", 85.0, 82.0).tier(), Some(ModelTier::A));
        assert_eq!(sample_model("b", 75.0, 72.0).tier(), Some(ModelTier::B));
        assert_eq!(sample_model("c", 65.0, 62.0).tier(), Some(ModelTier::C));
        assert_eq!(sample_model("d", 55.0, 52.0).tier(), Some(ModelTier::D));
    }

    #[test]
    fn test_empty_scores() {
        let model = Model::new("empty", "None");
        assert!(model.average_score().is_none());
        assert!(model.tier().is_none());
    }

    #[test]
    fn test_compare_models() {
        let a = sample_model("alpha", 90.0, 80.0);
        let b = sample_model("beta", 85.0, 95.0);
        let result = compare_models(&a, &b);
        assert_eq!(result.winner().name, "beta");
        assert_eq!(result.a_wins.len(), 1);
        assert_eq!(result.b_wins.len(), 1);
    }

    #[test]
    fn test_estimate_cost() {
        let model = Model::new("test", "Provider").with_pricing(2.50, 10.00);
        let cost = estimate_cost(&model, 1_000_000, 500_000).unwrap();
        assert!((cost - 7.5).abs() < 0.01);
    }

    #[test]
    fn test_estimate_cost_no_pricing() {
        let model = Model::new("test", "Provider");
        assert!(estimate_cost(&model, 1000, 500).is_none());
    }

    #[test]
    fn test_rank_by_category() {
        let models = vec![
            sample_model("low", 70.0, 65.0),
            sample_model("high", 95.0, 98.0),
            sample_model("mid", 82.0, 80.0),
        ];
        let ranked = rank_by_category(&models, BenchmarkCategory::Coding);
        assert_eq!(ranked[0].0.name, "high");
        assert_eq!(ranked[2].0.name, "low");
    }

    #[test]
    fn test_filter_by_tier() {
        let models = vec![
            sample_model("s1", 95.0, 92.0),
            sample_model("a1", 85.0, 82.0),
            sample_model("s2", 91.0, 93.0),
        ];
        let s_tier = filter_by_tier(&models, ModelTier::S);
        assert_eq!(s_tier.len(), 2);
    }

    #[test]
    fn test_best_value() {
        let cheap = Model::new("cheap", "Budget")
            .with_score(BenchmarkCategory::Reasoning, 80.0)
            .with_pricing(0.10, 0.30);
        let expensive = Model::new("expensive", "Premium")
            .with_score(BenchmarkCategory::Reasoning, 95.0)
            .with_pricing(15.0, 60.0);
        let models = vec![cheap, expensive];
        let best = best_value(&models).unwrap();
        assert_eq!(best.name, "cheap");
    }

    #[test]
    fn test_value_score() {
        let model = Model::new("test", "P")
            .with_score(BenchmarkCategory::Reasoning, 80.0)
            .with_pricing(5.0, 15.0);
        let vs = model.value_score().unwrap();
        assert!((vs - 4.0).abs() < 0.01); // 80 / (5+15) = 4.0
    }

    #[test]
    fn test_score_clamping() {
        let model = Model::new("test", "P")
            .with_score(BenchmarkCategory::Reasoning, 150.0)
            .with_score(BenchmarkCategory::Coding, -10.0);
        assert_eq!(model.score(BenchmarkCategory::Reasoning), Some(100.0));
        assert_eq!(model.score(BenchmarkCategory::Coding), Some(0.0));
    }

    #[test]
    fn test_model_display() {
        let model = sample_model("gpt-4o", 92.0, 91.0);
        let display = format!("{model}");
        assert!(display.contains("gpt-4o"));
        assert!(display.contains("S-Tier"));
    }
}

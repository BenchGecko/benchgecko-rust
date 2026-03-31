# benchgecko

Rust SDK for [BenchGecko](https://benchgecko.ai) -- the data platform for comparing AI model benchmarks, estimating inference costs, and exploring performance across providers.

## Overview

`benchgecko` gives you typed, idiomatic Rust primitives for working with LLM benchmark data. Build comparison tools, cost calculators, model selectors, and leaderboard UIs without scraping or maintaining your own dataset.

The crate provides:

- **Model** struct with builder pattern for constructing models with scores and pricing
- **BenchmarkCategory** enum covering 9 evaluation dimensions (Reasoning, Coding, Knowledge, Instruction, Multilingual, Safety, Long Context, Vision, Agentic)
- **ModelTier** classification (S through D) based on aggregate performance
- **compare_models()** for head-to-head analysis across shared categories
- **estimate_cost()** for calculating inference spend from token counts
- **rank_by_category()** and **filter_by_tier()** for leaderboard and filtering operations
- **best_value()** for finding the most cost-effective model in a set
- **Value score** computation that balances performance against price

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
benchgecko = "0.1"
```

## Quick Start

```rust
use benchgecko::{Model, BenchmarkCategory, compare_models, estimate_cost};

// Define models with benchmark scores and pricing
let gpt4 = Model::new("gpt-4o", "OpenAI")
    .with_context_window(128_000)
    .with_score(BenchmarkCategory::Reasoning, 92.3)
    .with_score(BenchmarkCategory::Coding, 89.1)
    .with_score(BenchmarkCategory::Knowledge, 88.7)
    .with_pricing(2.50, 10.00);

let claude = Model::new("claude-sonnet-4", "Anthropic")
    .with_context_window(200_000)
    .with_score(BenchmarkCategory::Reasoning, 94.1)
    .with_score(BenchmarkCategory::Coding, 93.7)
    .with_score(BenchmarkCategory::Knowledge, 91.2)
    .with_pricing(3.00, 15.00);

// Compare across shared categories
let result = compare_models(&gpt4, &claude);
println!("Winner: {}", result.winner().name);
println!("Categories compared: {}", result.categories_compared());

// Estimate cost for a request
let cost = estimate_cost(&gpt4, 5_000, 2_000).unwrap();
println!("Estimated cost: ${:.4}", cost);
```

## Tier Classification

Models are classified into tiers based on their average benchmark score:

| Tier | Average Score | Description |
|------|--------------|-------------|
| S | 90+ | Elite frontier models |
| A | 80-89 | Strong general-purpose models |
| B | 70-79 | Capable mid-range models |
| C | 60-69 | Budget or older generation |
| D | <60 | Entry-level or legacy |

```rust
use benchgecko::{Model, BenchmarkCategory, ModelTier, filter_by_tier};

let models = vec![
    Model::new("frontier-1", "Lab A")
        .with_score(BenchmarkCategory::Reasoning, 95.0)
        .with_score(BenchmarkCategory::Coding, 93.0),
    Model::new("budget-1", "Lab B")
        .with_score(BenchmarkCategory::Reasoning, 72.0)
        .with_score(BenchmarkCategory::Coding, 68.0),
];

let elite = filter_by_tier(&models, ModelTier::S);
println!("S-Tier models: {}", elite.len());
```

## Value Analysis

Find the best performance-per-dollar model using the built-in value score, which divides average benchmark performance by blended token price:

```rust
use benchgecko::{Model, BenchmarkCategory, best_value};

let models = vec![
    Model::new("expensive-frontier", "Premium Labs")
        .with_score(BenchmarkCategory::Reasoning, 96.0)
        .with_pricing(15.00, 60.00),
    Model::new("efficient-mid", "Budget AI")
        .with_score(BenchmarkCategory::Reasoning, 82.0)
        .with_pricing(0.15, 0.60),
];

if let Some(best) = best_value(&models) {
    println!("Best value: {} (score/dollar: {:.1})", best.name, best.value_score().unwrap());
}
```

## Benchmark Categories

The `BenchmarkCategory` enum covers the major evaluation dimensions tracked by [BenchGecko](https://benchgecko.ai):

| Category | Typical Benchmarks |
|----------|-------------------|
| Reasoning | GSM8K, MATH, ARC-Challenge |
| Coding | HumanEval, MBPP, SWE-bench |
| Knowledge | MMLU, HellaSwag, TriviaQA |
| Instruction | MT-Bench, AlpacaEval |
| Multilingual | MGSM, XLSum |
| Safety | TruthfulQA, BBQ |
| LongContext | RULER, Needle-in-a-Haystack |
| Vision | MMMU, MathVista |
| Agentic | WebArena, SWE-bench |

## Data Source

Benchmark data, model metadata, and pricing information are maintained by [BenchGecko](https://benchgecko.ai). Visit the platform for live leaderboards, interactive comparisons, and the full model database covering 300+ models across 50+ providers.

## License

MIT

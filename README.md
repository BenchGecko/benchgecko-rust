# BenchGecko Rust SDK

Official Rust client for the [BenchGecko](https://benchgecko.ai) API. Query AI model data, benchmark scores, and run side-by-side comparisons from Rust applications.

BenchGecko tracks every major AI model, benchmark, and provider. This crate wraps the public REST API with strongly typed Rust structs, proper error handling, and both blocking and async-ready patterns.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
benchgecko = "0.1"
```

## Quick Start

```rust
use benchgecko::BenchGecko;

fn main() -> Result<(), benchgecko::Error> {
    let client = BenchGecko::new();

    // List all tracked AI models
    let models = client.models()?;
    println!("Tracking {} models", models.len());

    // List all benchmarks
    let benchmarks = client.benchmarks()?;
    for b in &benchmarks {
        println!("{}", b.name.as_deref().unwrap_or("unnamed"));
    }

    // Compare two models head-to-head
    let comparison = client.compare(&["gpt-4o", "claude-opus-4"])?;
    println!("{:?}", comparison);

    Ok(())
}
```

## API Reference

### `BenchGecko::new()`

Create a client with the default base URL (`https://benchgecko.ai`).

### `BenchGecko::with_base_url(url)`

Create a client with a custom API base URL for testing or self-hosted instances.

### `client.models() -> Result<Vec<Model>, Error>`

Fetch all AI models. Each `Model` struct contains optional fields for name, provider, slug, and a `HashMap` for additional metadata like pricing and scores.

### `client.benchmarks() -> Result<Vec<Benchmark>, Error>`

Fetch all benchmarks. Each `Benchmark` struct contains name, slug, category, and extra metadata.

### `client.compare(models) -> Result<ComparisonResult, Error>`

Compare two or more models. Pass a slice of model slug strings. Returns a `ComparisonResult` with per-model data.

## Error Handling

The `Error` enum covers HTTP failures, API errors (with status code), and invalid input:

```rust
use benchgecko::{BenchGecko, Error};

let client = BenchGecko::new();
match client.models() {
    Ok(models) => println!("{} models", models.len()),
    Err(Error::Api { status, message }) => {
        eprintln!("API error {}: {}", status, message);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Data Attribution

Data provided by [BenchGecko](https://benchgecko.ai). Model benchmark scores are sourced from official evaluation suites and validated against published results. Pricing data is updated daily from provider APIs.

## Links

- [BenchGecko](https://benchgecko.ai) - AI model benchmarks, pricing, and rankings
- [API Documentation](https://benchgecko.ai/api-docs)
- [GitHub Repository](https://github.com/BenchGecko/benchgecko-rust)

## License

MIT

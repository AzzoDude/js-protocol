# js-protocol

[![Crates.io](https://img.shields.io/crates/v/js-protocol.svg)](https://crates.io/crates/js-protocol)
[![Documentation](https://docs.rs/js-protocol/badge.svg)](https://docs.rs/js-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, fully type-safe Rust representation of the **Chrome DevTools JavaScript Protocol (js_protocol)**, automatically generated from the official protocol definitions.

## 🚀 Key Features

- **Automated Type Generation**: Derived from official V8/Chrome protocol definitions.
- **Intelligent Integer Mapping**:
  - `i32`: Used for coordinates, offsets, and line/column numbers.
  - `u64`: Used for identifiers, counts, sizes, and ordinals.
  - `i64`: Default for general integer values.
- **Recursive Type Safety**: Handles recursive structures via `Box<T>` indirection.
- **Optimized Serialization**: 
  - Uses `serde` with `camelCase` renaming to match the protocol exactly.
  - Optional fields are omitted from JSON if `None` to reduce network overhead.
- **Full Documentation**: Protocol descriptions included as Rustdoc comments.

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
js-protocol = { version = "0.1.1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## 🛠 Usage Example

```rust
use js_protocol::runtime::{EvaluateParams, RemoteObject};

fn main() {
    // Example: Constructing a 'Runtime.evaluate' request
    let params = EvaluateParams {
        expression: "console.log('Hello from Rust!')".to_string(),
        ..Default::default()
    };

    println!("Serialized request: {:?}", serde_json::to_string(&params));
}
```

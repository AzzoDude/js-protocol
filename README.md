# js-protocol

[![Crates.io](https://img.shields.io/crates/v/js-protocol.svg)](https://crates.io/crates/js-protocol)
[![Documentation](https://docs.rs/js-protocol/badge.svg)](https://docs.rs/js-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, zero-allocation, fully compile-safe Rust representation of the **Chrome DevTools JavaScript Protocol (js_protocol)**, generated directly from the official protocol definitions.

---

## 🚀 Key Design Goals & Features

Most auto-generated CDP crates output raw, unidiomatic APIs with substantial runtime allocation overhead. This library is designed from the ground up to solve these issues:

### 1. Idiomatic Rust Naming Conventions
* All generated struct fields, getters, and builder setter methods are translated from the protocol's raw `camelCase` to standard Rust `snake_case` (e.g., `executionContextId` becomes `execution_context_id`, and `objectGroup` becomes `object_group`).
* Standard `#[serde(rename = "...")]` attributes ensure the serialized JSON wire protocol matches the exact formats required by Chrome.

### 2. Zero-Copy String Management
* Utilizes `Cow<'a, str>` instead of allocating heap memory (`String`) for string properties. 
* String arguments in builders use `impl Into<Cow<'a, str>>`, allowing you to pass static string literals (`&str`) or owned strings without unnecessary heap allocations.

### 3. Compile-Time Argument Safety
* The builder pattern differentiates between **required** and **optional** parameters. Required parameters are passed directly as arguments to the `builder(...)` function, guaranteeing protocol compliance at compile time:
  ```rust
  // `expression` is required (passed to builder), `silent` is optional (chained)
  let params = EvaluateParams::builder("1 + 1")
      .silent(true)
      .build();
  ```

### 4. Zero Dependencies & No Async Runtime Lock-in
* Contains **zero bloated dependencies** (depends only on `serde` and `serde_json`).
* Does not include a WebSocket client or force a specific async runtime (like `tokio`). This keeps the package extremely lightweight, compile-fast, and compatible with any async runtime or network stack.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
js-protocol = { version = "0.1.3", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

---

## 🛠 Usage Examples

### 1. Constructing a Request with Optional Parameters
```rust
use js_protocol::runtime::EvaluateParams;

fn main() {
    // 1. Build the command parameters
    let params = EvaluateParams::builder("console.log('Hello from Rust!')")
        .silent(true)
        .build();
    
    // 2. Read-only getters
    println!("Expression to run: {}", params.expression());
    
    // 3. Serialize to wire protocol payload (skips unset Option fields)
    let payload = serde_json::to_string(&params).unwrap();
    println!("Payload: {}", payload);
    // Output: {"expression":"console.log('Hello from Rust!')","silent":true}
}
```

### 2. Handling Command Request/Response Types
Every parameter struct implements `crate::CdpCommand<'a>` which binds it to its command method and its corresponding response (`Returns`) type:

```rust
use js_protocol::runtime::{EvaluateParams, EvaluateReturns};
use js_protocol::CdpCommand;

fn evaluate_js() {
    let params = EvaluateParams::builder("1 + 2").build();

    // The trait binds this command to its method name and response type:
    assert_eq!(EvaluateParams::METHOD, "Runtime.evaluate");
    
    // In your network client:
    // let response_json = websocket.send_command(EvaluateParams::METHOD, &params).await;
    // let response: EvaluateReturns = serde_json::from_str(&response_json).unwrap();
}
```

---

## 🏗 Code Generation Mechanics

The code is dynamically compiled using a custom Python script that performs advanced schema analysis:

1. **Fixed-Point Lifetime Propagation Pass**: The generator performs iterative analysis over the CDP types to detect circular type references, nesting, and dependency hierarchies. It automatically determines which types must have a lifetime parameter (`<'a>`) and wraps recursive structures inside `Box` to prevent infinite-size compilation errors.
2. **HTML/Markdown Escaping**: Schema documentation from the Chrome DevTools Protocol contains raw markdown and HTML brackets. The generator cleans and escapes these brackets into valid Rustdoc format, keeping compilation entirely warning-free.
3. **Domain-Specific Feature Flags**: Every CDP domain is represented by a Rust feature flag. You can optimize compile times by only compiling the domains your project needs:
   ```toml
   # Compile only the runtime and debugger domains
   js-protocol = { version = "0.1.3", default-features = false, features = ["runtime", "debugger"] }
   ```

### Regenerating the Code
To regenerate the Rust modules from a local protocol file:
```bash
python scripts/generate_rust_code.py --name js-protocol
```

---

## ⚖ License

Distributed under the MIT License. See `LICENSE` for more information.

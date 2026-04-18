import subprocess
import shutil
import os
import platform
import sys
import argparse
import json
import re
import urllib.request

dependencies_rs: dict[str, str] = {
    "tokio": "full",
    "serde": "derive",
    "serde_json": ""
}

profile_release_content: str = """
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
"""

def to_camel_case(snake_str):
    components = snake_str.replace('-', '_').split('_')
    return "".join(x[:1].upper() + x[1:] for x in components if x)

def format_rustdoc(description, indent_level=0, is_inner=False):
    if not description: return ""
    indent = " " * indent_level
    symbol = "//! " if is_inner else "/// "
    
    clean_text = description.replace("\\n", "\n").replace("`", "'")
    url_pattern = r'(?<!<)(https?://[^\s)]+)(?!>)'
    clean_text = re.sub(url_pattern, r'<\1>', clean_text)
    
    urls = []
    def url_repl(match):
        urls.append(match.group(0))
        return f"__URL_PLACEHOLDER_{len(urls)-1}__"
    
    placeholder_text = re.sub(r'<https?://[^>]+>', url_repl, clean_text)
    placeholder_text = placeholder_text.replace("<", r"\<").replace(">", r"\>")
    placeholder_text = placeholder_text.replace("[", r"\[").replace("]", r"\]")
    
    for i, url in enumerate(urls):
        placeholder_text = placeholder_text.replace(f"__URL_PLACEHOLDER_{i}__", url)
    
    lines = placeholder_text.split("\n")
    doc_lines = []
    for line in lines:
        clean_line = line.strip()
        doc_lines.append(f"{indent}{symbol}{clean_line}" if clean_line else f"{indent}{symbol}")
    return "\n".join(doc_lines) + "\n"

def get_rust_type(prop, current_struct_name=None):
    base_type = "serde_json::Value"
    is_recursive = False

    if "$ref" in prop:
        ref = prop["$ref"]
        if "." in ref:
            domain, t_name = ref.split(".")
            if t_name == "Value": t_name = "ProtocolValue"
            base_type = f"crate::{domain.lower()}::{t_name}"
            if t_name == current_struct_name: is_recursive = True
        else:
            base_type = ref
            if ref == "Value": base_type = "ProtocolValue"
            if ref == current_struct_name: is_recursive = True

    elif prop.get("type") == "string": base_type = "String"
    elif prop.get("type") == "number": base_type = "f64"
    elif prop.get("type") == "boolean": base_type = "bool"
    elif prop.get("type") == "any": base_type = "serde_json::Value"
    elif prop.get("type") == "array":
        item_type = get_rust_type(prop.get("items", {}))
        base_type = f"Vec<{item_type}>"
    elif prop.get("type") == "integer":
        name = prop.get("name", "").lower()
        if any(k in name for k in ["delta", "offset"]) or name in ["x", "y", "line", "column"]: 
            base_type = "i32"
        elif any(k in name for k in ["id", "count", "index", "size", "length", "ordinal"]): 
            base_type = "u64"
        else: 
            base_type = "i64"
    elif prop.get("type") == "object":
        base_type = "serde_json::Map<String, serde_json::Value>"
        
    if is_recursive:
        base_type = f"Box<{base_type}>"

    if prop.get("optional", False):
        return f"Option<{base_type}>"
    return base_type

def generate_cdp_modules(project_name: str):
    json_path = "js_protocol.json"
    parent_json = os.path.join("..", "js_protocol.json")
    
    if not os.path.exists(json_path) and not os.path.exists(parent_json):
        url = "https://raw.githubusercontent.com/ChromeDevTools/devtools-protocol/refs/heads/master/json/js_protocol.json"
        print(f"Downloading latest JS protocol from {url}...")
        try:
            urllib.request.urlretrieve(url, parent_json)
            json_path = parent_json
        except Exception as e:
            print(f"Failed to download protocol: {e}")
            return
    elif os.path.exists(parent_json):
        json_path = parent_json
        
    with open(json_path, "r", encoding="utf-8") as f:
        schema = json.load(f)

    project_path = ".."
    src_dir = os.path.join(project_path, "src")
    lib_rs_content = ["#![allow(non_snake_case)]", "#![allow(unused_imports)]", "#![allow(dead_code)]", ""]

    all_domains_in_file = [d.get("domain").lower() for d in schema.get("domains", [])]
    
    external_references = ["dom", "page", "network", "target"]
    for stub in external_references:
        if stub not in all_domains_in_file:
            stub_dir = os.path.join(src_dir, stub)
            os.makedirs(stub_dir, exist_ok=True)
            with open(os.path.join(stub_dir, "mod.rs"), "w", encoding="utf-8") as f:
                f.write(f"//! Stub for {stub} domain\n")
                f.write("pub type NodeId = i32;\npub type FrameId = String;\n")
            lib_rs_content.append(f"pub mod {stub};")

    for domain in schema.get("domains", []):
        d_name = domain.get("domain")
        lib_rs_content.append(f"pub mod {d_name.lower()};")
        domain_dir = os.path.join(src_dir, d_name.lower())
        os.makedirs(domain_dir, exist_ok=True)
        
        inner_docs = []
        mod_body = []
        if "description" in domain: inner_docs.append(format_rustdoc(domain['description'], 0, True))

        for t in domain.get("types", []):
            mod_body.append(format_rustdoc(t.get("description"), 0))
            t_id = t.get("id")
            safe_t_id = f"Protocol{t_id}" if t_id == "Value" else t_id

            if "enum" in t:
                mod_body.append("#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]")
                mod_body.append(f"pub enum {safe_t_id} {{")
                for i, e in enumerate(t["enum"]):
                    var = to_camel_case(e)
                    if var == "Self": var = "SelfValue"
                    if i == 0: mod_body.append("    #[default]")
                    mod_body.append(f"    {var},")
                mod_body.append("}\n")
            elif t.get("type") == "object" and "properties" in t:
                mod_body.append("#[derive(Debug, Clone, Serialize, Deserialize, Default)]")
                mod_body.append('#[serde(rename_all = "camelCase")]')
                mod_body.append(f"pub struct {safe_t_id} {{")
                for p in t["properties"]:
                    mod_body.append(format_rustdoc(p.get("description"), 4))
                    p_name = p["name"]
                    r_type = get_rust_type(p, t_id).replace("serde_json::Value", "JsonValue")
                    if "Option<" in r_type: mod_body.append('    #[serde(skip_serializing_if = "Option::is_none")]')
                    if p_name in ["type", "override", "match", "return"]:
                        mod_body.append(f'    #[serde(rename = "{p_name}")]')
                        p_name = f"{p_name}_"
                    mod_body.append(f"    pub {p_name}: {r_type},")
                mod_body.append("}\n")
            else:
                r_type = get_rust_type(t, t_id).replace("serde_json::Value", "JsonValue")
                mod_body.append(f"pub type {safe_t_id} = {r_type};\n")

        for cmd in domain.get("commands", []):
            c_name = to_camel_case(cmd.get("name"))
            for suffix, key in [("Params", "parameters"), ("Returns", "returns")]:
                props = cmd.get(key, [])
                if props:
                    mod_body.append(format_rustdoc(cmd.get("description"), 0))
                    mod_body.append("#[derive(Debug, Clone, Serialize, Deserialize, Default)]")
                    mod_body.append('#[serde(rename_all = "camelCase")]')
                    mod_body.append(f"pub struct {c_name}{suffix} {{")
                    for p in props:
                        mod_body.append(format_rustdoc(p.get("description"), 4))
                        p_name = p["name"]
                        r_type = get_rust_type(p).replace("serde_json::Value", "JsonValue")
                        if "Option<" in r_type: mod_body.append('    #[serde(skip_serializing_if = "Option::is_none")]')
                        if p_name in ["type", "override", "match", "return"]:
                            mod_body.append(f'    #[serde(rename = "{p_name}")]')
                            p_name = f"{p_name}_"
                        mod_body.append(f"    pub {p_name}: {r_type},")
                    mod_body.append("}\n")

        body_text = "\n".join(mod_body)
        mod_code = inner_docs[:]
        if "Serialize" in body_text or "Deserialize" in body_text:
            mod_code.append("use serde::{Serialize, Deserialize};")
        if "JsonValue" in body_text:
            mod_code.append("use serde_json::Value as JsonValue;")
        if len(mod_code) > len(inner_docs):
            mod_code.append("")
        mod_code.extend(mod_body)

        with open(os.path.join(domain_dir, "mod.rs"), "w", encoding="utf-8") as f:
            f.write("\n".join(mod_code))

    with open(os.path.join(src_dir, "lib.rs"), "w", encoding="utf-8") as f:
        f.write("\n".join(lib_rs_content))

def update_cargo_metadata(project_name):
    project_path = ".."
    path = os.path.join(project_path, "Cargo.toml")
    with open(path, "r", encoding="utf-8") as f: lines = f.readlines()
    
    new_lines = []
    if not any("authors =" in l for l in lines):
        for line in lines:
            new_lines.append(line)
            if line.strip() == "[package]":
                new_lines.append(f'authors = ["AzzoDude"]\n')
                new_lines.append(f'description = "Generated Rust types for the Chrome DevTools JS Protocol"\n')
                new_lines.append(f'license = "MIT"\n')
                new_lines.append(f'repository = "https://github.com/AzzoDude/{project_name}"\n')
                new_lines.append(f'readme = "README.md"\n')
                new_lines.append(f'keywords = ["cdp", "browser", "automation", "protocol"]\n')
                new_lines.append(f'categories = ["development-tools", "web-programming"]\n')
    else:
        new_lines = lines
            
    if not any("[profile.release]" in l for l in lines):
        new_lines.append('\n[profile.release]\nopt-level = 3\nlto = "fat"\ncodegen-units = 1\npanic = "abort"\nstrip = true\n')

    with open(path, "w", encoding="utf-8") as f: f.writelines(new_lines)

def update_gitignore():
    project_path = ".."
    path = os.path.join(project_path, ".gitignore")
    
    required_ignores = ["/target/", "Cargo.lock"]
    existing_lines = []
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8") as f:
            existing_lines = [l.strip() for l in f.readlines()]
            
    # Remove generic 'target/' if it exists to avoid ignoring 'src/target'
    if "target/" in existing_lines:
        existing_lines.remove("target/")
        
    for item in required_ignores:
        if item not in existing_lines:
            existing_lines.append(item)
            
    with open(path, "w", encoding="utf-8") as f:
        f.write("\n".join(existing_lines) + "\n")


def generate_readme(project_name):
    project_path = ".."
    path = os.path.join(project_path, "README.md")
    content = f"""# {project_name}

[![Crates.io](https://img.shields.io/crates/v/{project_name}.svg)](https://crates.io/crates/{project_name})
[![Documentation](https://docs.rs/{project_name}/badge.svg)](https://docs.rs/{project_name})
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
{project_name} = "0.1.0"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
```

## 🛠 Usage Example

```rust
use js_protocol::runtime::{{EvaluateParams, RemoteObject}};

fn main() {{
    // Example: Constructing a 'Runtime.evaluate' request
    let params = EvaluateParams {{
        expression: "console.log('Hello from Rust!')".to_string(),
        ..Default::default()
    }};

    println!("Serialized request: {{:?}}", serde_json::to_string(&params));
}}
```
"""
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate Rust types for JS Protocol.")
    parser.add_argument("--name", type=str, required=True, help="Project name")
    parser.add_argument("--release", action="store_true", help="Add release profile")
    parser.add_argument("--source", action="store_true", help="Generate source code")

    args = parser.parse_args()

    if args.source:
        print(f"Generating source code for {args.name}...")
        generate_cdp_modules(args.name)

    if args.release:
        print(f"Updating Cargo metadata for {args.name}...")
        update_cargo_metadata(args.name)
        print("Updating .gitignore...")
        update_gitignore()

    print(f"Generating README for {args.name}...")
    generate_readme(args.name)
    print(f"Generation complete for {args.name}.")
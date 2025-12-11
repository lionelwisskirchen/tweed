
# Tweed
**In-Place WebAssembly (WASM) Interpreter**

[![pipeline status](https://gitlab.lrz.de/aemy/tweed/inplace-wasm-interpreter-prototype/badges/develop/pipeline.svg)](https://gitlab.lrz.de/aemy/tweed/inplace-wasm-interpreter-prototype/-/commits/develop) [![coverage report](https://gitlab.lrz.de/aemy/tweed/inplace-wasm-interpreter-prototype/badges/develop/coverage.svg)](https://gitlab.lrz.de/aemy/tweed/inplace-wasm-interpreter-prototype/-/commits/develop) 

A modular Rust project offering an in-place WASM interpreter and a minimal CLI. 

---

## 📦 **Crates Overview**
The project is split into two core crates:

| Crate         | Purpose                                                             |
|---------------|---------------------------------------------------------------------|
| **`tweed`**   | Core library for the in-place WASM interpreter.                     |
| **`cli`**     | Minimal CLI tool to interact with the interpreter.                  |

---

## 🚀 **Quick Start**  

### **1. Core Library (`tweed`)**  
Add to your `Cargo.toml`:  
```toml  
[dependencies]  
tweed = { path = "./tweed" }  
```

**Example Usage**:  
```rust  
use tweed::{initialize, instantiate, invoke};  

let wasm_bytes = include_bytes!("module.wasm");  
let module: Module = initialize(wasm_bytes)?;

let store: Store = Store::default();

let instance_name = instantiate(&mut module, &mut store, None)?; 
let result = invoke(instance_name, "add", vec![Value::I32(1), Value::I32(2)])?;  
println!("Result: {:?}", result);
```  

---

### **2. CLI Tool (`cli`)**  
A minimal command-line interface to invoke WASM functions.

#### **Installation**  
```sh  
cd cli && cargo install --path .
```  

#### **Usage**  
```sh  
tweed-cli <PATH> invoke <FUNCTION> [PARAMS...]  
```  

**Example**:
The following command invokes the function named `add` with the parameter `1` and `2`. Currently, only `i32` data types are allowed as arguments.
```sh  
tweed-cli ./module.wasm invoke add 1 2  
Result: 3  
```  

---

## 🔗 **Links**  
- [WebAssembly Spec](https://webassembly.github.io/spec/)  
- [A fast in-place interpreter](https://arxiv.org/abs/2205.01183) 

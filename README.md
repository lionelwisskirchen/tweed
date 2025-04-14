
# Tweed
**In-Place WebAssembly (WASM) Interpreter**

A modular Rust project offering an in-place WASM interpreter, microcontroller integration (ESP32-C3), and a minimal CLI. 

---

## 📦 **Crates Overview**
The project is split into three core crates:

| Crate         | Purpose                                                             |
|---------------|---------------------------------------------------------------------|
| **`tweed`**   | Core library for the in-place WASM interpreter.                     |
| **`cli`**     | Minimal CLI tool to interact with the interpreter.                  |
| **`esp32c3`** | Provides an interface for ESP32-C3 microcontrollers to use `tweed`. |

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
let module = initialize(wasm_bytes)?;  
let instance = instantiate(module)?;  
let result = invoke(instance, "add", vec![Value::I32(1), Value::I32(2)])?;  
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
```sh  
tweed-cli ./module.wasm invoke add 1 2  
Result: 3  
```  

---

## 🔗 **Links**  
- [WebAssembly Spec](https://webassembly.github.io/spec/)  
- [A fast in-place interpreter](https://arxiv.org/abs/2205.01183) 

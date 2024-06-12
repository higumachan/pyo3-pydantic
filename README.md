### README

# Rust-Pydantic Converter

A library for efficiently converting between Pydantic's `BaseModel` in Python and Rust `struct`. This library aims to simplify the interoperability between Rust and Python, making it easy to use Rust's performance and safety with Python's ease of use and rich ecosystem.

## Features

- Convert Pydantic `BaseModel` instances to Rust `struct`.
- Convert Rust `struct` to Pydantic `BaseModel` instances.
- Efficient and fast conversions with minimal overhead.
- Easy to use API.

## Installation

To use this library, you'll need to have Rust and Python installed on your system. Additionally, you'll need to install the necessary Rust crates and Python packages.

### Rust

Add the following to your `Cargo.toml`:

```toml
[dependencies]
pyo3 = { version = "0.15", features = ["extension-module"] }
serde = "1.0"
serde_json = "1.0"
```

### Python

Install the necessary Python packages using pip:

```sh
pip install pyo3 pydantic
```

## Usage

### Example

Here's a simple example demonstrating how to use this library:

#### Rust

```rust
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};

// Define a Rust struct
#[derive(Serialize, Deserialize, Debug)]
struct MyStruct {
    id: i32,
    name: String,
    active: bool,
}

// Convert Pydantic BaseModel to Rust Struct
#[pyfunction]
fn pydantic_to_rust(py_obj: &PyAny) -> PyResult<String> {
    let json_str = py_obj.call_method0("json")?.extract::<String>()?;
    let my_struct: MyStruct = from_str(&json_str)?;
    Ok(format!("{:?}", my_struct))
}

// Convert Rust Struct to Pydantic BaseModel
#[pyfunction]
fn rust_to_pydantic(py: Python, id: i32, name: String, active: bool) -> PyResult<PyObject> {
    let my_struct = MyStruct { id, name, active };
    let json_str = to_string(&my_struct)?;
    let pydantic_mod = py.import("pydantic")?;
    let base_model_class = pydantic_mod.get("BaseModel")?;
    let py_obj = base_model_class.call1((json_str,))?;
    Ok(py_obj.to_object(py))
}

// Create a Python module
#[pymodule]
fn rust_pydantic_converter(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pydantic_to_rust, m)?)?;
    m.add_function(wrap_pyfunction!(rust_to_pydantic, m)?)?;
    Ok(())
}
```

#### Python

```python
from pydantic import BaseModel
import rust_pydantic_converter

class MyModel(BaseModel):
    id: int
    name: str
    active: bool

# Convert Pydantic BaseModel to Rust Struct
model = MyModel(id=1, name="Example", active=True)
rust_struct = rust_pydantic_converter.pydantic_to_rust(model)
print(rust_struct)

# Convert Rust Struct to Pydantic BaseModel
model_from_rust = rust_pydantic_converter.rust_to_pydantic(2, "Test", False)
print(model_from_rust)
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or improvements.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgements

Special thanks to the contributors of `pyo3`, `serde`, and `pydantic` for their excellent libraries and tools.

---

This README provides an overview of the project, including installation instructions, usage examples, and contribution guidelines. Adjust the content as necessary to fit your specific project details and requirements.
### README

# pyo3-pydantic

A library for efficiently converting between Pydantic's `BaseModel` in Python and Rust `struct`. This library aims to
simplify the interoperability between Rust and Python, making it easy to use Rust's performance and safety with Python's
ease of use and rich ecosystem.

## Features

- Convert Pydantic `BaseModel` instances to Rust `struct`(with serde::Deserialize).
- Convert Rust `struct`(with serde::Serialize) to Pydantic `BaseModel` instances.
- Easy to use API.

## Goals and non-goals

### Goals

- Fast conversion in safe Rust code.
- Easy to use API.
- The many use cases for conversion in Pydantic v2

### Non-goals

- Support for Pydantic v1
- Support for older versions of Python
- Fast conversion in unsafe Rust code (If there are very good results, we will consider providing it as a separate
  crate)

## How to use

### Rust

Add the following to your `Cargo.toml`:

```toml
[dependencies]
pyo3-pydantic = { git = "https://github.com/higumachan/pyo3-pydantic.git" }
```

## Usage

### Code example

```rust
#[derive(Deserialize)]
struct MyStruct {
    name: String,
    age: i32,
}

#[pyfunction]
fn some_function(py: Python, pydantic_model: PyObject) -> PyResult<i32> {
    let model: MyStruct = from_pydantic_model(py, pydantic_model, vec![])?;

    return Ok(model.age * 2);
}

/// A Python module implemented in Rust.
#[pymodule]
fn simple_example(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(some_function, m)?)?;
    Ok(())
}

```

```python
import simple_example
from pydantic import BaseModel


class MyModel(BaseModel):
    name: str
    age: int


if __name__ == '__main__':
    model = MyModel(name="higumachan", age=30)
    result = simple_example.some_function(model)
    print(result)  # == 60
```

### Example

see [examples/example-project](examples/simple-example)

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or
improvements.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Acknowledgements

Special thanks to the contributors of `pyo3`, `serde`, and `pydantic` for their excellent libraries and tools.

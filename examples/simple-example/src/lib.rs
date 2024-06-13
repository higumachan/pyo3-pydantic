use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use pyo3_pydantic::from_pydantic_model;

#[derive(Serialize, Deserialize, Debug)]
struct Pet {
    name: String,
    age: u64,
    weight: f64,
    is_vaccinated: bool,
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn pet_age_2times(py: Python, pet: PyObject) -> PyResult<PyObject> {
    let pet = from_pydantic_model::<Pet>(py, pet, vec![])?;
    let age = pet.age * 2;

    let pet = Pet {
        name: pet.name,
        age,
        weight: pet.weight,
        is_vaccinated: pet.is_vaccinated,
    };

    Ok(pyo3_pydantic::to_pydantic_model(
        py,
        pet,
        "simple_example_py",
        "Pet",
    )?)
}

/// A Python module implemented in Rust.
#[pymodule]
fn simple_example(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pet_age_2times, m)?)?;
    Ok(())
}

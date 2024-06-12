use pyo3::{IntoPy, PyObject, Python};
use pyo3::types::IntoPyDict;
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Pyo3 error: {0}")]
    Pyo3Error(#[from] pyo3::PyErr),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub fn from_pydantic_model<T: DeserializeOwned>(
    py: Python,
    pydantic_model: PyObject,
    exclude: Vec<&str>,
) -> Result<T, Error> {
    let exclude_py = exclude.into_py(py);
    let locals = [("target_model", pydantic_model), ("exclude", exclude_py)].into_py_dict_bound(py);
    let model = py
        .eval_bound(
            "target_model.model_dump_json(exclude=exclude)",
            None,
            Some(&locals),
        )?
        .to_string();
    Ok(serde_json::from_str(&model)?)
}

#[cfg(test)]
mod tests {
    use pyo3::ToPyObject;
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize, Debug)]
    struct Pet {
        name: String,
        age: u64,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct PetExcludeName {
        age: u64,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct Plant {
        kind: String,
    }

    fn setup(py: Python) -> PyObject {
        let locals =
            [("model", py.import_bound("pyo3_pydantic.model").unwrap())].into_py_dict_bound(py);
        py.eval_bound(
            "model.Pet.model_validate({'name': 'Garfield', 'age': 42})",
            None,
            Some(&locals),
        )
        .unwrap()
        .to_object(py)
    }

    #[test]
    fn test_from_pydantic_model() {
        Python::with_gil(|py| {
            let pet = setup(py);
            let pet: Pet = from_pydantic_model(py, pet, vec![]).unwrap();
            assert_eq!(pet.name, "Garfield");
            assert_eq!(pet.age, 42);
        });
    }

    #[test]
    fn test_from_pydantic_model_exclude() {
        Python::with_gil(|py| {
            let pet = setup(py);
            let pet: PetExcludeName = from_pydantic_model(py, pet, vec!["name"]).unwrap();
            assert_eq!(pet.age, 42);
        });
    }

    #[test]
    fn test_from_pydantic_model_serde_error() {
        Python::with_gil(|py| {
            let pet = setup(py);
            let plant: Result<Plant, Error> = from_pydantic_model(py, pet, vec![]);
            assert!(matches!(plant, Err(Error::SerdeError(_))));
        });
    }
}

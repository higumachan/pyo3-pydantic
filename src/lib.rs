use pyo3::{IntoPy, PyErr, PyObject, Python, ToPyObject};
use pyo3::types::IntoPyDict;
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Pyo3 error: {0}")]
    Pyo3Error(#[from] pyo3::PyErr),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

impl From<Error> for PyErr {
    fn from(err: Error) -> PyErr {
        match err {
            Error::Pyo3Error(err) => err,
            Error::SerdeError(err) => {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(err.to_string())
            }
        }
    }
}

/// Convert a Pydantic model to a Rust struct
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

/// Convert a Rust struct to a Pydantic model
pub fn to_pydantic_model<T: Serialize>(
    py: Python,
    model: T,
    python_package_path: &str,
    model_name: &str,
) -> Result<PyObject, Error> {
    let model_json = serde_json::to_string(&model)?;
    let locals = [
        (
            "model_package",
            py.import_bound(python_package_path)?.into_py(py),
        ),
        ("model_json", model_json.to_object(py)),
    ]
    .into_py_dict_bound(py);
    Ok(py
        .eval_bound(
            &format!(
                "model_package.{}.model_validate_json(model_json)",
                model_name
            ),
            None,
            Some(&locals),
        )?
        .to_object(py))
}

#[cfg(test)]
mod tests {
    use pyo3::prelude::{PyAnyMethods, PyTypeMethods};
    use pyo3::ToPyObject;
    use serde::Deserialize;

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct Pet {
        name: String,
        age: u64,
    }

    #[derive(Deserialize, Debug)]
    #[allow(dead_code)]
    struct PetExcludeName {
        age: u64,
    }

    #[derive(Serialize, Deserialize, Debug)]
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

    #[test]
    fn test_to_pydantic_model() {
        Python::with_gil(|py| {
            let pet = Pet {
                name: "Garfield".to_string(),
                age: 42,
            };
            let pydantic_pet = to_pydantic_model(py, pet, "pyo3_pydantic.model", "Pet").unwrap();
            let locals = [("pet", pydantic_pet)].into_py_dict_bound(py);
            let pet_name = py
                .eval_bound("pet.name", None, Some(&locals))
                .unwrap()
                .to_string();
            assert_eq!(pet_name, "Garfield");
            let pet_age = py
                .eval_bound("pet.age", None, Some(&locals))
                .unwrap()
                .extract::<u64>()
                .unwrap();
            assert_eq!(pet_age, 42);
        });
    }

    #[test]
    fn test_to_pydantic_model_validation_error() {
        Python::with_gil(|py| {
            let plant = Plant {
                kind: "Tree".to_string(),
            };
            let pydantic_pet: Result<PyObject, Error> =
                to_pydantic_model(py, plant, "pyo3_pydantic.model", "Pet");
            dbg!(&pydantic_pet);
            assert!(matches!(pydantic_pet, Err(Error::Pyo3Error(_))));
            match pydantic_pet {
                Err(Error::Pyo3Error(err)) => {
                    let exception_value = err.get_type_bound(py).name().unwrap().to_string();
                    assert_eq!(
                        exception_value,
                        "pydantic_core._pydantic_core.ValidationError"
                    );
                }
                _ => panic!("Expected Pyo3Error"),
            }
        });
    }

    #[test]
    fn test_to_pydantic_model_not_found_package_error() {
        Python::with_gil(|py| {
            let pet = Pet {
                name: "Garfield".to_string(),
                age: 42,
            };

            let pydantic_pet: Result<PyObject, Error> =
                to_pydantic_model(py, pet, "pyo3_pydantic.not_found_model", "Pet");
            dbg!(&pydantic_pet);
            assert!(matches!(pydantic_pet, Err(Error::Pyo3Error(_))));
            match pydantic_pet {
                Err(Error::Pyo3Error(err)) => {
                    let exception_value = err.get_type_bound(py).name().unwrap().to_string();
                    assert_eq!(exception_value, "ModuleNotFoundError");
                }
                _ => panic!("Expected Pyo3Error"),
            }
        });
    }
}

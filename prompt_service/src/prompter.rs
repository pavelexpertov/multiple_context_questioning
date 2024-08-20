use pyo3::prelude::*;
use pyo3::types::{PyDict, PyFunction, PyString};

pub struct Prompter {
    pipeline_function: Py<PyFunction>
}

impl Prompter {
    pub fn initialise() -> Prompter {
        pyo3::prepare_freethreaded_python();
        let pipeline_function = Python::with_gil(|py| -> PyResult<Py<PyFunction>> {
            let _ = py.import_bound("time").unwrap();
            let _ = py.import_bound("requests").unwrap();
            let locals = PyDict::new_bound(py);
            py.run_bound(
                r#"
def give_me_random():
    time.sleep(5)
    return "There you go, random!"
                "#,
                None,
                Some(&locals),
            ).unwrap();
            let pipeline_function: Py<PyFunction> = locals.get_item("give_me_random").unwrap().unwrap().extract().unwrap();

            Ok(pipeline_function)
        }).unwrap();
        Prompter { pipeline_function }
    }

    pub fn prompt(&self) -> Py<PyString> {
        let pipeline_function = &self.pipeline_function;
        let response = Python::with_gil(|py| -> PyResult<Py<PyString>> {
            let function = pipeline_function.bind(py);
            let response: Bound<'_, PyAny> = function.call0().unwrap();
            let response: Py<PyString> = response.extract().unwrap();
            Ok(response)
        }).unwrap();

        response
    }
}

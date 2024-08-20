use pyo3::prelude::*;
use pyo3::types::{PyDict, PyFunction, PyString, PyList};

pub struct Prompter {
    pipeline_function: PyObject
}

impl Prompter {
    pub fn initialise() -> Prompter {
        pyo3::prepare_freethreaded_python();
        let pipeline_function = Python::with_gil(|py| -> PyResult<PyObject> {
            let transfomers = py.import_bound("transformers").unwrap();
            let pipeline = transfomers.getattr("pipeline").unwrap();
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("model", "openai-community/gpt2").unwrap();
            //kwargs.set_item("model", "google/gemma-2-2b-it").unwrap();
            kwargs.set_item("device", "cuda").unwrap();
            let prompt = pipeline.call(("text-generation",), Some(&kwargs))
                .unwrap();
            let pipeline_function = prompt.unbind();


            Ok(pipeline_function)
        }).unwrap();
        Prompter { pipeline_function }
    }

    pub fn prompt(&self, user_prompt: &String) -> Py<PyString> {
        let pipeline_function = &self.pipeline_function;
        let response = Python::with_gil(|py| -> PyResult<Py<PyString>> {
            let function = pipeline_function.bind(py);
            let kwargs = PyDict::new_bound(py);
            //kwargs.set_item("max_new_tokens", 300);
            kwargs.set_item("max_length", 300);
            //kwargs.set_item("pad_token_id", 50256);
            let list: Bound<'_, PyList> = function.call((user_prompt,), Some(&kwargs)).unwrap().extract().unwrap();
            let dict: Bound<'_, PyDict> = list.get_item(0).unwrap().extract().unwrap();
            let response: Py<PyString> = dict.get_item("generated_text").unwrap().unwrap().extract().unwrap();

            Ok(response)
        }).unwrap();

        response
    }
}

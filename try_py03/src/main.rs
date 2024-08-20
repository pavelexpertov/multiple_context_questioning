use pyo3::prelude::*;
use pyo3::types::PyList;

fn main() {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| -> PyResult<()> {
        let module = py.import_bound("requests").unwrap();
        let get_func = module.dict().get_item("get");
        if let Ok(Some(get_func)) = get_func {
            let response = get_func.call1(("https://example.com",)).unwrap();
            let result = response.getattr("text").unwrap();
            let result: String = result.extract().unwrap();
            println!("{}", result);
        } else {
            panic!("wtf");
        }
        Ok(())
    }).unwrap();
}

fn example2() {
    pyo3::prepare_freethreaded_python();
    let result = get_result();
    Python::with_gil(|py| -> PyResult<()> {
        let result = result.into_bound(py);
        let result: i32 = result.extract().unwrap();
        println!("result is {}", result);
        Ok(())
    }).unwrap();
}

fn get_result() -> PyObject {
    let result = Python::with_gil(|py| -> PyResult<PyObject> {
        let result = py.eval_bound("2 + 3", None, None).unwrap();
        Ok(result.unbind())
    }).unwrap();
    result
}

fn example1() {
    Python::with_gil(|py| -> PyResult<()> {
        let x = PyList::empty_bound(py);
        x.append(3).unwrap();
        x.append("hi").unwrap();
        println!("{}", format!("{:?}", x));
        Ok(())
    });
}

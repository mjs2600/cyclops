use numpy::{IntoPyArray, PyArray2};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pyclops(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn descriptors(py: Python, image_filepath: &str) -> PyResult<Py<PyArray2<u8>>> {
        let descriptors = cyclops::get_descriptors(image_filepath).unwrap();
        Ok(descriptors.into_pyarray(py).to_owned())
    }

    Ok(())
}

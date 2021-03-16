use cyclops;
use numpy::{PyArray2, ToPyArray};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pyclops(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "sift_descriptors")]
    fn sift_descriptors(py: Python, image_filepath: &str) -> PyResult<Py<PyArray2<i32>>> {
        let image = cyclops::load_img(image_filepath).unwrap();
        let descriptors = cyclops::get_descriptors(&image);
        Ok(descriptors.to_pyarray(py).to_owned())
    }

    Ok(())
}

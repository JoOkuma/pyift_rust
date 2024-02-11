use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod adjacency;
mod bucket_queue;
mod heap;
mod priority_queue;
mod watershed;
pub use watershed::*;

#[pymodule]
fn pyift_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(watershed_from_minima_u8_2d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_u8_3d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_u16_2d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_u16_3d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_u32_2d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_u32_3d, m)?)?;

    m.add_function(wrap_pyfunction!(watershed_from_minima_i16_2d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_i16_3d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_i32_2d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_i32_3d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_i64_2d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_i64_3d, m)?)?;

    m.add_function(wrap_pyfunction!(watershed_from_minima_f32_2d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_f32_3d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_f64_2d, m)?)?;
    m.add_function(wrap_pyfunction!(watershed_from_minima_f64_3d, m)?)?;

    Ok(())
}

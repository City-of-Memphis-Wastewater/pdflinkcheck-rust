// lib.rs
pub use crate::analysis_pdfium::analyze_pdf;

// --- PyO3 Python Bindings ---
#[pyfunction]
#[pyo3(name = "analyze_pdf")] // This names the function in Python
fn analyze_pdf_py(path: String) -> PyResult<String> {
    let result = analyze_pdf(&path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;
    
    let json = serde_json::to_string(&result)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    Ok(json)
}

#[pymodule]
fn pdflinkcheck_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_pdf_py, m)?)?;
    Ok(())
}

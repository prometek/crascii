
#[cfg(feature = "python")]
mod python_bindings {
    use pyo3::prelude::*;
    use crate::{ASCIIImage, Options};
    use std::borrow::Cow;

    // Owned version of Options for Python
    #[pyclass]
    #[derive(Clone)]
    pub struct PyOptions {
        #[pyo3(get, set)]
        pub columns: Option<u32>,
        #[pyo3(get, set)]
        pub lines: Option<u32>,
        #[pyo3(get, set)]
        pub color: bool,
        #[pyo3(get, set)]
        pub charsets: String,
        #[pyo3(get, set)]
        pub output_path: String,
    }

    
    #[pymethods]
    impl PyOptions {
        #[new]
        pub fn new(
            columns: Option<u32>,
            lines: Option<u32>,
            color: bool,
            charsets: String,
            output_path: String,
        ) -> Self {
            PyOptions {
                columns,
                lines,
                color,
                charsets,
                output_path,
            }
        }
    }

    // Implement a method to convert PyOptions to Options<'static>
    impl PyOptions {
        fn to_options(&self) -> Options<'static> {

            Options {
                columns: self.columns,
                lines: self.lines,
                color: self.color,
                charsets: Cow::Owned(self.charsets.clone()),
                output_path: Cow::Owned(self.output_path.clone()),

            }
        }
    }

    // Owned version of ASCIIImage for Python
    #[pyclass]
    pub struct PyASCIIImage {
        ascii_image: ASCIIImage<'static>,
        _image_path: String,
       }

    // Implement methods for PyASCIIImage
    #[pymethods]
    impl PyASCIIImage {
        #[new]
        pub fn new(image_path: String, options: PyOptions) -> Self {
           
            let ascii_image = ASCIIImage::new(image_path.clone(), options.to_options());

            PyASCIIImage {
                ascii_image,
                _image_path: image_path,
            }
        }

        pub fn convert(&mut self) -> PyResult<()> {
            self.ascii_image.convert().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e))
            })
        }
    }

    #[pymodule]
    fn crascii(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<PyOptions>()?;
        m.add_class::<PyASCIIImage>()?;
        Ok(())
    }
}


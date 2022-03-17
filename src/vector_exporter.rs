use std::{fs::File, io::Write};

#[derive(Debug, Clone)]
pub enum ExportType {
    Text,
    Csv,
    Json,
}

pub enum ExportError {
    IoError(std::io::Error),
    JsonError(serde_json::Error),
}

impl Default for ExportType {
    fn default() -> Self {
        ExportType::Text
    }
}

pub trait IVectorExporter<T> {
    fn new(vec: Vec<T>, export_type: ExportType, export_type: &str) -> Self;

    fn export(&self) -> Result<(), ExportError>;
}

#[derive(Debug, Clone)]
pub struct VectorExporter<T> {
    pub vec: Vec<T>,
    pub export_type: ExportType,
    pub export_path: String,
}

impl IVectorExporter<String> for VectorExporter<String> {
    /// Initialize a new `VectorExporter` for type `String`
    ///
    /// # Example
    ///
    /// ```rust
    /// let vec_exporter: VectorExporter<String> = IVectorExporter::<String>::new(vec![], ExportType::default(), "/path/to/file");
    /// ```
    ///
    /// # Returns
    ///
    /// The `VectorExporter` struct for type `String`
    fn new(vec: Vec<String>, export_type: ExportType, export_path: &str) -> VectorExporter<String> {
        VectorExporter {
            vec,
            export_type,
            export_path: String::from(export_path),
        }
    }

    /// Export the `Vec` of type `String` to a file
    ///
    /// # Example
    ///
    /// ```rust
    /// let res = vec_exporter.export();
    /// ```
    ///
    /// # Returns
    ///
    /// A `Result` that can either contain an `Ok` or an `Error` struct
    fn export(&self) -> Result<(), ExportError> {
        let file = File::create(&self.export_path);
        let mut file = match file {
            Ok(file) => file,
            Err(e) => return Err(ExportError::IoError(e)),
        };
        match self.export_type {
            ExportType::Text => {
                let mut data = String::new();
                for l in &self.vec {
                    data.push_str(&format!("{}\n", l));
                }

                match write!(file, "{}", data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ExportError::IoError(e)),
                }
            }
            ExportType::Csv => {
                let mut data = String::new();
                for l in &self.vec {
                    data.push_str(&format!("\"{}\"\n", l));
                }

                match write!(file, "{}", data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ExportError::IoError(e)),
                }
            }
            ExportType::Json => {
                let serialized = match serde_json::to_string(&self.vec) {
                    Ok(d) => d,
                    Err(e) => return Err(ExportError::JsonError(e)),
                };

                match write!(file, "{}", serialized) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(ExportError::IoError(e)),
                }
            }
        }
    }
}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfFile {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub amount: Option<f64>,
    pub suggested_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceInfo {
    pub amount: Option<f64>,
    pub original_filename: String,
    pub suggested_filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenamePreview {
    pub original_name: String,
    pub suggested_name: String,
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameOperation {
    pub old_path: String,
    pub new_path: String,
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RenameResult {
    pub success: bool,
    pub message: String,
    pub processed_files: usize,
    pub failed_files: Vec<String>,
} 
use std::path::Path;
use crate::types::{PdfFile, RenamePreview};
use crate::file_service::FileService;

pub struct NamingEngine {
    #[allow(dead_code)]
    file_service: FileService,
}

impl NamingEngine {
    pub fn new() -> Self {
        NamingEngine {
            file_service: FileService::new(),
        }
    }

    /// 生成基于金额的文件名
    /// 格式：{金额}元_发票.pdf
    pub fn generate_filename(&self, amount: f64) -> String {
        format!("{:.2}元_发票.pdf", amount)
    }

    /// 格式化金额显示
    #[allow(dead_code)]
    pub fn format_amount(&self, amount: f64) -> String {
        format!("{:.2}", amount)
    }

    /// 为PDF文件生成建议的文件名
    pub fn generate_suggested_name(&self, pdf_file: &PdfFile) -> String {
        match pdf_file.amount {
            Some(amount) => self.generate_filename(amount),
            None => "未知金额_发票.pdf".to_string(),
        }
    }

    /// 批量生成重命名预览
    #[allow(dead_code)]
    pub fn generate_rename_previews(&self, files: &[PdfFile]) -> Vec<RenamePreview> {
        files.iter().map(|file| {
            let suggested_name = self.generate_suggested_name(file);
            
            RenamePreview {
                original_name: file.name.clone(),
                suggested_name,
                amount: file.amount,
            }
        }).collect()
    }

    /// 解决文件名冲突
    #[allow(dead_code)]
    pub fn resolve_naming_conflicts(&self, files: &[PdfFile]) -> Vec<String> {
        let mut name_counts = std::collections::HashMap::new();
        let mut resolved_names = Vec::new();

        for file in files {
            let base_name = self.generate_suggested_name(file);
            
            // 检查是否有冲突
            let count = name_counts.entry(base_name.clone()).or_insert(0);
            *count += 1;

            let final_name = if *count == 1 {
                base_name
            } else {
                // 处理冲突：添加序号
                let (stem, ext) = self.split_filename(&base_name);
                format!("{}_{}.{}", stem, count, ext)
            };

            resolved_names.push(final_name);
        }

        resolved_names
    }

    /// 分离文件名和扩展名
    #[allow(dead_code)]
    fn split_filename(&self, filename: &str) -> (String, String) {
        let path = Path::new(filename);
        let stem = path.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "file".to_string());
        let ext = path.extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_else(|| "pdf".to_string());
        
        (stem, ext)
    }

    /// 清理和验证文件名
    #[allow(dead_code)]
    pub fn sanitize_filename(&self, filename: &str) -> String {
        self.file_service.sanitize_filename(filename)
    }

    /// 生成完整的重命名路径
    #[allow(dead_code)]
    pub fn generate_full_rename_path(&self, original_path: &str, new_filename: &str) -> Option<String> {
        let original_path = Path::new(original_path);
        let directory = original_path.parent()?;
        let new_path = directory.join(new_filename);
        
        Some(new_path.to_string_lossy().to_string())
    }

    /// 验证生成的文件名是否有效
    #[allow(dead_code)]
    pub fn validate_generated_name(&self, filename: &str) -> bool {
        // 检查是否符合预期格式
        if !filename.ends_with("_发票.pdf") {
            return false;
        }

        // 验证金额部分
        if let Some(amount_part) = filename.strip_suffix("_发票.pdf") {
            if amount_part.ends_with("元") {
                let amount_str = amount_part.strip_suffix("元").unwrap_or("");
                if amount_str.parse::<f64>().is_ok() {
                    return self.file_service.validate_filename(filename);
                }
            }
        }

        // 检查是否是未知金额格式
        if filename == "未知金额_发票.pdf" {
            return self.file_service.validate_filename(filename);
        }

        false
    }

    /// 批量验证文件名
    #[allow(dead_code)]
    pub fn validate_batch_names(&self, filenames: &[String]) -> Vec<bool> {
        filenames.iter()
            .map(|name| self.validate_generated_name(name))
            .collect()
    }

    /// 检查目录中是否存在重名文件
    #[allow(dead_code)]
    pub fn check_naming_conflicts_in_directory(&self, directory: &str, new_filenames: &[String]) -> Vec<bool> {
        new_filenames.iter()
            .map(|filename| {
                let full_path = Path::new(directory).join(filename);
                full_path.exists()
            })
            .collect()
    }

    /// 为存在冲突的文件生成替代名称
    #[allow(dead_code)]
    pub fn resolve_directory_conflicts(&self, directory: &str, filenames: &[String]) -> Vec<String> {
        filenames.iter()
            .map(|filename| {
                self.file_service.resolve_filename_conflict(directory, filename)
            })
            .collect()
    }

    /// 生成重命名统计信息
    #[allow(dead_code)]
    pub fn generate_rename_stats(&self, files: &[PdfFile]) -> RenameStats {
        let total_files = files.len();
        let files_with_amount = files.iter().filter(|f| f.amount.is_some()).count();
        let files_without_amount = total_files - files_with_amount;
        
        let total_amount: f64 = files.iter()
            .filter_map(|f| f.amount)
            .sum();
        
        RenameStats {
            total_files,
            files_with_amount,
            files_without_amount,
            total_amount,
        }
    }
}

/// 重命名统计信息
#[allow(dead_code)]
#[derive(Debug)]
pub struct RenameStats {
    pub total_files: usize,
    pub files_with_amount: usize,
    pub files_without_amount: usize,
    pub total_amount: f64,
}

impl RenameStats {
    #[allow(dead_code)]
    pub fn average_amount(&self) -> f64 {
        if self.files_with_amount > 0 {
            self.total_amount / self.files_with_amount as f64
        } else {
            0.0
        }
    }
} 
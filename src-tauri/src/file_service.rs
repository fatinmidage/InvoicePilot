use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use chrono::{DateTime, Utc};
use crate::types::PdfFile;

pub struct FileService;

impl FileService {
    pub fn new() -> Self {
        FileService
    }

    /// 扫描指定目录中的PDF文件
    pub fn scan_directory(&self, directory_path: &str) -> Result<Vec<PdfFile>, String> {
        let path = Path::new(directory_path);
        
        if !path.exists() {
            return Err("目录不存在".to_string());
        }

        if !path.is_dir() {
            return Err("路径不是目录".to_string());
        }

        let mut pdf_files = Vec::new();
        
        for entry in WalkDir::new(path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension.to_string_lossy().to_lowercase() == "pdf" {
                        match self.create_pdf_file_info(path) {
                            Ok(pdf_file) => pdf_files.push(pdf_file),
                            Err(e) => {
                                eprintln!("处理文件时出错 {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }

        Ok(pdf_files)
    }

    /// 创建PdfFile信息
    pub fn create_pdf_file_info(&self, path: &Path) -> Result<PdfFile, String> {
        let metadata = fs::metadata(path)
            .map_err(|e| format!("无法获取文件信息: {}", e))?;

        let file_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown.pdf")
            .to_string();

        let file_path = path.to_string_lossy().to_string();

        let size = metadata.len();
        
        let modified = metadata.modified()
            .map_err(|e| format!("无法获取修改时间: {}", e))?;
        
        let modified_dt: DateTime<Utc> = modified.into();

        // 生成唯一ID
        let id = format!("{}", path.display()).replace('/', "_").replace('\\', "_");

        Ok(PdfFile {
            id,
            name: file_name,
            path: file_path,
            size,
            modified: modified_dt,
            amount: None,
            suggested_name: None,
        })
    }

    /// 重命名文件
    pub fn rename_file(&self, old_path: &str, new_path: &str) -> Result<(), String> {
        let old_path = Path::new(old_path);
        let new_path = Path::new(new_path);

        if !old_path.exists() {
            return Err("源文件不存在".to_string());
        }

        if new_path.exists() {
            return Err("目标文件已存在".to_string());
        }

        // 确保目标目录存在
        if let Some(parent) = new_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("无法创建目录: {}", e))?;
            }
        }

        fs::rename(old_path, new_path)
            .map_err(|e| format!("重命名失败: {}", e))?;

        Ok(())
    }

    /// 验证文件名是否合法
    #[allow(dead_code)]
    pub fn validate_filename(&self, filename: &str) -> bool {
        // 检查文件名是否包含非法字符
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*', '\\', '/'];
        
        for ch in filename.chars() {
            if invalid_chars.contains(&ch) {
                return false;
            }
        }

        // 检查文件名长度
        if filename.len() > 255 {
            return false;
        }

        // 检查是否为空
        if filename.trim().is_empty() {
            return false;
        }

        true
    }

    /// 生成安全的文件名
    #[allow(dead_code)]
    pub fn sanitize_filename(&self, filename: &str) -> String {
        let mut sanitized = filename.to_string();
        
        // 替换非法字符
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*', '\\', '/'];
        for ch in invalid_chars {
            sanitized = sanitized.replace(ch, "_");
        }

        // 限制长度
        if sanitized.len() > 255 {
            sanitized.truncate(252);
            sanitized.push_str("...");
        }

        sanitized
    }

    /// 解决文件名冲突
    pub fn resolve_filename_conflict(&self, directory: &str, filename: &str) -> String {
        let path = Path::new(directory);
        let mut counter = 1;
        let mut new_filename = filename.to_string();

        while path.join(&new_filename).exists() {
            if let Some(stem) = Path::new(filename).file_stem() {
                if let Some(extension) = Path::new(filename).extension() {
                    new_filename = format!("{}_{}.{}", 
                        stem.to_string_lossy(), 
                        counter, 
                        extension.to_string_lossy()
                    );
                } else {
                    new_filename = format!("{}_{}", filename, counter);
                }
            } else {
                new_filename = format!("{}_{}", filename, counter);
            }
            counter += 1;
        }

        new_filename
    }

    /// 获取文件的目录路径
    #[allow(dead_code)]
    pub fn get_directory_path(&self, file_path: &str) -> Option<String> {
        Path::new(file_path).parent()
            .map(|p| p.to_string_lossy().to_string())
    }

    /// 检查目录是否可写
    pub fn is_directory_writable(&self, directory_path: &str) -> bool {
        let path = Path::new(directory_path);
        
        if !path.exists() || !path.is_dir() {
            return false;
        }

        // 尝试创建一个临时文件来测试写权限
        let temp_file = path.join(".temp_write_test");
        match fs::File::create(&temp_file) {
            Ok(_) => {
                // 清理临时文件
                let _ = fs::remove_file(&temp_file);
                true
            }
            Err(_) => false,
        }
    }
} 
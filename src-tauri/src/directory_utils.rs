use std::path::Path;
use std::env;

/// 目录工具结构体
pub struct DirectoryUtils;

impl DirectoryUtils {
    /// 创建新的目录工具实例
    pub fn new() -> Self {
        DirectoryUtils
    }

    /// 获取当前工作目录
    /// 
    /// 优先级顺序：
    /// 1. 可执行文件所在目录（处理 macOS 应用包）
    /// 2. 当前工作目录（如果不是根目录且不是编译目录）
    /// 3. 用户文档目录
    /// 4. 用户主目录
    pub fn get_current_directory(&self) -> Result<String, String> {
        // 首先尝试获取可执行文件所在的目录（处理 macOS 应用包）
        if let Some(exe_dir) = self.get_executable_directory() {
            return Ok(exe_dir);
        }

        // 尝试获取当前工作目录
        if let Some(current_dir) = self.get_working_directory() {
            // 避免使用编译目录
            if !current_dir.contains("target/debug") && !current_dir.contains("target/release") {
                return Ok(current_dir);
            }
        }

        // 尝试获取用户文档目录
        if let Some(documents_dir) = self.get_documents_directory() {
            return Ok(documents_dir);
        }

        // 如果都失败了，返回用户主目录
        self.get_home_directory()
    }

    /// 获取可执行文件所在的目录
    /// 特别处理 macOS 应用包的情况
    fn get_executable_directory(&self) -> Option<String> {
        if let Ok(exe_path) = env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                if let Some(exe_dir_str) = exe_dir.to_str() {
                    // 在 macOS 应用包中，可执行文件通常在 .app/Contents/MacOS/ 目录下
                    // 我们需要找到一个更合适的目录
                    let exe_dir_path = Path::new(exe_dir_str);
                    
                    // 如果在 .app 包中，尝试找到应用包的父目录
                    if exe_dir_str.contains(".app/Contents/MacOS") {
                        return self.find_app_parent_directory(exe_dir_path);
                    }
                    
                    // 如果在开发模式下（target/debug 或 target/release），返回项目根目录
                    if exe_dir_str.contains("target/debug") || exe_dir_str.contains("target/release") {
                        // 从 target/debug 或 target/release 向上找到项目根目录
                        if let Some(project_root) = self.find_project_root(exe_dir_path) {
                            return Some(project_root);
                        }
                    }
                    
                    // 如果不在 .app 包中且不在开发模式，直接返回可执行文件所在目录
                    return Some(exe_dir_str.to_string());
                }
            }
        }
        None
    }

    /// 在 macOS 应用包中查找应用包的父目录
    fn find_app_parent_directory(&self, exe_dir_path: &Path) -> Option<String> {
        let mut current_path = exe_dir_path;
        while let Some(parent) = current_path.parent() {
            if let Some(parent_str) = parent.to_str() {
                if parent_str.ends_with(".app") {
                    // 找到了 .app 目录，返回其父目录
                    if let Some(app_parent) = parent.parent() {
                        if let Some(app_parent_str) = app_parent.to_str() {
                            return Some(app_parent_str.to_string());
                        }
                    }
                    break;
                }
            }
            current_path = parent;
        }
        None
    }

    /// 从给定路径向上查找项目根目录
    fn find_project_root(&self, start_path: &Path) -> Option<String> {
        let mut current_path = start_path;
        while let Some(parent) = current_path.parent() {
            // 优先查找 package.json 文件（Node.js/前端项目的标识）
            if parent.join("package.json").exists() {
                if let Some(project_root_str) = parent.to_str() {
                    return Some(project_root_str.to_string());
                }
            }
            // 检查是否存在 Cargo.toml 文件（Rust 项目的标识）
            // 但要确保不是 src-tauri 目录
            if parent.join("Cargo.toml").exists() {
                if let Some(parent_name) = parent.file_name() {
                    if parent_name != "src-tauri" {
                        if let Some(project_root_str) = parent.to_str() {
                            return Some(project_root_str.to_string());
                        }
                    }
                }
            }
            current_path = parent;
        }
        None
    }

    /// 获取当前工作目录
    fn get_working_directory(&self) -> Option<String> {
        if let Ok(current_dir) = env::current_dir() {
            if let Some(current_dir_str) = current_dir.to_str() {
                // 跳过根目录
                if current_dir_str != "/" {
                    return Some(current_dir_str.to_string());
                }
            }
        }
        None
    }

    /// 获取用户主目录
    fn get_home_directory(&self) -> Result<String, String> {
        env::var("HOME")
            .map_err(|_| "无法获取用户主目录".to_string())
            .or_else(|_| {
                // 在 Windows 上尝试 USERPROFILE
                env::var("USERPROFILE")
                    .map_err(|_| "无法获取用户主目录".to_string())
            })
            .or_else(|_| {
                // 最后的回退选项
                Ok("/Users".to_string())
            })
    }

    /// 获取用户文档目录
    fn get_documents_directory(&self) -> Option<String> {
        if let Ok(home) = env::var("HOME") {
            let documents_path = format!("{}/Documents", home);
            if Path::new(&documents_path).exists() {
                return Some(documents_path);
            }
        }
        
        // Windows 用户文档目录
        if let Ok(userprofile) = env::var("USERPROFILE") {
            let documents_path = format!("{}\\Documents", userprofile);
            if Path::new(&documents_path).exists() {
                return Some(documents_path);
            }
        }
        
        None
    }

    /// 验证目录是否存在且可访问
    pub fn is_directory_valid(&self, directory_path: &str) -> bool {
        let path = Path::new(directory_path);
        path.exists() && path.is_dir()
    }

    /// 验证目录是否可写
    pub fn is_directory_writable(&self, directory_path: &str) -> bool {
        let path = Path::new(directory_path);
        
        if !path.exists() || !path.is_dir() {
            return false;
        }

        // 尝试创建一个临时文件来测试写权限
        let temp_file = path.join(".temp_write_test");
        match std::fs::File::create(&temp_file) {
            Ok(_) => {
                // 清理临时文件
                let _ = std::fs::remove_file(&temp_file);
                true
            }
            Err(_) => false,
        }
    }

    /// 获取目录的父目录
    pub fn get_parent_directory(&self, directory_path: &str) -> Option<String> {
        Path::new(directory_path)
            .parent()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string())
    }

    /// 规范化目录路径
    pub fn normalize_path(&self, path: &str) -> String {
        // 替换反斜杠为正斜杠（Windows 兼容性）
        path.replace('\\', "/")
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_utils_creation() {
        let _dir_utils = DirectoryUtils::new();
        // 测试创建实例
        assert!(true);
    }

    #[test]
    fn test_get_current_directory() {
        let dir_utils = DirectoryUtils::new();
        match dir_utils.get_current_directory() {
            Ok(dir) => {
                assert!(!dir.is_empty());
                assert!(dir_utils.is_directory_valid(&dir));
            }
            Err(e) => {
                panic!("无法获取目录: {}", e);
            }
        }
    }

    #[test]
    fn test_path_normalization() {
        let dir_utils = DirectoryUtils::new();
        let windows_path = "C:\\Users\\Documents\\test";
        let normalized = dir_utils.normalize_path(windows_path);
        assert_eq!(normalized, "C:/Users/Documents/test");
    }

    #[test]
    fn test_documents_directory() {
        let dir_utils = DirectoryUtils::new();
        if let Some(docs_dir) = dir_utils.get_documents_directory() {
            assert!(dir_utils.is_directory_valid(&docs_dir));
        }
    }
} 
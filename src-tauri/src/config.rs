use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{AppError, AppResult};

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 默认扫描目录
    pub default_scan_directory: Option<String>,
    /// 金额识别配置
    pub amount_recognition: AmountRecognitionConfig,
    /// 文件过滤配置
    pub file_filter: FileFilterConfig,
    /// 重命名策略配置
    pub rename_strategy: RenameStrategyConfig,
    /// 界面配置
    pub ui_config: UiConfig,
}

/// 金额识别配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmountRecognitionConfig {
    /// 是否启用中文数字识别
    pub enable_chinese_digits: bool,
    /// 是否启用阿拉伯数字识别
    pub enable_arabic_digits: bool,
    /// 是否启用货币符号识别
    pub enable_currency_symbols: bool,
    /// 金额识别的最小值
    pub min_amount: f64,
    /// 金额识别的最大值
    pub max_amount: f64,
}

/// 文件过滤配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileFilterConfig {
    /// 支持的文件扩展名
    pub supported_extensions: Vec<String>,
    /// 最大文件大小（字节）
    pub max_file_size: u64,
    /// 是否包含子目录
    pub include_subdirectories: bool,
    /// 排除的文件名模式
    pub exclude_patterns: Vec<String>,
}

/// 重命名策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameStrategyConfig {
    /// 文件名格式模板
    pub filename_template: String,
    /// 是否在重命名前备份
    pub backup_before_rename: bool,
    /// 重名文件的处理方式
    pub conflict_resolution: ConflictResolution,
    /// 是否保留原文件修改时间
    pub preserve_modification_time: bool,
}

/// 重名文件处理方式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// 添加序号后缀
    AddSuffix,
    /// 跳过重名文件
    Skip,
    /// 覆盖已存在文件
    Overwrite,
    /// 询问用户
    Ask,
}

/// 界面配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// 主题
    pub theme: Theme,
    /// 语言
    pub language: Language,
    /// 是否显示详细日志
    pub show_detailed_logs: bool,
    /// 是否自动刷新文件列表
    pub auto_refresh: bool,
}

/// 主题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

/// 语言
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Chinese,
    English,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            default_scan_directory: None,
            amount_recognition: AmountRecognitionConfig::default(),
            file_filter: FileFilterConfig::default(),
            rename_strategy: RenameStrategyConfig::default(),
            ui_config: UiConfig::default(),
        }
    }
}

impl Default for AmountRecognitionConfig {
    fn default() -> Self {
        AmountRecognitionConfig {
            enable_chinese_digits: true,
            enable_arabic_digits: true,
            enable_currency_symbols: true,
            min_amount: 0.01,
            max_amount: 1000000.0,
        }
    }
}

impl Default for FileFilterConfig {
    fn default() -> Self {
        FileFilterConfig {
            supported_extensions: vec!["pdf".to_string()],
            max_file_size: 100 * 1024 * 1024, // 100MB
            include_subdirectories: false,
            exclude_patterns: vec![
                ".*".to_string(),           // 隐藏文件
                "~*".to_string(),           // 临时文件
                "*.tmp".to_string(),        // 临时文件
            ],
        }
    }
}

impl Default for RenameStrategyConfig {
    fn default() -> Self {
        RenameStrategyConfig {
            filename_template: "{金额}元_发票.pdf".to_string(),
            backup_before_rename: false,
            conflict_resolution: ConflictResolution::AddSuffix,
            preserve_modification_time: true,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        UiConfig {
            theme: Theme::System,
            language: Language::Chinese,
            show_detailed_logs: false,
            auto_refresh: true,
        }
    }
}

/// 配置管理器
pub struct ConfigManager {
    config: AppConfig,
    config_path: PathBuf,
}

impl ConfigManager {
    /// 创建新的配置管理器
    pub fn new() -> AppResult<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_config(&config_path)?;
        
        Ok(ConfigManager {
            config,
            config_path,
        })
    }

    /// 获取配置文件路径
    fn get_config_path() -> AppResult<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            let app_config_dir = config_dir.join("InvoicePilot");
            std::fs::create_dir_all(&app_config_dir)?;
            Ok(app_config_dir.join("config.json"))
        } else {
            Err(AppError::file_system_error("无法获取配置目录"))
        }
    }

    /// 加载配置
    fn load_config(config_path: &PathBuf) -> AppResult<AppConfig> {
        if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            let config: AppConfig = serde_json::from_str(&content)
                .map_err(|e| AppError::validation_error(&format!("配置文件格式错误: {}", e)))?;
            Ok(config)
        } else {
            // 如果配置文件不存在，使用默认配置
            let default_config = AppConfig::default();
            // 保存默认配置到文件
            let content = serde_json::to_string_pretty(&default_config)
                .map_err(|e| AppError::validation_error(&format!("序列化配置失败: {}", e)))?;
            std::fs::write(config_path, content)?;
            Ok(default_config)
        }
    }

    /// 保存配置
    pub fn save_config(&self) -> AppResult<()> {
        let content = serde_json::to_string_pretty(&self.config)
            .map_err(|e| AppError::validation_error(&format!("序列化配置失败: {}", e)))?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }

    /// 获取当前配置
    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    /// 更新配置
    pub fn update_config(&mut self, new_config: AppConfig) -> AppResult<()> {
        self.config = new_config;
        self.save_config()
    }

    /// 重置为默认配置
    pub fn reset_to_default(&mut self) -> AppResult<()> {
        self.config = AppConfig::default();
        self.save_config()
    }

    /// 验证配置
    #[allow(dead_code)]
    pub fn validate_config(&self) -> AppResult<()> {
        let config = &self.config;
        
        // 验证金额范围
        if config.amount_recognition.min_amount < 0.0 {
            return Err(AppError::validation_error("最小金额不能为负数"));
        }
        
        if config.amount_recognition.max_amount <= config.amount_recognition.min_amount {
            return Err(AppError::validation_error("最大金额必须大于最小金额"));
        }

        // 验证文件大小限制
        if config.file_filter.max_file_size == 0 {
            return Err(AppError::validation_error("最大文件大小必须大于0"));
        }

        // 验证支持的文件扩展名
        if config.file_filter.supported_extensions.is_empty() {
            return Err(AppError::validation_error("必须指定至少一个支持的文件扩展名"));
        }

        // 验证文件名模板
        if config.rename_strategy.filename_template.is_empty() {
            return Err(AppError::validation_error("文件名模板不能为空"));
        }

        Ok(())
    }

    /// 获取默认扫描目录
    #[allow(dead_code)]
    pub fn get_default_scan_directory(&self) -> Option<String> {
        self.config.default_scan_directory.clone()
    }

    /// 设置默认扫描目录
    #[allow(dead_code)]
    pub fn set_default_scan_directory(&mut self, directory: Option<String>) -> AppResult<()> {
        self.config.default_scan_directory = directory;
        self.save_config()
    }
}

/// 添加dirs依赖的辅助函数
/// 在实际应用中，需要在Cargo.toml中添加dirs依赖
mod dirs {
    use std::path::PathBuf;
    
    pub fn config_dir() -> Option<PathBuf> {
        #[cfg(target_os = "macos")]
        {
            home_dir().map(|h| h.join("Library").join("Application Support"))
        }
        #[cfg(target_os = "windows")]
        {
            std::env::var("APPDATA").ok().map(PathBuf::from)
        }
        #[cfg(target_os = "linux")]
        {
            std::env::var("XDG_CONFIG_HOME")
                .ok()
                .map(PathBuf::from)
                .or_else(|| home_dir().map(|h| h.join(".config")))
        }
    }

    fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME").ok().map(PathBuf::from)
    }
} 
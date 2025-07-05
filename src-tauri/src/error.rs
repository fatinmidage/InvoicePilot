use std::fmt;
use serde::{Deserialize, Serialize};

/// 应用程序错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    /// PDF解析错误
    PdfParseError(String),
    /// 金额提取错误
    AmountExtractionError(String),
    /// 文件系统错误
    FileSystemError(String),
    /// 命名错误
    NamingError(String),
    /// 权限错误
    PermissionError(String),
    /// IO错误
    IoError(String),
    /// 验证错误
    ValidationError(String),
    /// 未知错误
    UnknownError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::PdfParseError(msg) => write!(f, "PDF解析错误: {}", msg),
            AppError::AmountExtractionError(msg) => write!(f, "金额提取错误: {}", msg),
            AppError::FileSystemError(msg) => write!(f, "文件系统错误: {}", msg),
            AppError::NamingError(msg) => write!(f, "命名错误: {}", msg),
            AppError::PermissionError(msg) => write!(f, "权限错误: {}", msg),
            AppError::IoError(msg) => write!(f, "IO错误: {}", msg),
            AppError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            AppError::UnknownError(msg) => write!(f, "未知错误: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// 应用程序结果类型
pub type AppResult<T> = Result<T, AppError>;

/// 错误辅助函数
impl AppError {
    /// 创建PDF解析错误
    pub fn pdf_parse_error(msg: &str) -> Self {
        AppError::PdfParseError(msg.to_string())
    }

    /// 创建金额提取错误
    #[allow(dead_code)]
    pub fn amount_extraction_error(msg: &str) -> Self {
        AppError::AmountExtractionError(msg.to_string())
    }

    /// 创建文件系统错误
    pub fn file_system_error(msg: &str) -> Self {
        AppError::FileSystemError(msg.to_string())
    }

    /// 创建命名错误
    #[allow(dead_code)]
    pub fn naming_error(msg: &str) -> Self {
        AppError::NamingError(msg.to_string())
    }

    /// 创建权限错误
    pub fn permission_error(msg: &str) -> Self {
        AppError::PermissionError(msg.to_string())
    }

    /// 创建IO错误
    pub fn io_error(msg: &str) -> Self {
        AppError::IoError(msg.to_string())
    }

    /// 创建验证错误
    pub fn validation_error(msg: &str) -> Self {
        AppError::ValidationError(msg.to_string())
    }

    /// 创建未知错误
    pub fn unknown_error(msg: &str) -> Self {
        AppError::UnknownError(msg.to_string())
    }

    /// 获取错误代码
    #[allow(dead_code)]
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::PdfParseError(_) => "PDF_PARSE_ERROR",
            AppError::AmountExtractionError(_) => "AMOUNT_EXTRACTION_ERROR",
            AppError::FileSystemError(_) => "FILE_SYSTEM_ERROR",
            AppError::NamingError(_) => "NAMING_ERROR",
            AppError::PermissionError(_) => "PERMISSION_ERROR",
            AppError::IoError(_) => "IO_ERROR",
            AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::UnknownError(_) => "UNKNOWN_ERROR",
        }
    }

    /// 获取用户友好的错误消息
    #[allow(dead_code)]
    pub fn user_message(&self) -> String {
        match self {
            AppError::PdfParseError(_) => "无法解析PDF文件，请检查文件是否损坏".to_string(),
            AppError::AmountExtractionError(_) => "无法从PDF中提取金额信息".to_string(),
            AppError::FileSystemError(_) => "文件操作失败，请检查文件权限".to_string(),
            AppError::NamingError(_) => "文件命名失败，请检查文件名格式".to_string(),
            AppError::PermissionError(_) => "没有足够的权限执行此操作".to_string(),
            AppError::IoError(_) => "输入输出操作失败".to_string(),
            AppError::ValidationError(_) => "数据验证失败".to_string(),
            AppError::UnknownError(_) => "发生未知错误".to_string(),
        }
    }

    /// 是否可以重试
    #[allow(dead_code)]
    pub fn is_retryable(&self) -> bool {
        match self {
            AppError::PdfParseError(_) => false,
            AppError::AmountExtractionError(_) => false,
            AppError::FileSystemError(_) => true,
            AppError::NamingError(_) => true,
            AppError::PermissionError(_) => false,
            AppError::IoError(_) => true,
            AppError::ValidationError(_) => false,
            AppError::UnknownError(_) => false,
        }
    }
}

/// 从标准错误类型转换
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::PermissionDenied => {
                AppError::permission_error(&format!("权限不足: {}", err))
            }
            std::io::ErrorKind::NotFound => {
                AppError::file_system_error(&format!("文件不存在: {}", err))
            }
            std::io::ErrorKind::AlreadyExists => {
                AppError::file_system_error(&format!("文件已存在: {}", err))
            }
            _ => AppError::io_error(&format!("IO错误: {}", err)),
        }
    }
}

/// 从PDF提取错误转换
impl From<pdf_extract::OutputError> for AppError {
    fn from(err: pdf_extract::OutputError) -> Self {
        AppError::pdf_parse_error(&format!("PDF解析错误: {}", err))
    }
}

/// 从正则表达式错误转换
impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> Self {
        AppError::validation_error(&format!("正则表达式错误: {}", err))
    }
}

/// 错误处理宏
#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err);
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(AppError::unknown_error(&format!($fmt, $($arg)*)));
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return Err(AppError::unknown_error(&format!($fmt, $($arg)*)));
        }
    };
}

/// 错误上下文扩展
pub trait ErrorContext<T> {
    #[allow(dead_code)]
    fn with_context(self, context: &str) -> AppResult<T>;
    #[allow(dead_code)]
    fn with_pdf_context(self, context: &str) -> AppResult<T>;
    #[allow(dead_code)]
    fn with_file_context(self, context: &str) -> AppResult<T>;
}

impl<T> ErrorContext<T> for Result<T, AppError> {
    #[allow(dead_code)]
    fn with_context(self, context: &str) -> AppResult<T> {
        self.map_err(|e| AppError::unknown_error(&format!("{}: {}", context, e)))
    }

    #[allow(dead_code)]
    fn with_pdf_context(self, context: &str) -> AppResult<T> {
        self.map_err(|e| AppError::pdf_parse_error(&format!("{}: {}", context, e)))
    }

    #[allow(dead_code)]
    fn with_file_context(self, context: &str) -> AppResult<T> {
        self.map_err(|e| AppError::file_system_error(&format!("{}: {}", context, e)))
    }
}

 
// 模块声明
mod types;
mod error;
mod config;
mod pdf_service;
mod file_service;
mod naming_engine;

use types::*;
use config::*;
use pdf_service::*;
use file_service::*;
use naming_engine::*;

use std::sync::Mutex;
use tauri::State;

// 全局状态管理
struct AppState {
    config_manager: Mutex<ConfigManager>,
    pdf_parser: Mutex<PdfParser>,
    file_service: Mutex<FileService>,
    naming_engine: Mutex<NamingEngine>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 扫描指定目录中的PDF文件
#[tauri::command]
async fn scan_pdf_files(directory: String, state: State<'_, AppState>) -> Result<Vec<PdfFile>, String> {
    let file_service = state.file_service.lock().unwrap();
    let pdf_parser = state.pdf_parser.lock().unwrap();
    let naming_engine = state.naming_engine.lock().unwrap();
    
    // 扫描PDF文件
    let mut files = file_service.scan_directory(&directory)
        .map_err(|e| e.to_string())?;
    
    // 为每个文件分析PDF内容并生成建议文件名
    for file in &mut files {
        match pdf_parser.analyze_pdf(&file.path) {
            Ok(invoice_info) => {
                file.amount = invoice_info.amount;
                file.suggested_name = Some(naming_engine.generate_suggested_name(&file));
            }
            Err(e) => {
                eprintln!("分析PDF文件失败 {}: {}", file.path, e);
                file.suggested_name = Some("未知金额_发票.pdf".to_string());
            }
        }
    }
    
    Ok(files)
}

/// 分析单个PDF文件的内容
#[tauri::command]
async fn analyze_pdf_content(file_path: String, state: State<'_, AppState>) -> Result<InvoiceInfo, String> {
    let pdf_parser = state.pdf_parser.lock().unwrap();
    
    pdf_parser.analyze_pdf(&file_path)
        .map_err(|e| e.to_string())
}

/// 生成重命名预览
#[tauri::command]
async fn preview_rename(file_paths: Vec<String>, state: State<'_, AppState>) -> Result<Vec<RenamePreview>, String> {
    let file_service = state.file_service.lock().unwrap();
    let pdf_parser = state.pdf_parser.lock().unwrap();
    let naming_engine = state.naming_engine.lock().unwrap();
    
    let mut previews = Vec::new();
    
    for file_path in file_paths {
        // 创建文件信息
        let file_info = match file_service.create_pdf_file_info(&std::path::Path::new(&file_path)) {
            Ok(info) => info,
            Err(e) => {
                eprintln!("创建文件信息失败 {}: {}", file_path, e);
                continue;
            }
        };
        
        // 分析PDF内容
        let amount = match pdf_parser.analyze_pdf(&file_path) {
            Ok(invoice_info) => invoice_info.amount,
            Err(_) => None,
        };
        
        // 生成预览
        let suggested_name = match amount {
            Some(amt) => naming_engine.generate_filename(amt),
            None => "未知金额_发票.pdf".to_string(),
        };
        
        previews.push(RenamePreview {
            original_name: file_info.name,
            suggested_name,
            amount,
        });
    }
    
    Ok(previews)
}

/// 执行文件重命名
#[tauri::command]
async fn execute_rename(renames: Vec<RenameOperation>, state: State<'_, AppState>) -> Result<RenameResult, String> {
    let file_service = state.file_service.lock().unwrap();
    
    let mut success_count = 0;
    let mut failed_files = Vec::new();
    
    for rename_op in renames {
        match file_service.rename_file(&rename_op.old_path, &rename_op.new_path) {
            Ok(()) => {
                success_count += 1;
            }
            Err(e) => {
                failed_files.push(format!("{}: {}", rename_op.old_path, e));
            }
        }
    }
    
    let total_files = success_count + failed_files.len();
    let success = failed_files.is_empty();
    
    let message = if success {
        format!("成功重命名 {} 个文件", success_count)
    } else {
        format!("成功重命名 {} 个文件，失败 {} 个文件", success_count, failed_files.len())
    };
    
    Ok(RenameResult {
        success,
        message,
        processed_files: total_files,
        failed_files,
    })
}

/// 选择目录
#[tauri::command]
async fn select_directory() -> Result<String, String> {
    // 获取应用程序所在的目录
    
    // 首先尝试获取可执行文件所在的目录
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Some(exe_dir_str) = exe_dir.to_str() {
                // 在 macOS 应用包中，可执行文件通常在 .app/Contents/MacOS/ 目录下
                // 我们需要找到一个更合适的目录
                let exe_dir_path = std::path::Path::new(exe_dir_str);
                
                // 如果在 .app 包中，尝试找到应用包的父目录
                if exe_dir_str.contains(".app/Contents/MacOS") {
                    // 向上查找到 .app 目录，然后取其父目录
                    let mut current_path = exe_dir_path;
                    while let Some(parent) = current_path.parent() {
                        if let Some(parent_str) = parent.to_str() {
                            if parent_str.ends_with(".app") {
                                // 找到了 .app 目录，返回其父目录
                                if let Some(app_parent) = parent.parent() {
                                    if let Some(app_parent_str) = app_parent.to_str() {
                                        return Ok(app_parent_str.to_string());
                                    }
                                }
                                break;
                            }
                        }
                        current_path = parent;
                    }
                }
                
                // 如果不在 .app 包中，直接返回可执行文件所在目录
                return Ok(exe_dir_str.to_string());
            }
        }
    }
    
    // 尝试获取当前工作目录（但跳过根目录）
    if let Ok(current_dir) = std::env::current_dir() {
        if let Some(current_dir_str) = current_dir.to_str() {
            if current_dir_str != "/" {
                return Ok(current_dir_str.to_string());
            }
        }
    }
    
    // 如果都失败了，返回用户主目录
    let home_dir = std::env::var("HOME")
        .unwrap_or_else(|_| "/Users".to_string());
    Ok(home_dir)
}

/// 获取应用配置
#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config_manager = state.config_manager.lock().unwrap();
    Ok(config_manager.get_config().clone())
}

/// 更新应用配置
#[tauri::command]
async fn update_config(new_config: AppConfig, state: State<'_, AppState>) -> Result<(), String> {
    let mut config_manager = state.config_manager.lock().unwrap();
    config_manager.update_config(new_config)
        .map_err(|e| e.to_string())
}

/// 重置配置为默认值
#[tauri::command]
async fn reset_config(state: State<'_, AppState>) -> Result<(), String> {
    let mut config_manager = state.config_manager.lock().unwrap();
    config_manager.reset_to_default()
        .map_err(|e| e.to_string())
}

/// 验证目录权限
#[tauri::command]
async fn validate_directory(directory: String, state: State<'_, AppState>) -> Result<bool, String> {
    let file_service = state.file_service.lock().unwrap();
    Ok(file_service.is_directory_writable(&directory))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化应用状态
    let app_state = AppState {
        config_manager: Mutex::new(ConfigManager::new().expect("无法初始化配置管理器")),
        pdf_parser: Mutex::new(PdfParser::new()),
        file_service: Mutex::new(FileService::new()),
        naming_engine: Mutex::new(NamingEngine::new()),
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_pdf_files,
            analyze_pdf_content,
            preview_rename,
            execute_rename,
            select_directory,
            get_config,
            update_config,
            reset_config,
            validate_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

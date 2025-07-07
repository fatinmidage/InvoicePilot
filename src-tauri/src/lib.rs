// 模块声明
mod types;
mod error;
mod config;
mod pdf_service;
mod file_service;
mod naming_engine;
pub mod directory_utils;

use types::*;
use config::*;
use pdf_service::*;
use file_service::*;
use naming_engine::*;
use directory_utils::*;

use std::sync::Mutex;
use tauri::State;

// 全局状态管理
struct AppState {
    config_manager: Mutex<ConfigManager>,
    pdf_parser: Mutex<PdfParser>,
    file_service: Mutex<FileService>,
    naming_engine: Mutex<NamingEngine>,
    directory_utils: Mutex<DirectoryUtils>,
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
    
    // 解决重名冲突
    // 第一步：解决批量文件内部的重名冲突
    let resolved_names = naming_engine.resolve_naming_conflicts(&files);
    
    // 第二步：检查并解决与目录中已存在文件的冲突
    let final_names = naming_engine.resolve_directory_conflicts(&directory, &resolved_names);
    
    // 第三步：将最终解决冲突后的文件名更新到每个文件
    for (i, file) in files.iter_mut().enumerate() {
        if let Some(final_name) = final_names.get(i) {
            file.suggested_name = Some(final_name.clone());
        }
    }
    
    Ok(files)
}

/// 扫描指定目录中的图片文件
#[tauri::command]
async fn scan_image_files(directory: String, state: State<'_, AppState>) -> Result<Vec<ImageFile>, String> {
    let file_service = state.file_service.lock().unwrap();
    let naming_engine = state.naming_engine.lock().unwrap();
    
    // 扫描图片文件
    let mut files = file_service.scan_image_files(&directory)
        .map_err(|e| e.to_string())?;
    
    // 为每个图片文件生成建议文件名（基于时间戳）
    for file in &mut files {
        let suggested_name = naming_engine.generate_image_filename(&file.name, &file.modified);
        file.suggested_name = Some(suggested_name);
    }
    
    // 解决重名冲突
    let resolved_names = naming_engine.resolve_image_naming_conflicts(&files);
    
    // 检查并解决与目录中已存在文件的冲突
    let final_names = naming_engine.resolve_directory_conflicts(&directory, &resolved_names);
    
    // 将最终解决冲突后的文件名更新到每个文件
    for (i, file) in files.iter_mut().enumerate() {
        if let Some(final_name) = final_names.get(i) {
            file.suggested_name = Some(final_name.clone());
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
async fn select_directory(state: State<'_, AppState>) -> Result<String, String> {
    let directory_utils = state.directory_utils.lock().unwrap();
    let directory = directory_utils.get_current_directory()?;
    Ok(directory)
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
    let directory_utils = state.directory_utils.lock().unwrap();
    Ok(directory_utils.is_directory_writable(&directory))
}

/// 验证目录是否有效
#[tauri::command]
async fn validate_directory_exists(directory: String, state: State<'_, AppState>) -> Result<bool, String> {
    let directory_utils = state.directory_utils.lock().unwrap();
    Ok(directory_utils.is_directory_valid(&directory))
}

/// 获取目录的父目录
#[tauri::command]
async fn get_parent_directory(directory: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    let directory_utils = state.directory_utils.lock().unwrap();
    Ok(directory_utils.get_parent_directory(&directory))
}

/// 规范化目录路径
#[tauri::command]
async fn normalize_directory_path(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let directory_utils = state.directory_utils.lock().unwrap();
    Ok(directory_utils.normalize_path(&path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化应用状态
    let app_state = AppState {
        config_manager: Mutex::new(ConfigManager::new().expect("无法初始化配置管理器")),
        pdf_parser: Mutex::new(PdfParser::new()),
        file_service: Mutex::new(FileService::new()),
        naming_engine: Mutex::new(NamingEngine::new()),
        directory_utils: Mutex::new(DirectoryUtils::new()),
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            scan_pdf_files,
            scan_image_files,
            analyze_pdf_content,
            preview_rename,
            execute_rename,
            select_directory,
            get_config,
            update_config,
            reset_config,
            validate_directory,
            validate_directory_exists,
            get_parent_directory,
            normalize_directory_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 导入Tauri API
import { invoke } from '@tauri-apps/api/core';

// 文件接口定义
interface FileItem {
  id: string;
  name: string;
  path: string;
  size: number;
  modified: string;
  amount?: number;
  suggested_name?: string;
  selected: boolean;
}

// 图片文件接口定义
interface ImageFile {
  id: string;
  name: string;
  path: string;
  size: number;
  modified: string;
  suggested_name?: string;
  selected: boolean;
}

// 重命名操作接口
interface RenameOperation {
  old_path: string;
  new_path: string;
  amount?: number;
}

// 重命名结果接口
interface RenameResult {
  success: boolean;
  message: string;
  processed_files: number;
  failed_files: string[];
}

// Tab管理器
class TabManager {
  private currentTab: string = 'pdf';
  private onTabChange: (tab: string) => Promise<void>;

  constructor(onTabChange: (tab: string) => Promise<void>) {
    this.onTabChange = onTabChange;
    this.bindEvents();
  }

  private bindEvents() {
    const tabButtons = document.querySelectorAll('.tab-button');
    tabButtons.forEach(button => {
      button.addEventListener('click', () => {
        const tabId = button.getAttribute('data-tab');
        if (tabId) {
          this.switchTab(tabId);
        }
      });
    });
  }

  private async switchTab(tabId: string) {
    if (this.currentTab === tabId) return;

    // 更新按钮状态
    document.querySelectorAll('.tab-button').forEach(button => {
      button.classList.remove('active');
    });
    document.querySelector(`[data-tab="${tabId}"]`)?.classList.add('active');

    // 更新面板显示
    document.querySelectorAll('.tab-panel').forEach(panel => {
      panel.classList.remove('active');
    });
    document.getElementById(`${tabId}-tab`)?.classList.add('active');

    this.currentTab = tabId;
    
    // 等待异步操作完成
    await this.onTabChange(tabId);
  }

  getCurrentTab(): string {
    return this.currentTab;
  }
}

// 应用状态
class AppState {
  files: FileItem[] = [];
  selectedFiles: Set<string> = new Set();
  currentDirectory: string = "";
  currentTab: string = "pdf";

  constructor() {
    // 不在构造函数中调用异步方法
  }

  // 初始化文件列表（使用真实API）
  async initializeFiles() {
    try {
      // 获取默认目录
      this.currentDirectory = await invoke<string>('select_directory');
      await this.loadFiles();
    } catch (error) {
      console.error('❌ 初始化文件列表失败:', error);
      // 如果失败，显示错误状态
      this.currentDirectory = "获取目录失败";
      this.files = [];
    }
  }

  // 加载文件列表（根据当前tab类型）
  async loadFiles() {
    try {
      if (this.currentTab === 'pdf') {
        const files = await invoke<FileItem[]>('scan_pdf_files', { 
          directory: this.currentDirectory 
        });
        
        this.files = files.map(file => ({
          ...file,
          selected: false
        }));
      } else if (this.currentTab === 'image') {
        const files = await invoke<ImageFile[]>('scan_image_files', { 
          directory: this.currentDirectory 
        });
        
        this.files = files.map(file => ({
          ...file,
          selected: false
        }));
      }
      
      // 清空选择状态
      this.selectedFiles.clear();
    } catch (error) {
      console.error('❌ 加载文件列表失败:', error);
      this.files = [];
      // 发生错误时也要清空选择状态
      this.selectedFiles.clear();
    }
  }

  // 设置当前tab并重新加载文件
  async setCurrentTab(tab: string) {
    this.currentTab = tab;
    await this.loadFiles();
  }

  // 设置当前目录
  async setCurrentDirectory(directory: string) {
    this.currentDirectory = directory;
    await this.loadFiles();
  }

  // 切换文件选择状态
  toggleFileSelection(fileId: string) {
    const file = this.files.find(f => f.id === fileId);
    if (file) {
      file.selected = !file.selected;
      if (file.selected) {
        this.selectedFiles.add(fileId);
      } else {
        this.selectedFiles.delete(fileId);
      }
    }
  }

  // 全选/取消全选
  toggleSelectAll() {
    const allSelected = this.files.every(f => f.selected);
    this.files.forEach(file => {
      file.selected = !allSelected;
      if (file.selected) {
        this.selectedFiles.add(file.id);
      } else {
        this.selectedFiles.delete(file.id);
      }
    });
  }

  // 获取选中的文件数量
  getSelectedCount() {
    return this.selectedFiles.size;
  }

  // 获取总文件数
  getTotalCount() {
    return this.files.length;
  }

  // 获取选中的文件列表
  getSelectedFiles() {
    return this.files.filter(f => f.selected);
  }
}

// UI管理器
class UIManager {
  private appState: AppState;
  private fileListElement!: HTMLElement;
  private selectAllCheckbox!: HTMLInputElement;
  private selectAllLabel!: HTMLElement;
  private startRenameButton!: HTMLButtonElement;
  private previewArea!: HTMLElement;

  constructor(appState: AppState) {
    this.appState = appState;
    this.initializeElements();
    this.bindEvents();
    this.render();
  }

  // 根据当前tab动态获取正确的元素
  private getCurrentElements() {
    const isImageTab = this.appState.currentTab === 'image';
    
    return {
      fileListElement: document.getElementById(isImageTab ? "image-file-list" : "file-list") as HTMLElement,
      selectAllCheckbox: document.getElementById(isImageTab ? "image-select-all" : "select-all") as HTMLInputElement,
      selectAllLabel: document.querySelector(isImageTab ? '#image-tab .select-all-label' : '#pdf-tab .select-all-label') as HTMLElement,
      startRenameButton: document.getElementById(isImageTab ? "start-image-rename" : "start-rename") as HTMLButtonElement,
      previewArea: document.getElementById(isImageTab ? "image-preview-area" : "preview-area") as HTMLElement
    };
  }

  private initializeElements() {
    // 初始化默认元素（PDF标签页）
    const elements = this.getCurrentElements();
    this.fileListElement = elements.fileListElement;
    this.selectAllCheckbox = elements.selectAllCheckbox;
    this.selectAllLabel = elements.selectAllLabel;
    this.startRenameButton = elements.startRenameButton;
    this.previewArea = elements.previewArea;
  }

  // 更新当前元素引用
  private updateCurrentElements() {
    const elements = this.getCurrentElements();
    this.fileListElement = elements.fileListElement;
    this.selectAllCheckbox = elements.selectAllCheckbox;
    this.selectAllLabel = elements.selectAllLabel;
    this.startRenameButton = elements.startRenameButton;
    this.previewArea = elements.previewArea;
  }

  // 绑定事件
  private bindEvents() {
    // 刷新按钮事件
    document.getElementById("refresh-btn")?.addEventListener("click", () => {
      this.refreshFiles();
    });

    // 设置按钮事件
    document.getElementById("settings-btn")?.addEventListener("click", () => {
      this.openSettings();
    });

    // 绑定两个标签页的全选和重命名按钮事件
    this.bindTabEvents('pdf');
    this.bindTabEvents('image');
  }

  // 为特定标签页绑定事件
  private bindTabEvents(tabType: 'pdf' | 'image') {
    const prefix = tabType === 'image' ? 'image-' : '';
    
    // 全选复选框事件
    const selectAllCheckbox = document.getElementById(`${prefix}select-all`) as HTMLInputElement;
    if (selectAllCheckbox) {
      selectAllCheckbox.addEventListener("change", () => {
        if (this.appState.currentTab === tabType) {
          this.appState.toggleSelectAll();
          this.render();
        }
      });
    }

    // 开始重命名按钮事件
    const startRenameButton = document.getElementById(`${prefix === 'image-' ? 'start-image-rename' : 'start-rename'}`) as HTMLButtonElement;
    if (startRenameButton) {
      startRenameButton.addEventListener("click", () => {
        if (this.appState.currentTab === tabType) {
          this.startRename();
        }
      });
    }
  }

  // 渲染文件列表
  private renderFileList() {
    // 更新当前元素引用
    this.updateCurrentElements();
    
    if (!this.fileListElement) {
      console.error('文件列表元素未找到');
      return;
    }

    this.fileListElement.innerHTML = "";

    this.appState.files.forEach(file => {
      const fileItemElement = document.createElement("div");
      fileItemElement.className = `file-item ${file.selected ? "selected" : ""}`;
      
      // 格式化文件大小
      const formattedSize = this.formatFileSize(file.size);
      // 格式化修改时间
      const formattedDate = this.formatDate(file.modified);
      
      // 根据当前tab显示不同的图标
      const fileIcon = this.appState.currentTab === 'pdf' ? '📄' : '🖼️';
      
      fileItemElement.innerHTML = `
        <input type="checkbox" class="checkbox file-checkbox" data-file-id="${file.id}" ${file.selected ? "checked" : ""}>
        <div class="file-icon">${fileIcon}</div>
        <div class="file-info">
          <div class="file-name" title="${file.name}">${file.name}</div>
          <div class="file-meta">${formattedSize} • ${formattedDate}</div>
        </div>
      `;

      // 添加点击事件
      fileItemElement.addEventListener("click", (e) => {
        if (e.target instanceof HTMLInputElement) {
          return; // 如果点击的是复选框，让复选框处理
        }
        this.toggleFileSelection(file.id);
      });

      // 复选框事件
      const checkbox = fileItemElement.querySelector(".file-checkbox") as HTMLInputElement;
      checkbox.addEventListener("change", () => {
        this.toggleFileSelection(file.id);
      });

      this.fileListElement.appendChild(fileItemElement);
    });
  }

  // 格式化文件大小
  private formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
  }

  // 格式化日期
  private formatDate(dateString: string): string {
    try {
      const date = new Date(dateString);
      return date.toLocaleDateString('zh-CN') + ' ' + date.toLocaleTimeString('zh-CN', {
        hour: '2-digit',
        minute: '2-digit'
      });
    } catch {
      return dateString;
    }
  }

  // 渲染统计信息
  private renderStats() {
    // 更新当前元素引用
    this.updateCurrentElements();
    
    if (!this.selectAllLabel || !this.selectAllCheckbox || !this.startRenameButton) {
      console.error('统计信息元素未找到');
      return;
    }

    const selectedCount = this.appState.getSelectedCount();
    const totalCount = this.appState.getTotalCount();

    // 更新全选标签
    this.selectAllLabel.textContent = `全选 (${selectedCount}/${totalCount})`;

    // 更新全选复选框状态
    if (selectedCount === 0) {
      this.selectAllCheckbox.checked = false;
      this.selectAllCheckbox.indeterminate = false;
    } else if (selectedCount === totalCount) {
      this.selectAllCheckbox.checked = true;
      this.selectAllCheckbox.indeterminate = false;
    } else {
      this.selectAllCheckbox.checked = false;
      this.selectAllCheckbox.indeterminate = true;
    }

    // 更新重命名按钮
    this.startRenameButton.disabled = selectedCount === 0;
    this.startRenameButton.textContent = `开始重命名 (${selectedCount})`;
  }

  // 渲染预览区域
  private renderPreview() {
    // 更新当前元素引用
    this.updateCurrentElements();
    
    if (!this.previewArea) {
      console.error('预览区域元素未找到');
      return;
    }

    const selectedFiles = this.appState.getSelectedFiles();
    
    if (selectedFiles.length === 0) {
      const fileType = this.appState.currentTab === 'pdf' ? '文件' : '图片文件';
      this.previewArea.innerHTML = `
        <p class="placeholder-text">请选择要重命名的${fileType}</p>
      `;
    } else {
      this.previewArea.innerHTML = `
        <div class="preview-content">
          ${selectedFiles.map(file => `
            <div class="preview-item">
              <div class="original-name">原名：${file.name}</div>
              <div class="new-name">新名：${this.generateNewName(file)}</div>
            </div>
          `).join("")}
        </div>
      `;
    }
  }

  // 生成新文件名（根据文件类型）
  private generateNewName(file: FileItem): string {
    if (file.suggested_name) {
      return file.suggested_name;
    }
    
    if (this.appState.currentTab === 'pdf') {
      if (file.amount) {
        return `${file.amount.toFixed(2)}元_发票.pdf`;
      }
      return "未知金额_发票.pdf";
    } else {
      // 图片文件保持原文件名
      return file.name;
    }
  }

  // 切换文件选择状态
  private toggleFileSelection(fileId: string) {
    this.appState.toggleFileSelection(fileId);
    this.render();
  }

  // 刷新文件列表
  private async refreshFiles() {
    try {
      await this.appState.loadFiles();
      this.render();
    } catch (error) {
      console.error("刷新文件列表失败:", error);
      alert("刷新文件列表失败");
    }
  }

  // 打开设置
  private openSettings() {
    // 创建设置对话框
    const settingsHTML = `
      <div class="settings-overlay">
        <div class="settings-modal">
          <div class="settings-header">
            <h3>关于 InvoicePilot</h3>
            <button class="close-btn" id="close-settings">×</button>
          </div>
          <div class="settings-content">
            <div class="app-info">
              <div class="app-icon-large">📄</div>
              <div class="app-details">
                <h4>InvoicePilot</h4>
                <p class="app-description">PDF发票文件重命名工具</p>
                <div class="version-info">
                  <p><strong>版本：</strong>0.1.0</p>
                  <p><strong>作者：</strong>Wind</p>
                  <p><strong>邮箱：</strong>fg1048596@gmail.com</p>
                </div>
              </div>
            </div>
            <div class="features-info">
              <h5>功能特点</h5>
              <ul>
                <li>智能识别PDF发票内容</li>
                <li>自动提取金额信息</li>
                <li>批量重命名文件</li>
                <li>支持多种文件格式</li>
              </ul>
            </div>
          </div>
        </div>
      </div>
    `;
    
    // 添加对话框到页面
    const settingsElement = document.createElement('div');
    settingsElement.innerHTML = settingsHTML;
    document.body.appendChild(settingsElement);
    
    // 绑定关闭事件
    const closeBtn = document.getElementById('close-settings');
    const overlay = document.querySelector('.settings-overlay') as HTMLElement;
    
    const closeSettings = () => {
      document.body.removeChild(settingsElement);
    };
    
    closeBtn?.addEventListener('click', closeSettings);
    overlay?.addEventListener('click', (e) => {
      if (e.target === overlay) {
        closeSettings();
      }
    });
    
    // ESC键关闭
    const handleEsc = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        closeSettings();
        document.removeEventListener('keydown', handleEsc);
      }
    };
    document.addEventListener('keydown', handleEsc);
  }

  // 开始重命名
  private async startRename() {
    const selectedFiles = this.appState.getSelectedFiles();
    if (selectedFiles.length === 0) return;

    try {
      this.startRenameButton.disabled = true;
      this.startRenameButton.textContent = "重命名中...";
      
      // 准备重命名操作
      const renameOperations: RenameOperation[] = selectedFiles.map(file => {
        // 处理Windows和Unix路径分隔符
        const lastSeparatorIndex = Math.max(file.path.lastIndexOf('/'), file.path.lastIndexOf('\\'));
        const directory = file.path.substring(0, lastSeparatorIndex);
        const separator = file.path.includes('\\') ? '\\' : '/';
        
        // 根据当前tab类型生成不同的默认文件名
        let newFileName: string;
        if (this.appState.currentTab === 'pdf') {
          newFileName = file.suggested_name || `${file.amount?.toFixed(2) || '未知金额'}元_发票.pdf`;
        } else {
          // 图片文件使用suggested_name，如果没有则保持原名
          newFileName = file.suggested_name || file.name;
        }
        
        const newPath = `${directory}${separator}${newFileName}`;
        
        return {
          old_path: file.path,
          new_path: newPath,
          amount: file.amount
        };
      });
      
      // 执行重命名
      const result = await invoke<RenameResult>('execute_rename', { 
        renames: renameOperations 
      });
      
      if (result.success) {
        // 重命名成功后清空选择并刷新
        this.appState.selectedFiles.clear();
        this.appState.files.forEach(file => file.selected = false);
        await this.appState.loadFiles();
        this.render();
        
        alert(`${result.message}`);
      } else {
        let errorMessage = `重命名部分失败: ${result.message}`;
        if (result.failed_files.length > 0) {
          errorMessage += "\n\n失败详情:\n" + result.failed_files.join("\n");
          console.error("失败的文件:", result.failed_files);
        }
        alert(errorMessage);
      }
      
    } catch (error) {
      console.error("重命名失败:", error);
      alert("重命名失败，请检查文件权限或重试");
    } finally {
      this.startRenameButton.disabled = false;
      const selectedCount = this.appState.getSelectedCount();
      this.startRenameButton.textContent = `开始重命名 (${selectedCount})`;
    }
  }

  // 渲染整个UI
  public render() {
    this.renderFileList();
    this.renderStats();
    this.renderPreview();
    this.updateDirectoryDisplay();
  }

  // 公共刷新方法
  public async refresh() {
    try {
      await this.appState.loadFiles();
      this.render();
    } catch (error) {
      console.error("刷新文件列表失败:", error);
      alert("刷新文件列表失败");
    }
  }

  // 更新目录显示
  private updateDirectoryDisplay() {
    const directoryElement = document.getElementById("current-directory");
    const fileCountElement = document.getElementById("file-count");
    
    if (directoryElement) {
      directoryElement.textContent = this.appState.currentDirectory || "未选择目录";
    }
    
    if (fileCountElement) {
      const totalFiles = this.appState.getTotalCount();
      const fileType = this.appState.currentTab === 'pdf' ? 'PDF' : '图片';
      fileCountElement.textContent = `${totalFiles} 个${fileType}文件`;
    }
    
    // 同时更新图片标签页的目录显示
    const imageDirectoryElement = document.getElementById("image-current-directory");
    const imageFileCountElement = document.getElementById("image-file-count");
    
    if (imageDirectoryElement) {
      imageDirectoryElement.textContent = this.appState.currentDirectory || "未选择目录";
    }
    
    if (imageFileCountElement) {
      const totalFiles = this.appState.getTotalCount();
      imageFileCountElement.textContent = `${totalFiles} 个图片文件`;
    }
  }
}

// 应用初始化
window.addEventListener("DOMContentLoaded", async () => {
  try {
    const appState = new AppState();
    
    // 等待异步初始化完成
    await appState.initializeFiles();
    
    const uiManager = new UIManager(appState);
    
    // 初始化Tab管理器
    const tabManager = new TabManager(async (tabId: string): Promise<void> => {
      // 更新AppState的当前tab并重新加载文件
      await appState.setCurrentTab(tabId);
      uiManager.render();
      
      // 根据当前标签页更新刷新按钮的行为
      const refreshBtn = document.getElementById("refresh-btn");
      if (refreshBtn) {
        refreshBtn.onclick = () => {
          uiManager.refresh();
        };
      }
    });
    
    // 将管理器保存到全局作用域以供调试使用
    (window as any).uiManager = uiManager;
    (window as any).tabManager = tabManager;
    
  } catch (error) {
    console.error("应用初始化失败:", error);
    // 如果初始化失败，仍然创建UI但使用空状态
    const appState = new AppState();
    appState.files = [];
    const uiManager = new UIManager(appState);
    const tabManager = new TabManager(async (tabId: string): Promise<void> => {
      // 切换标签页处理（错误状态下的回调）
    });
    (window as any).uiManager = uiManager;
    (window as any).tabManager = tabManager;
  }
});

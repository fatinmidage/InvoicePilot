// 文件接口定义
interface FileItem {
  id: string;
  name: string;
  size: string;
  date: string;
  selected: boolean;
}

// 应用状态
class AppState {
  files: FileItem[] = [];
  selectedFiles: Set<string> = new Set();

  constructor() {
    this.initializeFiles();
  }

  // 初始化文件列表（模拟数据）
  initializeFiles() {
    this.files = [
      {
        id: "1",
        name: "invoice_2024_001.pdf",
        size: "245 KB",
        date: "2024-01-15 10:30",
        selected: false
      },
      {
        id: "2",
        name: "bill_company_abc.pdf",
        size: "189 KB",
        date: "2024-01-16 14:22",
        selected: false
      },
      {
        id: "3",
        name: "receipt_jan_2024.pdf",
        size: "156 KB",
        date: "2024-01-20 09:15",
        selected: false
      },
      {
        id: "4",
        name: "invoice_supplier_xyz_20240125.pdf",
        size: "298 KB",
        date: "2024-01-25 16:45",
        selected: false
      },
      {
        id: "5",
        name: "billing_statement_feb.pdf",
        size: "203 KB",
        date: "2024-02-01 11:30",
        selected: false
      },
      {
        id: "6",
        name: "invoice_march_2024_company_def.pdf",
        size: "324 KB",
        date: "2024-03-05 09:45",
        selected: false
      },
      {
        id: "7",
        name: "receipt_grocery_store_march.pdf",
        size: "167 KB",
        date: "2024-03-10 14:20",
        selected: false
      },
      {
        id: "8",
        name: "utility_bill_march_2024.pdf",
        size: "234 KB",
        date: "2024-03-15 16:30",
        selected: false
      },
      {
        id: "9",
        name: "invoice_consulting_services_q1.pdf",
        size: "412 KB",
        date: "2024-03-20 11:15",
        selected: false
      },
      {
        id: "10",
        name: "tax_document_personal_2024.pdf",
        size: "567 KB",
        date: "2024-03-25 13:50",
        selected: false
      },
      {
        id: "11",
        name: "invoice_office_supplies_march.pdf",
        size: "278 KB",
        date: "2024-03-28 08:30",
        selected: false
      },
      {
        id: "12",
        name: "receipt_restaurant_business_dinner.pdf",
        size: "143 KB",
        date: "2024-04-02 19:45",
        selected: false
      },
      {
        id: "13",
        name: "invoice_software_license_annual.pdf",
        size: "389 KB",
        date: "2024-04-05 10:20",
        selected: false
      },
      {
        id: "14",
        name: "billing_statement_april_2024.pdf",
        size: "312 KB",
        date: "2024-04-10 15:30",
        selected: false
      },
      {
        id: "15",
        name: "invoice_marketing_campaign_q2.pdf",
        size: "456 KB",
        date: "2024-04-15 12:00",
        selected: false
      },
      {
        id: "16",
        name: "这是一个非常长的PDF文件名用于测试显示效果当文件名超过60个字符时会发生什么情况测试长度是否足够显示完整内容！.pdf",
        size: "789 KB",
        date: "2024-04-20 16:30",
        selected: false
      }
    ];
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

  // 初始化DOM元素
  private initializeElements() {
    this.fileListElement = document.getElementById("file-list") as HTMLElement;
    this.selectAllCheckbox = document.getElementById("select-all") as HTMLInputElement;
    this.selectAllLabel = document.querySelector(".select-all-label") as HTMLElement;
    this.startRenameButton = document.getElementById("start-rename") as HTMLButtonElement;
    this.previewArea = document.getElementById("preview-area") as HTMLElement;
  }

  // 绑定事件
  private bindEvents() {
    // 全选复选框事件
    this.selectAllCheckbox.addEventListener("change", () => {
      this.appState.toggleSelectAll();
      this.render();
    });

    // 刷新按钮事件
    document.getElementById("refresh-btn")?.addEventListener("click", () => {
      this.refreshFiles();
    });

    // 设置按钮事件
    document.getElementById("settings-btn")?.addEventListener("click", () => {
      this.openSettings();
    });

    // 开始重命名按钮事件
    this.startRenameButton.addEventListener("click", () => {
      this.startRename();
    });
  }

  // 渲染文件列表
  private renderFileList() {
    this.fileListElement.innerHTML = "";

    this.appState.files.forEach(file => {
      const fileItemElement = document.createElement("div");
      fileItemElement.className = `file-item ${file.selected ? "selected" : ""}`;
      fileItemElement.innerHTML = `
        <input type="checkbox" class="checkbox file-checkbox" data-file-id="${file.id}" ${file.selected ? "checked" : ""}>
        <div class="file-icon">📄</div>
        <div class="file-info">
          <div class="file-name" title="${file.name}">${file.name}</div>
          <div class="file-meta">${file.size} • ${file.date}</div>
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

  // 渲染统计信息
  private renderStats() {
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
    const selectedFiles = this.appState.getSelectedFiles();
    
    if (selectedFiles.length === 0) {
      this.previewArea.innerHTML = `
        <p class="placeholder-text">请选择要重命名的文件</p>
      `;
    } else {
      this.previewArea.innerHTML = `
        <div class="preview-content">
          <h4 style="margin-bottom: 12px; font-size: 0.85rem; color: var(--color-text-primary);">
            重命名预览 (${selectedFiles.length} 个文件)
          </h4>
          <div class="preview-list">
            ${selectedFiles.map(file => `
              <div class="preview-item" style="margin-bottom: 8px; padding: 8px; background-color: var(--color-surface-interactive); border-radius: 4px; font-size: 0.75rem;">
                <div style="color: var(--color-text-secondary); margin-bottom: 4px;">原名：${file.name}</div>
                <div style="color: var(--color-text-primary); font-weight: 500;">新名：${this.generateNewName(file.name)}</div>
              </div>
            `).join("")}
          </div>
        </div>
      `;
    }
  }

  // 生成新文件名（模拟智能重命名）
  private generateNewName(originalName: string): string {
    // 简单的重命名规则示例
    const baseName = originalName.replace(".pdf", "");
    const currentDate = new Date().toISOString().split("T")[0];
    
    if (baseName.includes("invoice")) {
      return `发票_${currentDate}_${Math.random().toString(36).substr(2, 4).toUpperCase()}.pdf`;
    } else if (baseName.includes("bill")) {
      return `账单_${currentDate}_${Math.random().toString(36).substr(2, 4).toUpperCase()}.pdf`;
    } else if (baseName.includes("receipt")) {
      return `收据_${currentDate}_${Math.random().toString(36).substr(2, 4).toUpperCase()}.pdf`;
    } else {
      return `文件_${currentDate}_${Math.random().toString(36).substr(2, 4).toUpperCase()}.pdf`;
    }
  }

  // 切换文件选择状态
  private toggleFileSelection(fileId: string) {
    this.appState.toggleFileSelection(fileId);
    this.render();
  }

  // 刷新文件列表
  private refreshFiles() {
    console.log("刷新文件列表");
    // 这里可以调用Tauri命令获取实际的文件列表
    // await invoke("get_pdf_files");
    this.render();
  }

  // 打开设置
  private openSettings() {
    console.log("打开设置");
    // 这里可以打开设置对话框
  }

  // 开始重命名
  private async startRename() {
    const selectedFiles = this.appState.getSelectedFiles();
    if (selectedFiles.length === 0) return;

    console.log("开始重命名", selectedFiles);
    
    try {
      // 这里调用Tauri命令执行重命名
      // const result = await invoke("rename_files", { files: selectedFiles });
      
      // 模拟重命名过程
      this.startRenameButton.disabled = true;
      this.startRenameButton.textContent = "重命名中...";
      
      // 模拟延时
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // 重命名完成后清空选择
      this.appState.selectedFiles.clear();
      this.appState.files.forEach(file => file.selected = false);
      
      this.render();
      
      // 显示成功消息
      alert(`成功重命名 ${selectedFiles.length} 个文件！`);
      
    } catch (error) {
      console.error("重命名失败:", error);
      alert("重命名失败，请检查文件权限或重试");
    } finally {
      this.startRenameButton.disabled = false;
      this.startRenameButton.textContent = "开始重命名 (0)";
    }
  }

  // 渲染整个UI
  public render() {
    this.renderFileList();
    this.renderStats();
    this.renderPreview();
  }
}

// 应用初始化
window.addEventListener("DOMContentLoaded", () => {
  const appState = new AppState();
  const uiManager = new UIManager(appState);
  
  // 将uiManager保存到全局作用域以供调试使用
  (window as any).uiManager = uiManager;
  
  console.log("PDF发票文件重命名工具已启动");
});

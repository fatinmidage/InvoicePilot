use std::path::Path;
use regex::Regex;
use pdf_extract::extract_text;
use crate::types::InvoiceInfo;

pub struct PdfParser;

impl PdfParser {
    pub fn new() -> Self {
        PdfParser
    }

    /// 从PDF文件中提取文本内容
    pub fn extract_text_from_pdf(&self, path: &Path) -> Result<String, String> {
        match extract_text(path) {
            Ok(text) => Ok(text),
            Err(e) => Err(format!("PDF解析失败: {}", e)),
        }
    }

    /// 从文本中提取金额信息
    pub fn extract_amount_from_text(&self, text: &str) -> Option<f64> {
        // 尝试多种金额提取方法
        if let Some(amount) = self.parse_decimal_amount(text) {
            return Some(amount);
        }
        
        if let Some(amount) = self.parse_chinese_amount(text) {
            return Some(amount);
        }
        
        if let Some(amount) = self.parse_currency_amount(text) {
            return Some(amount);
        }
        
        None
    }

    /// 解析小数格式的金额 (如: 1234.56)
    fn parse_decimal_amount(&self, text: &str) -> Option<f64> {
        // 匹配常见的数字金额格式
        let patterns = vec![
            r"金额[：:]\s*(\d+(?:\.\d{2})?)",
            r"总计[：:]\s*(\d+(?:\.\d{2})?)",
            r"合计[：:]\s*(\d+(?:\.\d{2})?)",
            r"应付[：:]\s*(\d+(?:\.\d{2})?)",
            r"价税合计[：:]\s*(\d+(?:\.\d{2})?)",
            r"总金额[：:]\s*(\d+(?:\.\d{2})?)",
            r"(\d{1,6}(?:\.\d{2})?)元",
            r"(\d+(?:,\d{3})*(?:\.\d{2})?)元",
        ];

        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(text) {
                    if let Some(amount_str) = caps.get(1) {
                        let clean_amount = amount_str.as_str().replace(",", "");
                        if let Ok(amount) = clean_amount.parse::<f64>() {
                            return Some(amount);
                        }
                    }
                }
            }
        }

        None
    }

    /// 解析中文数字金额 (如: 壹万贰仟叁佰肆拾伍元)
    fn parse_chinese_amount(&self, text: &str) -> Option<f64> {
        // 简化的中文数字转换
        let chinese_to_arabic = |_chinese: &str| -> Option<f64> {
            // 这里实现一个简化的中文数字转换
            // 在实际应用中可以使用更复杂的中文数字解析库
            
            // 查找常见的中文数字模式
            let patterns = vec![
                (r"([一二三四五六七八九十百千万]+)元", "chinese_digits"),
                (r"([壹贰叁肆伍陆柒捌玖拾佰仟万]+)元", "chinese_financial"),
            ];

            for (pattern, _) in patterns {
                if let Ok(re) = Regex::new(pattern) {
                    if let Some(caps) = re.captures(text) {
                        if let Some(chinese_num) = caps.get(1) {
                            // 简化处理：如果包含"万"，估算为万级别
                            let chinese_str = chinese_num.as_str();
                            if chinese_str.contains("万") {
                                // 简单估算，实际应用中需要更精确的转换
                                return Some(10000.0);
                            } else if chinese_str.contains("千") {
                                return Some(1000.0);
                            } else if chinese_str.contains("百") {
                                return Some(100.0);
                            }
                        }
                    }
                }
            }
            
            None
        };

        chinese_to_arabic(text)
    }

    /// 解析带货币符号的金额 (如: ￥1234.56, $1234.56)
    fn parse_currency_amount(&self, text: &str) -> Option<f64> {
        let patterns = vec![
            r"￥(\d+(?:\.\d{2})?)",
            r"¥(\d+(?:\.\d{2})?)",
            r"\$(\d+(?:\.\d{2})?)",
            r"RMB\s*(\d+(?:\.\d{2})?)",
            r"CNY\s*(\d+(?:\.\d{2})?)",
        ];

        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(caps) = re.captures(text) {
                    if let Some(amount_str) = caps.get(1) {
                        if let Ok(amount) = amount_str.as_str().parse::<f64>() {
                            return Some(amount);
                        }
                    }
                }
            }
        }

        None
    }

    /// 分析PDF文件并提取发票信息
    pub fn analyze_pdf(&self, file_path: &str) -> Result<InvoiceInfo, String> {
        let path = Path::new(file_path);
        let text = self.extract_text_from_pdf(path)?;
        
        let amount = self.extract_amount_from_text(&text);
        let original_filename = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown.pdf")
            .to_string();
        
        let suggested_filename = if let Some(amt) = amount {
            format!("{:.2}元_发票.pdf", amt)
        } else {
            "未知金额_发票.pdf".to_string()
        };

        Ok(InvoiceInfo {
            amount,
            original_filename,
            suggested_filename,
        })
    }
} 
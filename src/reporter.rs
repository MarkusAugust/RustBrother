// src/reporter.rs - Template-based output formatting
// This module handles generating reports using HTML templates

use crate::{AnalysisResult, CssClass, ComplexityWarning, ComplexityWarningType, WarningSeverity};
use serde_json;
use std::collections::HashMap;

/// Generate a report in the specified format
pub fn generate_report(result: &AnalysisResult, format: &str) -> anyhow::Result<String> {
    match format.to_lowercase().as_str() {
        "json" => generate_json_report(result),
        "html" => generate_html_report(result),
        "text" | _ => generate_text_report(result), // Default to text
    }
}

/// Generate a human-readable text report with complexity warnings
fn generate_text_report(result: &AnalysisResult) -> anyhow::Result<String> {
    let mut report = String::new();
    
    // Header with summary statistics - EPIC THEME RESTORED!
    report.push_str("⚔️  RustBrother CSS Analysis Report\n");
    report.push_str("====================================\n\n");
    
    // Summary section
    let total_classes = result.used_classes.len() + result.unused_classes.len();
    let unused_percentage = if total_classes > 0 {
        (result.unused_classes.len() as f64 / total_classes as f64 * 100.0).round()
    } else {
        0.0
    };
    
    report.push_str(&format!("📊 Territory Analysis:\n"));
    report.push_str(&format!("  Total CSS classes found: {}\n", total_classes));
    report.push_str(&format!("  Active classes: {}\n", result.used_classes.len()));
    report.push_str(&format!("  Corrupted remnants: {} ({:.0}%)\n", result.unused_classes.len(), unused_percentage));
    report.push_str(&format!("  Files patrolled: {}\n", result.total_files_scanned));
    
    // Complexity warnings summary
    if !result.complexity_warnings.is_empty() {
        let (high, medium, low) = count_warnings_by_severity(&result.complexity_warnings);
        report.push_str(&format!("  ⚠️  Dark sorcery detected: {} (🔴 {} forbidden, 🟡 {} cursed, 🟢 {} tainted)\n", 
            result.complexity_warnings.len(), high, medium, low));
    }
    report.push_str("\n");
    
    // Complexity warnings section (show first, as it's about code quality)
    if !result.complexity_warnings.is_empty() {
        report.push_str("⚠️  Dark Sorcery Detected:\n");
        report.push_str("-------------------------\n");
        
        // Group warnings by file
        let warnings_by_file = group_warnings_by_file(&result.complexity_warnings);
        let mut sorted_files: Vec<_> = warnings_by_file.keys().collect();
        sorted_files.sort();
        
        for file_path in sorted_files {
            let warnings = warnings_by_file.get(file_path).unwrap();
            report.push_str(&format!("\n📄 {}:\n", file_path));
            
            for warning in warnings {
                let severity_icon = match warning.severity {
                    WarningSeverity::High => "🔴",
                    WarningSeverity::Medium => "🟡",
                    WarningSeverity::Low => "🟢",
                };
                
                let warning_type = format_warning_type(&warning.warning_type);
                
                report.push_str(&format!("  {} {} (line {})\n", severity_icon, warning_type, warning.line_number));
                report.push_str(&format!("     Spell pattern: {}\n", warning.pattern));
                report.push_str(&format!("     💡 {}\n", warning.suggestion));
                report.push_str("\n");
            }
        }
    }
    
    // Unused classes section
    if !result.unused_classes.is_empty() {
        report.push_str("🗑️  Remnants Marked for Purging:\n");
        report.push_str("-------------------------------\n");
        
        let mut classes_by_file = HashMap::new();
        for class in &result.unused_classes {
            classes_by_file
                .entry(&class.file_path)
                .or_insert_with(Vec::new)
                .push(class);
        }
        
        let mut sorted_files: Vec<_> = classes_by_file.keys().collect();
        sorted_files.sort();
        
        for file_path in sorted_files {
            let classes = classes_by_file.get(file_path).unwrap();
            report.push_str(&format!("\n📄 {}:\n", file_path));
            
            for class in classes {
                report.push_str(&format!("  • .{} (line {})\n", class.name, class.line_number));
            }
        }
    } else {
        report.push_str("🎉 Excellent! No CSS corruption detected in your territory!\n");
    }
    
    // Custom properties section
    if !result.used_custom_properties.is_empty() {
        report.push_str("\n\n🎨 CSS Artifacts:\n");
        report.push_str("-----------------\n");
        report.push_str(&format!("Found {} custom properties\n", result.used_custom_properties.len()));
    }
    
    report.push_str("\n⚔️  Enforcement complete!\n");
    
    // Add complexity suggestions at the end
    if !result.complexity_warnings.is_empty() {
        report.push_str("\n💡 Arcane Knowledge for Cleansing Dark Sorcery:\n");
        report.push_str("• Forge explicit class mapping: CLASS_MAP[variant][size]\n");
        report.push_str("• Banish complex logic into helper functions\n");
        report.push_str("• Consider CSS-in-JS libraries with better static analysis\n");
        report.push_str("• Break down large template expressions into smaller incantations\n");
    }
    
    Ok(report)
}

/// Generate a JSON report for programmatic consumption with complexity warnings
fn generate_json_report(result: &AnalysisResult) -> anyhow::Result<String> {
    let (high, medium, low) = count_warnings_by_severity(&result.complexity_warnings);
    
    let json_report = serde_json::json!({
        "summary": {
            "total_css_classes": result.used_classes.len() + result.unused_classes.len(),
            "used_classes": result.used_classes.len(),
            "unused_classes": result.unused_classes.len(),
            "unused_percentage": if result.used_classes.len() + result.unused_classes.len() > 0 {
                (result.unused_classes.len() as f64 / (result.used_classes.len() + result.unused_classes.len()) as f64 * 100.0).round()
            } else {
                0.0
            },
            "total_files_scanned": result.total_files_scanned,
            "css_files_scanned": result.total_css_files,
            "js_files_scanned": result.total_js_files,
            "custom_properties_found": result.used_custom_properties.len(),
            "complexity_warnings": {
                "total": result.complexity_warnings.len(),
                "high": high,
                "medium": medium,
                "low": low
            }
        },
        "unused_classes": result.unused_classes,
        "used_classes": result.used_classes,
        "custom_properties": result.used_custom_properties,
        "complexity_warnings": result.complexity_warnings,
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    Ok(serde_json::to_string_pretty(&json_report)?)
}

/// Generate an HTML report using templates
fn generate_html_report(result: &AnalysisResult) -> anyhow::Result<String> {
    let total_classes = result.used_classes.len() + result.unused_classes.len();
    let unused_percentage = if total_classes > 0 {
        (result.unused_classes.len() as f64 / total_classes as f64 * 100.0).round()
    } else {
        0.0
    };
    
    let (high, medium, low) = count_warnings_by_severity(&result.complexity_warnings);
    
    // Load main template
    let main_template = include_str!("../templates/report.html");
    
    // Generate content sections using templates
    let unused_content = generate_unused_classes_content(&result.unused_classes);
    let complexity_content = generate_complexity_warnings_content(&result.complexity_warnings);
    
    // Replace all template variables in the main template
    let html = main_template
        .replace("{{TIMESTAMP}}", &chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .replace("{{TOTAL_CLASSES}}", &total_classes.to_string())
        .replace("{{USED_CLASSES}}", &result.used_classes.len().to_string())
        .replace("{{UNUSED_CLASSES}}", &result.unused_classes.len().to_string())
        .replace("{{UNUSED_PERCENTAGE}}", &format!("{:.0}", unused_percentage))
        .replace("{{UNUSED_CLASSES_CONTENT}}", &unused_content)
        .replace("{{COMPLEXITY_WARNINGS_CONTENT}}", &complexity_content)
        .replace("{{COMPLEXITY_WARNINGS_TOTAL}}", &result.complexity_warnings.len().to_string())
        .replace("{{COMPLEXITY_HIGH}}", &high.to_string())
        .replace("{{COMPLEXITY_MEDIUM}}", &medium.to_string())
        .replace("{{COMPLEXITY_LOW}}", &low.to_string())
        .replace("{{TOTAL_FILES}}", &result.total_files_scanned.to_string())
        .replace("{{CSS_FILES}}", &result.total_css_files.to_string())
        .replace("{{CUSTOM_PROPERTIES}}", &result.used_custom_properties.len().to_string());

    Ok(html)
}

/// Generate the content for unused classes section using templates
fn generate_unused_classes_content(unused_classes: &[CssClass]) -> String {
    if unused_classes.is_empty() {
        return include_str!("../templates/no-unused.html").to_string();
    }

    // Group classes by file
    let classes_by_file = group_classes_by_file(unused_classes);
    let mut sorted_files: Vec<_> = classes_by_file.keys().collect();
    sorted_files.sort();

    // Generate content for each file using templates
    sorted_files
        .iter()
        .map(|&file_path| generate_file_group_html(file_path, classes_by_file.get(file_path).unwrap()))
        .collect::<Vec<String>>()
        .join("\n")
}

/// Generate the content for complexity warnings section using templates
fn generate_complexity_warnings_content(warnings: &[ComplexityWarning]) -> String {
    if warnings.is_empty() {
        return r#"<div class="no-unused">🎉 No dark sorcery detected! Your code is pure and righteous.</div>"#.to_string();
    }

    // Group warnings by file
    let warnings_by_file = group_warnings_by_file(warnings);
    let mut sorted_files: Vec<_> = warnings_by_file.keys().collect();
    sorted_files.sort();

    // Generate content for each file using templates
    sorted_files
        .iter()
        .map(|&file_path| generate_complexity_file_group_html(file_path, warnings_by_file.get(file_path).unwrap()))
        .collect::<Vec<String>>()
        .join("\n")
}

/// Generate HTML for a complexity warning file group using template
fn generate_complexity_file_group_html(file_path: &str, warnings: &[&ComplexityWarning]) -> String {
    let file_template = include_str!("../templates/complexity-file-group.html");
    
    // Generate all warning items for this file using template
    let warning_items = warnings
        .iter()
        .map(|warning| generate_complexity_warning_html(warning))
        .collect::<Vec<String>>()
        .join("\n        ");

    // Replace variables in file group template
    file_template
        .replace("{{FILE_PATH}}", file_path)
        .replace("{{WARNING_COUNT}}", &warnings.len().to_string())
        .replace("{{WARNING_ITEMS}}", &warning_items)
}

/// Generate HTML for a single complexity warning using template
fn generate_complexity_warning_html(warning: &ComplexityWarning) -> String {
    let warning_template = include_str!("../templates/complexity-warning.html");
    
    let severity_color = match warning.severity {
        WarningSeverity::High => "#fc8181",     // Red for high
        WarningSeverity::Medium => "#fbd38d",   // Orange for medium  
        WarningSeverity::Low => "#68d391",      // Green for low
    };
    
    warning_template
        .replace("{{WARNING_TYPE}}", &format_warning_type(&warning.warning_type))
        .replace("{{LINE_NUMBER}}", &warning.line_number.to_string())
        .replace("{{PATTERN}}", &html_escape(&warning.pattern))
        .replace("{{SUGGESTION}}", &html_escape(&warning.suggestion))
        .replace("{{SEVERITY_COLOR}}", severity_color)
}

/// Generate HTML for a single file group using template
fn generate_file_group_html(file_path: &str, classes: &[&CssClass]) -> String {
    let file_template = include_str!("../templates/file-group.html");
    
    // Generate all class items for this file using template
    let class_items = classes
        .iter()
        .map(|class| generate_class_item_html(class))
        .collect::<Vec<String>>()
        .join("\n        ");

    // Replace variables in file group template
    file_template
        .replace("{{FILE_PATH}}", file_path)
        .replace("{{CLASS_COUNT}}", &classes.len().to_string())
        .replace("{{CLASS_ITEMS}}", &class_items)
}

/// Generate HTML for a single class item using template
fn generate_class_item_html(class: &CssClass) -> String {
    let item_template = include_str!("../templates/class-item.html");
    
    item_template
        .replace("{{CLASS_NAME}}", &class.name)
        .replace("{{LINE_NUMBER}}", &class.line_number.to_string())
}

/// Helper function to group classes by file path
fn group_classes_by_file(classes: &[CssClass]) -> HashMap<&str, Vec<&CssClass>> {
    let mut grouped = HashMap::new();
    for class in classes {
        grouped
            .entry(class.file_path.as_str())
            .or_insert_with(Vec::new)
            .push(class);
    }
    grouped
}

// Helper functions for complexity warnings
fn count_warnings_by_severity(warnings: &[ComplexityWarning]) -> (usize, usize, usize) {
    let mut high = 0;
    let mut medium = 0;
    let mut low = 0;
    
    for warning in warnings {
        match warning.severity {
            WarningSeverity::High => high += 1,
            WarningSeverity::Medium => medium += 1,
            WarningSeverity::Low => low += 1,
        }
    }
    
    (high, medium, low)
}

fn group_warnings_by_file(warnings: &[ComplexityWarning]) -> HashMap<&str, Vec<&ComplexityWarning>> {
    let mut grouped = HashMap::new();
    for warning in warnings {
        grouped
            .entry(warning.file_path.as_str())
            .or_insert_with(Vec::new)
            .push(warning);
    }
    grouped
}

fn format_warning_type(warning_type: &ComplexityWarningType) -> &'static str {
    match warning_type {
        ComplexityWarningType::DynamicClassConstruction => "Forbidden dynamic class sorcery",
        ComplexityWarningType::DeepTemplateNesting => "Cursed template nesting",
        ComplexityWarningType::ConditionalClassAssignment => "Dark conditional magic",
        ComplexityWarningType::MultiVariablePattern => "Multi-variable dark arts",
        ComplexityWarningType::UntrackedDynamicPattern => "Untrackable shadow magic",
    }
}

/// Escape HTML entities
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
     .replace('\'', "&#x27;")
}
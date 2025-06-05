// src/complexity_analyzer.rs - FIXED VERSION

use regex::Regex;
use walkdir::WalkDir;
use std::fs;
use std::path::Path;
use crate::{ComplexityWarning, ComplexityWarningType, WarningSeverity, AnalysisConfig};

/// Find complexity warnings in JavaScript/React files
pub fn find_complexity_warnings(path: &Path, config: &AnalysisConfig) -> anyhow::Result<Vec<ComplexityWarning>> {
    let mut warnings = Vec::new();
    
    if !config.enable_complexity_warnings {
        return Ok(warnings);
    }
    
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| should_process_js_file(e.path(), config))
    {
        let file_path = entry.path().to_string_lossy().to_string();
        
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_warnings = analyze_file_complexity(&content, &file_path, config);
            warnings.extend(file_warnings);
        }
    }
    
    Ok(warnings)
}

/// Analyze a single file for complexity patterns
fn analyze_file_complexity(content: &str, file_path: &str, config: &AnalysisConfig) -> Vec<ComplexityWarning> {
    let mut warnings = Vec::new();
    
    // Pattern 1: Dynamic class construction with template literals
    warnings.extend(detect_dynamic_class_construction(content, file_path));
    
    // Pattern 2: Complex conditional class assignment
    warnings.extend(detect_conditional_class_assignment(content, file_path));
    
    // Pattern 3: Deep template nesting
    warnings.extend(detect_deep_template_nesting(content, file_path));
    
    // Pattern 4: Untrackable dynamic patterns
    warnings.extend(detect_untrackable_patterns(content, file_path));
    
    // Filter by severity threshold
    warnings.into_iter()
        .filter(|w| severity_level(&w.severity) >= severity_level(&config.complexity_threshold))
        .collect()
}

/// Detect dynamic class construction patterns - SIMPLIFIED AND FIXED
fn detect_dynamic_class_construction(content: &str, file_path: &str) -> Vec<ComplexityWarning> {
    let mut warnings = Vec::new();
    
    // Single comprehensive regex for all styles[`...${...}...`] patterns
    let dynamic_styles_regex = Regex::new(r"styles\[\s*`([^`]*)`\s*\]").unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        for capture in dynamic_styles_regex.captures_iter(line) {
            if let Some(template_content) = capture.get(1) {
                let template = template_content.as_str();
                let full_pattern = capture.get(0).unwrap().as_str();
                
                // Skip if it doesn't contain template variables
                if !template.contains("${") {
                    continue;
                }
                
                // Count the number of variables
                let variable_count = template.matches("${").count();
                
                let (warning_type, severity, suggestion) = match variable_count {
                    3.. => (
                        ComplexityWarningType::MultiVariablePattern,
                        WarningSeverity::High,
                        "Multiple variables in template make static analysis very difficult. Consider CSS-in-JS with explicit variants or a class builder function".to_string()
                    ),
                    2 => {
                        // Check if it's the variant_size pattern like ${variant}_${size}
                        if template.matches('_').count() == 1 && template.contains("}_${") {
                            (
                                ComplexityWarningType::MultiVariablePattern,
                                WarningSeverity::Medium,
                                "The ${variable}_${variable} pattern is hard to analyze statically. Consider explicit class mapping: CLASS_MAP[variant][size]".to_string()
                            )
                        } else {
                            (
                                ComplexityWarningType::DynamicClassConstruction,
                                WarningSeverity::Medium,
                                "Multiple variables in template. Consider explicit class mapping for better maintainability".to_string()
                            )
                        }
                    },
                    1 => (
                        ComplexityWarningType::DynamicClassConstruction,
                        WarningSeverity::Low,
                        "Single variable template. Consider using direct class references: styles.specificClassName".to_string()
                    ),
                    _ => continue, // No variables, skip
                };
                
                warnings.push(ComplexityWarning {
                    file_path: file_path.to_string(),
                    line_number: line_num + 1,
                    warning_type,
                    pattern: full_pattern.to_string(),
                    suggestion,
                    severity,
                });
            }
        }
    }
    
    warnings
}

/// Detect complex conditional class assignment
fn detect_conditional_class_assignment(content: &str, file_path: &str) -> Vec<ComplexityWarning> {
    let mut warnings = Vec::new();
    
    // Pattern: Complex ternary operations with styles
    let complex_ternary_regex = Regex::new(
        r"const\s+\w+\s*=\s*[^?]+\?\s*styles\["
    ).unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        if complex_ternary_regex.is_match(line) && line.contains("styles[") {
            warnings.push(ComplexityWarning {
                file_path: file_path.to_string(),
                line_number: line_num + 1,
                warning_type: ComplexityWarningType::ConditionalClassAssignment,
                pattern: line.trim().to_string(),
                suggestion: "Consider using a function to handle conditional class logic: getClassName(condition, variant)".to_string(),
                severity: WarningSeverity::Medium,
            });
        }
    }
    
    warnings
}

/// Detect deeply nested template patterns
fn detect_deep_template_nesting(content: &str, file_path: &str) -> Vec<ComplexityWarning> {
    let mut warnings = Vec::new();
    
    // Pattern: Template literals inside template literals or very long template expressions
    for (line_num, line) in content.lines().enumerate() {
        // Count backticks and template expressions
        let backtick_count = line.matches('`').count();
        let template_expr_count = line.matches("${").count();
        
        // If we have multiple backticks or many template expressions on one line
        if (backtick_count >= 4) || (template_expr_count >= 3 && line.len() > 80) {
            warnings.push(ComplexityWarning {
                file_path: file_path.to_string(),
                line_number: line_num + 1,
                warning_type: ComplexityWarningType::DeepTemplateNesting,
                pattern: line.trim().to_string(),
                suggestion: "Break complex template expressions into separate variables for clarity".to_string(),
                severity: WarningSeverity::Medium,
            });
        }
    }
    
    warnings
}

/// Detect patterns that are essentially untrackable by static analysis
fn detect_untrackable_patterns(content: &str, file_path: &str) -> Vec<ComplexityWarning> {
    let mut warnings = Vec::new();
    
    // Pattern 1: Dynamic property access with computed strings
    let computed_access_regex = Regex::new(r"styles\[\s*[a-zA-Z_][a-zA-Z0-9_]*\s*\+\s*[a-zA-Z_][a-zA-Z0-9_]*\s*\]").unwrap();
    
    // Pattern 2: Function calls that return class names
    let function_call_regex = Regex::new(r"styles\[\s*[a-zA-Z_][a-zA-Z0-9_]*\([^)]*\)\s*\]").unwrap();
    
    for (line_num, line) in content.lines().enumerate() {
        if computed_access_regex.is_match(line) {
            warnings.push(ComplexityWarning {
                file_path: file_path.to_string(),
                line_number: line_num + 1,
                warning_type: ComplexityWarningType::UntrackedDynamicPattern,
                pattern: line.trim().to_string(),
                suggestion: "Dynamic string concatenation makes static analysis impossible. Use template literals or explicit mapping".to_string(),
                severity: WarningSeverity::High,
            });
        }
        
        if function_call_regex.is_match(line) {
            warnings.push(ComplexityWarning {
                file_path: file_path.to_string(),
                line_number: line_num + 1,
                warning_type: ComplexityWarningType::UntrackedDynamicPattern,
                pattern: line.trim().to_string(),
                suggestion: "Function calls in class access make static analysis impossible. Consider explicit class mapping".to_string(),
                severity: WarningSeverity::High,
            });
        }
    }
    
    warnings
}

/// Convert severity to numeric level for comparison
fn severity_level(severity: &WarningSeverity) -> u8 {
    match severity {
        WarningSeverity::Low => 1,
        WarningSeverity::Medium => 2,
        WarningSeverity::High => 3,
    }
}

/// Check if we should process this JavaScript/TypeScript file
fn should_process_js_file(path: &Path, config: &AnalysisConfig) -> bool {
    let is_js_file = path.extension().map_or(false, |ext| {
        matches!(ext.to_str(), Some("js") | Some("jsx") | Some("ts") | Some("tsx"))
    });
    
    if !is_js_file {
        return false;
    }
    
    let path_str = path.to_string_lossy();
    !config.ignore_patterns.iter().any(|pattern| path_str.contains(pattern))
}
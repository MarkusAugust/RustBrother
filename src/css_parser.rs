// src/css_parser.rs - CSS/SCSS parsing logic with fixed nesting
// This module handles parsing CSS and SCSS files to extract class definitions and custom properties

use regex::Regex;
use walkdir::WalkDir;
use std::fs;
use std::path::Path;
use std::collections::HashSet;
use crate::{CssClass, CustomProperty, AnalysisConfig};

/// Find all CSS classes defined in CSS/SCSS files
pub fn find_css_classes(path: &Path, config: &AnalysisConfig) -> anyhow::Result<Vec<CssClass>> {
    let mut classes = Vec::new();
    
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| should_process_css_file(e.path(), config))
    {
        let file_path = entry.path().to_string_lossy().to_string();
        
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_classes = if is_scss_file(entry.path()) {
                parse_scss_file(&content, &file_path)?
            } else {
                parse_css_file(&content, &file_path)?
            };
            classes.extend(file_classes);
        }
    }
    
    Ok(classes)
}

/// Parse SCSS files with proper nesting support
fn parse_scss_file(content: &str, file_path: &str) -> anyhow::Result<Vec<CssClass>> {
    let mut classes = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let expanded_selectors = expand_scss_nesting(&lines);
    
    for (line_number, selector) in expanded_selectors {
        let class_names = extract_classes_from_selector(&selector);
        for class_name in class_names {
            classes.push(CssClass {
                name: class_name,
                file_path: file_path.to_string(),
                line_number: line_number + 1,
            });
        }
    }
    
    Ok(classes)
}

/// Expand SCSS nesting to full selectors - FIXED VERSION
fn expand_scss_nesting(lines: &[&str]) -> Vec<(usize, String)> {
    let mut result = Vec::new();
    let mut nesting_stack = Vec::new();
    let mut indent_stack = Vec::new(); // Track indentation levels
    
    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        let indent_level = line.len() - line.trim_start().len();
        
        if is_comment_or_empty(trimmed) {
            continue;
        }
        
        if is_closing_brace(trimmed) {
            // Pop from both stacks when we see a closing brace
            if !nesting_stack.is_empty() {
                nesting_stack.pop();
            }
            if !indent_stack.is_empty() {
                indent_stack.pop();
            }
            continue;
        }
        
        // Handle indentation changes - pop stack if we've outdented
        while !indent_stack.is_empty() && indent_level <= *indent_stack.last().unwrap() {
            nesting_stack.pop();
            indent_stack.pop();
        }
        
        if let Some(selector) = extract_selector_from_line(trimmed) {
            let full_selector = build_full_selector(&nesting_stack, &selector);
            
            if contains_class_selector(&full_selector) {
                result.push((line_num, full_selector.clone()));
            }
            
            if is_opening_block(trimmed) {
                nesting_stack.push(full_selector);
                indent_stack.push(indent_level);
            }
        }
    }
    
    result
}

/// Build full selector from nesting stack and current selector - FIXED VERSION
fn build_full_selector(stack: &[String], current: &str) -> String {
    if current.starts_with('&') {
        // Handle & reference - preserve exact concatenation
        if let Some(parent) = stack.last() {
            // Simply concatenate parent + current without &
            // e.g., "panel" + "&_outline" = "panel_outline"
            // e.g., "panel_graphic" + "&Icon" = "panel_graphicIcon"
            format!("{}{}", parent, &current[1..])
        } else {
            // If no parent, just remove the &
            current[1..].to_string()
        }
    } else if stack.is_empty() {
        // Root level selector
        current.to_string()
    } else {
        // Nested selector without & - space-separated
        format!("{} {}", stack.join(" "), current)
    }
}

/// Extract selector from a line, handling various SCSS patterns - IMPROVED VERSION
fn extract_selector_from_line(line: &str) -> Option<String> {
    let cleaned = line.split('{').next()?.trim();
    
    // Skip property declarations (contain : but not pseudo-selectors)
    if cleaned.contains(':') && !cleaned.starts_with(':') && !cleaned.contains("::") {
        return None;
    }
    
    // Skip @rules, variables, etc.
    if cleaned.starts_with('@') || cleaned.starts_with('$') {
        return None;
    }
    
    // Skip media queries and other at-rules
    if cleaned.starts_with("@media") || cleaned.starts_with("@supports") {
        return None;
    }
    
    if !cleaned.is_empty() {
        Some(cleaned.to_string())
    } else {
        None
    }
}

/// Parse regular CSS files
fn parse_css_file(content: &str, file_path: &str) -> anyhow::Result<Vec<CssClass>> {
    let mut classes = Vec::new();
    let class_regex = Regex::new(r"\.([a-zA-Z][a-zA-Z0-9_-]*)\s*[{:,]")?;
    
    for (line_num, line) in content.lines().enumerate() {
        if is_comment_or_empty(line) {
            continue;
        }
        
        for capture in class_regex.captures_iter(line) {
            if let Some(class_name) = capture.get(1) {
                classes.push(CssClass {
                    name: class_name.as_str().to_string(),
                    file_path: file_path.to_string(),
                    line_number: line_num + 1,
                });
            }
        }
    }
    
    Ok(classes)
}

/// Find CSS custom properties (CSS variables) in stylesheets
pub fn find_custom_properties(path: &Path, config: &AnalysisConfig) -> anyhow::Result<Vec<CustomProperty>> {
    let mut properties = Vec::new();
    let property_regex = Regex::new(r"(--[a-zA-Z][a-zA-Z0-9_-]*)\s*:\s*([^;]+);")?;
    
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| should_process_css_file(e.path(), config))
    {
        let file_path = entry.path().to_string_lossy().to_string();
        
        if let Ok(content) = fs::read_to_string(entry.path()) {
            for (line_num, line) in content.lines().enumerate() {
                for capture in property_regex.captures_iter(line) {
                    if let (Some(name), Some(value)) = (capture.get(1), capture.get(2)) {
                        properties.push(CustomProperty {
                            name: name.as_str().to_string(),
                            value: value.as_str().trim().to_string(),
                            file_path: file_path.to_string(),
                            line_number: line_num + 1,
                        });
                    }
                }
            }
        }
    }
    
    Ok(properties)
}

/// Find where CSS custom properties are used with var() function
pub fn find_custom_property_usage(path: &Path, config: &AnalysisConfig) -> anyhow::Result<HashSet<String>> {
    let mut used_properties = HashSet::new();
    let var_regex = Regex::new(r"var\(\s*(--[a-zA-Z][a-zA-Z0-9_-]*)\s*\)")?;
    
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| should_process_css_file(e.path(), config))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            for capture in var_regex.captures_iter(&content) {
                if let Some(property_name) = capture.get(1) {
                    used_properties.insert(property_name.as_str().to_string());
                }
            }
        }
    }
    
    Ok(used_properties)
}

/// Extract class names from a CSS selector
fn extract_classes_from_selector(selector: &str) -> Vec<String> {
    let class_regex = Regex::new(r"\.([a-zA-Z][a-zA-Z0-9_-]*)").unwrap();
    class_regex
        .captures_iter(selector)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .collect()
}

// Helper functions
fn is_scss_file(path: &Path) -> bool {
    path.extension().map_or(false, |ext| ext == "scss" || ext == "sass")
}

fn is_comment_or_empty(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*")
}

fn is_closing_brace(line: &str) -> bool {
    line.trim() == "}"
}

fn is_opening_block(line: &str) -> bool {
    line.contains('{')
}

fn contains_class_selector(selector: &str) -> bool {
    selector.contains('.')
}

fn should_process_css_file(path: &Path, config: &AnalysisConfig) -> bool {
    let is_css_file = path.extension().map_or(false, |ext| {
        matches!(ext.to_str(), Some("css") | Some("scss") | Some("sass"))
    });
    
    if !is_css_file {
        return false;
    }
    
    let path_str = path.to_string_lossy();
    !config.ignore_patterns.iter().any(|pattern| path_str.contains(pattern))
}
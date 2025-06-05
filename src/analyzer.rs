// src/analyzer.rs - Main analysis logic
// This module contains the core logic to compare CSS definitions with usage

use std::collections::HashSet;
use std::path::Path;
use crate::{AnalysisResult, CssClass, CustomProperty, AnalysisConfig};
use crate::css_parser::{find_css_classes, find_custom_properties, find_custom_property_usage};
use crate::js_parser::find_js_css_references_with_context;
use crate::complexity_analyzer::find_complexity_warnings; // NEW!

/// Main function that analyzes a directory and returns results
pub fn analyze_directory(path: &Path, config: &AnalysisConfig) -> anyhow::Result<AnalysisResult> {
    // Step 1: Find all CSS classes defined in stylesheets
    println!("🔍 Scanning CSS files for class definitions...");
    let css_classes = find_css_classes(path, config)?;
    
    // Step 2: Extract CSS class names for context-aware JS parsing
    let css_class_names: Vec<String> = css_classes.iter().map(|c| c.name.clone()).collect();
    
    // Step 3: Find all CSS class references in JavaScript/React files (with CSS context)
    println!("🔍 Scanning JS/React files for class usage...");
    let js_references = find_js_css_references_with_context(path, config, &css_class_names)?;
    
    // Step 4: Find CSS custom properties
    println!("🔍 Scanning for CSS custom properties...");
    let custom_properties = find_custom_properties(path, config)?;
    
    // Step 5: Find custom property usage
    println!("🔍 Analyzing custom property usage...");
    let used_property_names = find_custom_property_usage(path, config)?;
    
    // Step 6: NEW! Find complexity warnings
    println!("🔍 Analyzing code complexity patterns...");
    let complexity_warnings = find_complexity_warnings(path, config)?;
    
    // Step 7: Analyze usage patterns
    println!("📊 Analyzing usage patterns...");
    let analysis = analyze_css_usage(
        css_classes, 
        js_references, 
        custom_properties, 
        used_property_names, 
        complexity_warnings // NEW!
    )?;
    
    println!("✅ Analysis complete!");
    Ok(analysis)
}

/// Updated analysis function to include complexity warnings
fn analyze_css_usage(
    css_classes: Vec<CssClass>,
    js_references: Vec<String>,
    custom_properties: Vec<CustomProperty>,
    used_property_names: HashSet<String>,
    complexity_warnings: Vec<crate::ComplexityWarning>, // NEW!
) -> anyhow::Result<AnalysisResult> {
    
    let js_class_set: HashSet<String> = js_references.into_iter().collect();
    
    let (used_classes, unused_classes): (Vec<CssClass>, Vec<CssClass>) = css_classes
        .into_iter()
        .partition(|css_class| {
            js_class_set.contains(&css_class.name)
        });
    
    let (used_custom_properties, unused_custom_properties): (Vec<CustomProperty>, Vec<CustomProperty>) = 
        custom_properties
            .into_iter()
            .partition(|property| {
                used_property_names.contains(&property.name)
            });
    
    let total_css_files = count_unique_css_files(&used_classes, &unused_classes);
    let total_js_files = count_unique_js_files(&js_class_set);
    let total_files_scanned = total_css_files + total_js_files;
    
    Ok(AnalysisResult {
        used_classes,
        unused_classes,
        used_custom_properties,
        unused_custom_properties,
        complexity_warnings, // NEW!
        total_files_scanned,
        total_css_files,
        total_js_files,
    })
}

/// Count the number of unique CSS files that were processed
/// This is used for statistics in the final report
fn count_unique_css_files(used_classes: &[CssClass], unused_classes: &[CssClass]) -> usize {
    let mut unique_files = HashSet::new();
    
    // Add file paths from used classes
    for class in used_classes {
        unique_files.insert(&class.file_path);
    }
    
    // Add file paths from unused classes
    for class in unused_classes {
        unique_files.insert(&class.file_path);
    }
    
    unique_files.len()
}

/// Count the number of unique JS files (estimated from class references)
fn count_unique_js_files(_js_class_set: &HashSet<String>) -> usize {
    // For now, we'll return 0 since we don't track which files the references came from
    // This could be improved to track source files in the parser
    0
}

/// Advanced analysis functions (for future phases)
/// These are placeholder functions that we can implement later

/// Detect CSS classes that are always overridden
/// This would look for patterns where a class is defined but always overridden by more specific selectors
#[allow(dead_code)]
fn find_always_overridden_classes(_css_classes: &[CssClass]) -> Vec<CssClass> {
    // TODO: Implement in Phase 2
    // This would analyze CSS specificity and find classes that are never actually applied
    Vec::new()
}

/// Detect excessive use of !important
/// This would flag CSS rules that use !important too frequently
#[allow(dead_code)]
fn find_excessive_important_usage(_path: &Path) -> anyhow::Result<Vec<String>> {
    // TODO: Implement in Phase 2
    // This would scan CSS files and count !important usage per file
    Ok(Vec::new())
}
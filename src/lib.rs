// src/lib.rs - complexity analysis module and types

use serde::{Deserialize, Serialize};

// Add the new complexity analyzer module
pub mod css_parser;
pub mod js_parser;
pub mod analyzer;
pub mod reporter;
pub mod complexity_analyzer; 

// Re-export functions including complexity analysis
pub use analyzer::analyze_directory;
pub use css_parser::{find_css_classes, find_custom_properties, find_custom_property_usage};
pub use js_parser::{find_js_css_references, find_js_css_references_with_context, extract_css_references, extract_css_references_with_css_context};
pub use reporter::generate_report;
pub use complexity_analyzer::find_complexity_warnings; 

/// Represents a single CSS class found in a stylesheet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssClass {
    pub name: String,
    pub file_path: String,
    pub line_number: usize,
}

/// Represents a CSS custom property (CSS variable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomProperty {
    pub name: String,
    pub value: String,
    pub file_path: String,
    pub line_number: usize,
}

/// NEW: Represents a complexity warning found in the codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityWarning {
    pub file_path: String,
    pub line_number: usize,
    pub warning_type: ComplexityWarningType,
    pub pattern: String,
    pub suggestion: String,
    pub severity: WarningSeverity,
}

/// NEW: Types of complexity warnings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityWarningType {
    DynamicClassConstruction,
    DeepTemplateNesting,
    ConditionalClassAssignment,
    MultiVariablePattern,
    UntrackedDynamicPattern,
}

/// NEW: Severity levels for warnings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarningSeverity {
    Low,    // Minor complexity, easy to refactor
    Medium, // Moderate complexity, consider refactoring
    High,   // High complexity, should refactor for maintainability
}

/// Analysis results to include complexity warnings
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub used_classes: Vec<CssClass>,
    pub unused_classes: Vec<CssClass>,
    pub used_custom_properties: Vec<CustomProperty>,
    pub unused_custom_properties: Vec<CustomProperty>,
    pub complexity_warnings: Vec<ComplexityWarning>, 
    pub total_files_scanned: usize,
    pub total_css_files: usize,
    pub total_js_files: usize,
}

/// Cconfiguration to include complexity analysis options
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    pub include_css_modules: bool,
    pub include_styled_components: bool,
    pub ignore_patterns: Vec<String>,
    pub enable_complexity_warnings: bool, 
    pub complexity_threshold: WarningSeverity, 
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            include_css_modules: true,
            include_styled_components: false,
            ignore_patterns: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "dist".to_string(),
                "build".to_string(),
            ],
            enable_complexity_warnings: true, // Enable by default
            complexity_threshold: WarningSeverity::Medium, // Show medium and high by default
        }
    }
}
// src/js_parser.rs - JavaScript/React parsing logic
// Enhanced to detect dynamic class name patterns

use regex::Regex;
use walkdir::WalkDir;
use std::fs;
use std::path::Path;
use std::collections::HashSet;
use crate::AnalysisConfig;

/// Find all CSS class references in JavaScript/TypeScript/React files
pub fn find_js_css_references(path: &Path, config: &AnalysisConfig) -> anyhow::Result<Vec<String>> {
    let mut references = HashSet::new();
    
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| should_process_js_file(e.path(), config))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_references = extract_css_references(&content, config);
            references.extend(file_references);
        }
    }
    
    // Convert to sorted Vec and remove duplicates
    let mut sorted_refs: Vec<String> = references.into_iter().collect();
    sorted_refs.sort();
    Ok(sorted_refs)
}

/// NEW: Find all CSS class references with known CSS classes for context
/// This version uses actual CSS class definitions to generate smarter dynamic variants
pub fn find_js_css_references_with_context(path: &Path, config: &AnalysisConfig, known_css_classes: &[String]) -> anyhow::Result<Vec<String>> {
    let mut references = HashSet::new();
    
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| should_process_js_file(e.path(), config))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            let file_references = extract_css_references_with_css_context(&content, config, known_css_classes);
            references.extend(file_references);
        }
    }
    
    // Convert to sorted Vec and remove duplicates
    let mut sorted_refs: Vec<String> = references.into_iter().collect();
    sorted_refs.sort();
    Ok(sorted_refs)
}

/// Extract CSS class references from JavaScript content
pub fn extract_css_references(content: &str, config: &AnalysisConfig) -> Vec<String> {
    let mut references = HashSet::new(); // Use HashSet to automatically handle duplicates
    
    // Pattern 1: className="class1 class2"
    references.extend(extract_simple_classnames(content));
    
    // Pattern 2: className={'class1 class2'}
    references.extend(extract_object_classnames(content));
    
    // Pattern 3: CSS modules (if enabled)
    if config.include_css_modules {
        references.extend(extract_css_modules_references(content));
        
        // Pattern 4: Template literal patterns
        references.extend(extract_template_literal_classes(content));
    }
    
    // Pattern 5: styled-components (if enabled)
    if config.include_styled_components {
        references.extend(extract_styled_components_references(content));
    }
    
    // Convert to Vec and sort
    let mut result: Vec<String> = references.into_iter().collect();
    result.sort();
    result
}

/// Extract CSS class references from JavaScript content with known CSS classes
/// This version only returns classes that exist in the known_css_classes list
pub fn extract_css_references_with_css_context(content: &str, config: &AnalysisConfig, known_css_classes: &[String]) -> Vec<String> {
    // Convert to HashSet for faster lookups
    let known_classes_set: HashSet<String> = known_css_classes.iter().cloned().collect();
    let mut references = HashSet::new();
    
    // Pattern 1: className="class1 class2"
    let simple_classes = extract_simple_classnames(content);
    for class in simple_classes {
        if known_classes_set.contains(&class) {
            references.insert(class);
        }
    }
    
    // Pattern 2: className={'class1 class2'}
    let object_classes = extract_object_classnames(content);
    for class in object_classes {
        if known_classes_set.contains(&class) {
            references.insert(class);
        }
    }
    
    // Pattern 3: CSS modules (if enabled)
    if config.include_css_modules {
        let css_module_classes = extract_css_modules_references(content);
        for class in css_module_classes {
            if known_classes_set.contains(&class) {
                references.insert(class);
            }
        }
        
        // Pattern 4: Dynamic CSS modules with CSS context
        let dynamic_classes = extract_dynamic_css_modules_with_context(content, known_css_classes);
        for class in dynamic_classes {
            if known_classes_set.contains(&class) {
                references.insert(class);
            }
        }
        
        // Pattern 5: Variable assignment with CSS context
        let variable_classes = extract_variable_assignment_patterns_with_context(content, known_css_classes);
        for class in variable_classes {
            if known_classes_set.contains(&class) {
                references.insert(class);
            }
        }
        
        // Pattern 6: Template literal patterns
        let template_classes = extract_template_literal_classes(content);
        for class in template_classes {
            if known_classes_set.contains(&class) {
                references.insert(class);
            }
        }
    }
    
    // Pattern 7: styled-components (if enabled)
    if config.include_styled_components {
        let styled_classes = extract_styled_components_references(content);
        for class in styled_classes {
            if known_classes_set.contains(&class) {
                references.insert(class);
            }
        }
    }
    
    // Convert to Vec and sort
    let mut result: Vec<String> = references.into_iter().collect();
    result.sort();
    result
}

/// Extract simple className patterns: className="class1 class2"
fn extract_simple_classnames(content: &str) -> Vec<String> {
    let mut classes = Vec::new();
    let regex = Regex::new(r#"className\s*=\s*["'`]([^"'`]+)["'`]"#).unwrap();
    
    for capture in regex.captures_iter(content) {
        if let Some(classes_str) = capture.get(1) {
            classes.extend(split_class_string(classes_str.as_str()));
        }
    }
    
    classes
}

/// Extract object className patterns: className={'class1 class2'}
fn extract_object_classnames(content: &str) -> Vec<String> {
    let mut classes = Vec::new();
    let regex = Regex::new(r#"className\s*=\s*\{\s*['"`]([^'"`]+)['"`]\s*\}"#).unwrap();
    
    for capture in regex.captures_iter(content) {
        if let Some(classes_str) = capture.get(1) {
            classes.extend(split_class_string(classes_str.as_str()));
        }
    }
    
    classes
}

/// Extract CSS modules references (styles.className)
fn extract_css_modules_references(content: &str) -> Vec<String> {
    let mut classes = Vec::new();
    
    // Direct usage: styles.className
    let direct_regex = Regex::new(r"styles\.([a-zA-Z][a-zA-Z0-9_-]*)").unwrap();
    for capture in direct_regex.captures_iter(content) {
        if let Some(class_name) = capture.get(1) {
            let class_str = class_name.as_str().to_string();
            // Filter out template literal variables
            if !class_str.starts_with("${") && !class_str.ends_with("}") {
                classes.push(class_str);
            }
        }
    }
    
    // Template literals: ${styles.className}
    let template_regex = Regex::new(r"\$\{styles\.([a-zA-Z][a-zA-Z0-9_-]*)\}").unwrap();
    for capture in template_regex.captures_iter(content) {
        if let Some(class_name) = capture.get(1) {
            classes.push(class_name.as_str().to_string());
        }
    }
    
    // Object destructuring: const { className } = styles
    let destructure_regex = Regex::new(r"const\s*\{\s*([^}]+)\s*\}\s*=\s*styles").unwrap();
    for capture in destructure_regex.captures_iter(content) {
        if let Some(destructured) = capture.get(1) {
            let class_names = extract_destructured_class_names(destructured.as_str());
            classes.extend(class_names);
        }
    }
    
    // NEW: Variable assignment patterns
    // const varName = styles[`template`] followed by usage in className
    classes.extend(extract_variable_assignment_patterns(content));
    
    classes
}

/// NEW: Extract template literal class patterns
/// Handles: `${styles.button} ${variantClass}`
fn extract_template_literal_classes(content: &str) -> Vec<String> {
    let mut classes = Vec::new();
    
    // Look for template literals that contain styles references
    let template_regex = Regex::new(r"`[^`]*\$\{[^}]*styles\.[^}]+\}[^`]*`").unwrap();
    
    for template_match in template_regex.find_iter(content) {
        let template_content = template_match.as_str();
        
        // Extract direct styles.className references within the template
        let styles_regex = Regex::new(r"styles\.([a-zA-Z][a-zA-Z0-9_-]*)").unwrap();
        for capture in styles_regex.captures_iter(template_content) {
            if let Some(class_name) = capture.get(1) {
                classes.push(class_name.as_str().to_string());
            }
        }
    }
    
    classes
}

/// Extract styled-components references (for future implementation)
fn extract_styled_components_references(_content: &str) -> Vec<String> {
    // TODO: Implement styled-components parsing
    // Look for styled.div`...` patterns and extract class usage
    Vec::new()
}

/// Extract class names from destructuring assignment
fn extract_destructured_class_names(destructured: &str) -> Vec<String> {
    destructured
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter(|s| is_valid_class_name(s))
        .map(|s| s.to_string())
        .collect()
}

/// NEW: Extract patterns where styles[template] is assigned to a variable
/// then that variable is used in className
/// Example: const colorClassName = color !== 'none' ? styles[`accordion_${color}`] : '';
fn extract_variable_assignment_patterns(content: &str) -> Vec<String> {
    let mut classes = HashSet::new();
    
    // Pattern 1: const varName = condition ? styles[`template`] : '';
    // This captures: const colorClassName = color !== 'none' ? styles[`accordion_${color}`] : '';
    let conditional_assignment_regex = Regex::new(
        r"const\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*[^?]*\?\s*styles\[\s*`([^`]*)\$\{[^}]+\}([^`]*)`\s*\]\s*:"
    ).unwrap();
    
    for capture in conditional_assignment_regex.captures_iter(content) {
        let var_name = capture.get(1).map_or("", |m| m.as_str());
        let prefix = capture.get(2).map_or("", |m| m.as_str());
        let suffix = capture.get(3).map_or("", |m| m.as_str());
        
        // Check if this variable is used in a className
        if is_variable_used_in_classname(content, var_name) {
            // Generate variants based on prefix
            let variants = generate_variants_for_prefix(prefix);
            
            for variant in &variants {
                let class_name = format!("{}{}{}", prefix, variant, suffix);
                if !class_name.is_empty() && is_valid_class_name(&class_name) {
                    classes.insert(class_name);
                }
            }
        }
    }
    
    // Pattern 2: Direct assignment: const varName = styles[`template`];
    let direct_assignment_regex = Regex::new(
        r"const\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*styles\[\s*`([^`]*)\$\{[^}]+\}([^`]*)`\s*\]"
    ).unwrap();
    
    for capture in direct_assignment_regex.captures_iter(content) {
        let var_name = capture.get(1).map_or("", |m| m.as_str());
        let prefix = capture.get(2).map_or("", |m| m.as_str());
        let suffix = capture.get(3).map_or("", |m| m.as_str());
        
        // Check if this variable is used in a className
        if is_variable_used_in_classname(content, var_name) {
            let variants = generate_variants_for_prefix(prefix);
            
            for variant in &variants {
                let class_name = format!("{}{}{}", prefix, variant, suffix);
                if !class_name.is_empty() && is_valid_class_name(&class_name) {
                    classes.insert(class_name);
                }
            }
        }
    }
    
    classes.into_iter().collect()
}

/// Check if a variable is used in any className assignment
fn is_variable_used_in_classname(content: &str, var_name: &str) -> bool {
    // Look for patterns like: className={...varName...} or className={`...${varName}...`}
    let usage_patterns = vec![
        format!(r"className\s*=\s*\{{[^}}]*\b{}\b[^}}]*\}}", var_name),
        format!(r"className\s*=\s*`[^`]*\${{\s*{}\s*\}}[^`]*`", var_name),
        format!(r"className\s*=\s*\{{`[^`]*\${{\s*{}\s*\}}[^`]*`\}}", var_name),
    ];
    
    for pattern in usage_patterns {
        if let Ok(regex) = Regex::new(&pattern) {
            if regex.is_match(content) {
                return true;
            }
        }
    }
    
    false
}

/// Generate common variants based on the prefix pattern
fn generate_variants_for_prefix(prefix: &str) -> Vec<&'static str> {
    if prefix.contains("button") {
        vec!["filled", "outline", "primary", "secondary", "success", "warning", "error", "small", "medium", "large"]
    } else if prefix.contains("accordion") {
        vec!["forest", "denim", "ochre", "burgundy", "graphite"]
    } else if prefix.contains("card") || prefix.contains("panel") {
        vec!["forest", "denim", "ochre", "burgundy", "graphite", "primary", "secondary", "dark", "light"]
    } else if prefix.contains("badge") || prefix.contains("tag") {
        vec!["success", "warning", "error", "info", "primary", "secondary"]
    } else if prefix.contains("text") || prefix.contains("font") {
        vec!["small", "medium", "large", "xs", "sm", "md", "lg", "xl", "primary", "secondary", "muted"]
    } else {
        // Default variants for unknown patterns
        vec![
            "filled", "outline", "primary", "secondary", "success", "warning", "error",
            "forest", "denim", "ochre", "burgundy", "graphite",
            "small", "medium", "large", "xs", "sm", "md", "lg", "xl"
        ]
    }
}

fn extract_dynamic_css_modules_with_context(content: &str, known_css_classes: &[String]) -> Vec<String> {
    let mut classes = HashSet::new();
    
    // Pattern 1: styles[`prefix_${variable}`] or styles[`prefix_${variable}_suffix`]
    // Example: styles[`button_${variant}`] or styles[`card_${color}_outline`]
    let template_bracket_regex = Regex::new(r"styles\[\s*`([^`$]*)\$\{[^}]+\}([^`]*)`\s*\]").unwrap();
    for capture in template_bracket_regex.captures_iter(content) {
        let prefix = capture.get(1).map_or("", |m| m.as_str());
        let suffix = capture.get(2).map_or("", |m| m.as_str());
        
        // Only process if we have at least a prefix or suffix (not pure variable)
        if !prefix.is_empty() || !suffix.is_empty() {
            let matching_classes = find_matching_css_classes(known_css_classes, prefix, suffix);
            classes.extend(matching_classes);
        }
    }
    
    // Pattern 2: styles[`${variable1}_${variable2}`] - pure variables with underscore
    // Example: styles[`${variant}_${size}`] -> "systemIcon_small", "themeIcon_large"
    let double_variable_regex = Regex::new(r"styles\[\s*`\$\{[^}]+\}_\$\{[^}]+\}`\s*\]").unwrap();
    if double_variable_regex.is_match(content) {
        let variant_size_classes = find_variant_size_classes(known_css_classes);
        classes.extend(variant_size_classes);
    }
    
    // Pattern 3: styles[`${variable1}${variable2}`] - pure variables concatenated
    // Example: styles[`${prefix}${suffix}`] 
    let concat_variable_regex = Regex::new(r"styles\[\s*`\$\{[^}]+\}\$\{[^}]+\}`\s*\]").unwrap();
    if concat_variable_regex.is_match(content) {
        // This is harder to predict, but we can try common concatenation patterns
        let concat_classes = find_concatenation_classes(known_css_classes);
        classes.extend(concat_classes);
    }
    
    // Pattern 4: styles[`${variable}_literal`] or styles[`literal_${variable}`]
    // Example: styles[`${variant}_outline`] or styles[`button_${size}`]
    let mixed_literal_regex = Regex::new(r"styles\[\s*`(?:([^`$]+)\$\{[^}]+\}|(\$\{[^}]+\})([^`$]+))`\s*\]").unwrap();
    for capture in mixed_literal_regex.captures_iter(content) {
        if let Some(prefix) = capture.get(1) {
            // Pattern: literal_${variable}
            let matching_classes = find_css_classes_with_prefix(known_css_classes, prefix.as_str());
            classes.extend(matching_classes);
        } else if let Some(suffix) = capture.get(3) {
            // Pattern: ${variable}_literal
            let matching_classes = find_css_classes_with_suffix(known_css_classes, suffix.as_str());
            classes.extend(matching_classes);
        }
    }
    
    // Pattern 5: styles['prefix_' + variable] - old-style concatenation
    let concat_regex = Regex::new(r#"styles\[.*['"`]([a-zA-Z_][a-zA-Z0-9_-]*)['"`].*\+.*\]"#).unwrap();
    for capture in concat_regex.captures_iter(content) {
        if let Some(prefix_match) = capture.get(1) {
            let prefix_str = prefix_match.as_str();
            let matching_classes = find_css_classes_with_prefix(known_css_classes, prefix_str);
            classes.extend(matching_classes);
        }
    }
    
    // Pattern 6: Complex template literals with multiple variables
    // Example: styles[`${base}_${variant}_${size}`]
    let multi_variable_regex = Regex::new(r"styles\[\s*`[^`]*\$\{[^}]+\}[^`]*\$\{[^}]+\}[^`]*\$\{[^}]+\}[^`]*`\s*\]").unwrap();
    if multi_variable_regex.is_match(content) {
        let multi_var_classes = find_multi_variable_classes(known_css_classes);
        classes.extend(multi_var_classes);
    }
    
    classes.into_iter().collect()
}

/// NEW: Extract variable assignment patterns using actual CSS class definitions
fn extract_variable_assignment_patterns_with_context(content: &str, known_css_classes: &[String]) -> Vec<String> {
    let mut classes = HashSet::new();
    
    // Pattern 1: const varName = condition ? styles[`template`] : '';
    let conditional_assignment_regex = Regex::new(
        r"const\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*[^?]*\?\s*styles\[\s*`([^`]*)\$\{[^}]+\}([^`]*)`\s*\]\s*:"
    ).unwrap();
    
    for capture in conditional_assignment_regex.captures_iter(content) {
        let var_name = capture.get(1).map_or("", |m| m.as_str());
        let prefix = capture.get(2).map_or("", |m| m.as_str());
        let suffix = capture.get(3).map_or("", |m| m.as_str());
        
        // Check if this variable is used in a className
        if is_variable_used_in_classname(content, var_name) {
            // Find all CSS classes that match this pattern
            let matching_classes = find_matching_css_classes(known_css_classes, prefix, suffix);
            classes.extend(matching_classes);
        }
    }
    
    // Pattern 2: Direct assignment: const varName = styles[`template`];
    let direct_assignment_regex = Regex::new(
        r"const\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*styles\[\s*`([^`]*)\$\{[^}]+\}([^`]*)`\s*\]"
    ).unwrap();
    
    for capture in direct_assignment_regex.captures_iter(content) {
        let var_name = capture.get(1).map_or("", |m| m.as_str());
        let prefix = capture.get(2).map_or("", |m| m.as_str());
        let suffix = capture.get(3).map_or("", |m| m.as_str());
        
        // Check if this variable is used in a className
        if is_variable_used_in_classname(content, var_name) {
            let matching_classes = find_matching_css_classes(known_css_classes, prefix, suffix);
            classes.extend(matching_classes);
        }
    }
    
    classes.into_iter().collect()
}



/// Find CSS classes that start with a given prefix
fn find_css_classes_with_prefix(known_css_classes: &[String], prefix: &str) -> Vec<String> {
    known_css_classes
        .iter()
        .filter(|class_name| class_name.starts_with(prefix) && class_name.len() > prefix.len())
        .cloned()
        .collect()
}

/// Check if a string is a valid CSS class name
fn is_valid_class_name(name: &str) -> bool {
    // Filter out template literal variables and other invalid patterns
    if name.starts_with("${") || name.ends_with("}") || name.contains("${") {
        return false;
    }
    
    let class_regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();
    class_regex.is_match(name)
}

/// Split class string by whitespace and filter empty strings
fn split_class_string(classes_str: &str) -> Vec<String> {
    classes_str
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
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

/// Find classes that match variant_size pattern (like systemIcon_small, themeIcon_large)
/// This handles the ${variant}_${size} pattern specifically
fn find_variant_size_classes(known_css_classes: &[String]) -> Vec<String> {
    let mut matches = Vec::new();
    
    // Look for any class that has exactly one underscore and matches known patterns
    for class_name in known_css_classes {
        if class_name.matches('_').count() == 1 {
            let parts: Vec<&str> = class_name.split('_').collect();
            if parts.len() == 2 {
                let potential_variant = parts[0];
                let potential_size = parts[1];
                
                // Check if this looks like a variant_size pattern
                if is_likely_variant(potential_variant) && is_likely_size(potential_size) {
                    matches.push(class_name.clone());
                }
            }
        }
    }
    
    matches
}

/// Find classes that could be concatenations of variables
fn find_concatenation_classes(known_css_classes: &[String]) -> Vec<String> {
    let mut matches = Vec::new();
    
    // Look for classes that could be formed by concatenating common prefixes and suffixes
    let common_prefixes = ["button", "card", "panel", "icon", "text", "bg", "border"];
    let common_suffixes = ["Small", "Medium", "Large", "Primary", "Secondary", "Success", "Error", "Warning", "Info"];
    
    for class_name in known_css_classes {
        for prefix in &common_prefixes {
            for suffix in &common_suffixes {
                let expected = format!("{}{}", prefix, suffix);
                if class_name == &expected {
                    matches.push(class_name.clone());
                }
            }
        }
        
        // Also check for camelCase concatenations
        if is_camel_case_concatenation(class_name) {
            matches.push(class_name.clone());
        }
    }
    
    matches
}

/// Find classes that end with a specific suffix
fn find_css_classes_with_suffix(known_css_classes: &[String], suffix: &str) -> Vec<String> {
    known_css_classes
        .iter()
        .filter(|class_name| class_name.ends_with(suffix) && class_name.len() > suffix.len())
        .cloned()
        .collect()
}

/// Find classes that could be formed by multiple variables
fn find_multi_variable_classes(known_css_classes: &[String]) -> Vec<String> {
    let mut matches = Vec::new();
    
    // Look for classes with multiple underscores or complex patterns
    for class_name in known_css_classes {
        let underscore_count = class_name.matches('_').count();
        
        // Classes with 2+ underscores might be multi-variable patterns
        if underscore_count >= 2 {
            matches.push(class_name.clone());
        }
        
        // Also check for mixed camelCase and underscore patterns
        if underscore_count >= 1 && has_mixed_case_pattern(class_name) {
            matches.push(class_name.clone());
        }
    }
    
    matches
}

/// Check if a string looks like a CSS variant name
fn is_likely_variant(s: &str) -> bool {
    let common_variants = [
        "systemIcon", "themeIcon", "button", "card", "panel", "badge", "tag", 
        "input", "select", "checkbox", "radio", "text", "heading", "link"
    ];
    
    common_variants.contains(&s) || s.ends_with("Icon") || s.ends_with("Button") || s.ends_with("Card")
}

/// Check if a string looks like a CSS size name
fn is_likely_size(s: &str) -> bool {
    let common_sizes = [
        "small", "medium", "large", "extraLarge", "xs", "sm", "md", "lg", "xl", "xxl",
        "mini", "tiny", "huge", "massive"
    ];
    
    common_sizes.contains(&s)
}

/// Check if a class name looks like a camelCase concatenation
fn is_camel_case_concatenation(class_name: &str) -> bool {
    // Look for patterns like buttonPrimary, cardLarge, etc.
    let has_lowercase_start = class_name.chars().next().map_or(false, |c| c.is_lowercase());
    let has_uppercase_middle = class_name.chars().skip(1).any(|c| c.is_uppercase());
    
    has_lowercase_start && has_uppercase_middle && class_name.chars().all(|c| c.is_alphanumeric())
}

/// Check if a class name has mixed case patterns (camelCase + underscores)
fn has_mixed_case_pattern(class_name: &str) -> bool {
    class_name.contains('_') && class_name.chars().any(|c| c.is_uppercase())
}

/// Enhanced version of find_matching_css_classes with better pattern matching
fn find_matching_css_classes(known_css_classes: &[String], prefix: &str, suffix: &str) -> Vec<String> {
    let mut matches = Vec::new();
    
    for class_name in known_css_classes {
        if class_name.starts_with(prefix) && class_name.ends_with(suffix) {
            // Make sure it's not just the prefix + suffix (empty middle part)
            let middle_part = &class_name[prefix.len()..class_name.len() - suffix.len()];
            if !middle_part.is_empty() && is_valid_class_name(class_name) {
                matches.push(class_name.clone());
            }
        }
    }
    
    // If we didn't find exact matches and the prefix/suffix pattern looks generic,
    // try fuzzy matching
    if matches.is_empty() && (prefix.is_empty() || suffix.is_empty()) {
        let pattern = format!("{}{}", prefix, suffix);
        for class_name in known_css_classes {
            if class_name.contains(&pattern) && is_valid_class_name(class_name) {
                matches.push(class_name.clone());
            }
        }
    }
    
    matches
}
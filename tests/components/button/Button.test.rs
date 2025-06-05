use rustbrother::{analyze_directory, AnalysisConfig};
use std::path::Path;

#[test]
fn test_button_component_full_analysis() {
    let test_path = Path::new("tests/components/button");
    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
        ..Default::default()
    };

    // Run full analysis on the button component directory
    let result = analyze_directory(test_path, &config).unwrap();
    
    println!("📊 Analysis Results:");
    println!("  Total CSS files: {}", result.total_css_files);
    println!("  Total JS files: {}", result.total_js_files);
    println!("  Used classes: {}", result.used_classes.len());
    println!("  Unused classes: {}", result.unused_classes.len());
    
    // Print the actual classes
    println!("✅ Used classes:");
    for class in &result.used_classes {
        println!("  - {} ({}:{})", class.name, class.file_path, class.line_number);
    }
    
    println!("🚫 Unused classes:");
    for class in &result.unused_classes {
        println!("  - {} ({}:{})", class.name, class.file_path, class.line_number);
    }
    
    // Test that iconLarge is in the unused classes
    let unused_class_names: Vec<String> = result.unused_classes.iter().map(|c| c.name.clone()).collect();
    assert!(unused_class_names.contains(&"iconLarge".to_string()), 
        "iconLarge should be detected as unused");
    
    // Test that button is in the used classes
    let used_class_names: Vec<String> = result.used_classes.iter().map(|c| c.name.clone()).collect();
    assert!(used_class_names.contains(&"button".to_string()), 
        "button should be detected as used");
    
    println!("✅ Full analysis test passed!");
}
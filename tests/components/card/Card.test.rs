use rustbrother::{analyze_directory, AnalysisConfig};
use std::path::Path;

#[test]
fn test_card_component_full_analysis() {
    let test_path = Path::new("tests/components/card");
    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
        ..Default::default()
    };

    // Run full analysis on the card component directory
    let result = analyze_directory(test_path, &config).unwrap();
    
    println!("📊 Card Analysis Results:");
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
    
    // Test that base classes are used
    let used_class_names: Vec<String> = result.used_classes.iter().map(|c| c.name.clone()).collect();
    assert!(used_class_names.contains(&"card".to_string()), "card should be detected as used");
    assert!(used_class_names.contains(&"cardHeader".to_string()), "cardHeader should be detected as used");
    assert!(used_class_names.contains(&"cardBody".to_string()), "cardBody should be detected as used");
    assert!(used_class_names.contains(&"cardFooter".to_string()), "cardFooter should be detected as used");
    
    // Test dynamic theme variants
    assert!(used_class_names.contains(&"card_theme_dark".to_string()), "card_theme_dark should be detected as used");
    assert!(used_class_names.contains(&"card_theme_light".to_string()), "card_theme_light should be detected as used");
    assert!(used_class_names.contains(&"card_theme_colorful".to_string()), "card_theme_colorful should be detected as used");
    
    // Test dynamic elevation variants
    assert!(used_class_names.contains(&"card_elevation_low".to_string()), "card_elevation_low should be detected as used");
    assert!(used_class_names.contains(&"card_elevation_medium".to_string()), "card_elevation_medium should be detected as used");
    assert!(used_class_names.contains(&"card_elevation_high".to_string()), "card_elevation_high should be detected as used");
    
    println!("✅ Card full analysis test passed!");
}
// tests/components/button/Button.test.rs

use rustbrother::{extract_css_references_with_css_context, AnalysisConfig};
use std::fs;

#[test]
fn test_button_component() {
    let component_content = fs::read_to_string("tests/components/button/Button.tsx").unwrap();
    
    let css_classes = vec![
        "button".to_string(),
        "button_primary".to_string(),
        "button_secondary".to_string(),
        "button_danger".to_string(),
        "button_small".to_string(),
        "button_medium".to_string(),
        "button_large".to_string(),
        "button_disabled".to_string(),
        "buttonText".to_string(),
        "iconWrapper".to_string(),
        "icon".to_string(),
    ];

    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
        ..Default::default()
    };

    let classes = extract_css_references_with_css_context(&component_content, &config, &css_classes);
    
    // Base classes
    assert!(classes.contains(&"button".to_string()));
    assert!(classes.contains(&"buttonText".to_string()));
    assert!(classes.contains(&"iconWrapper".to_string()));
    assert!(classes.contains(&"icon".to_string()));
    
    // Dynamic variants
    assert!(classes.contains(&"button_primary".to_string()));
    assert!(classes.contains(&"button_secondary".to_string()));
    assert!(classes.contains(&"button_danger".to_string()));
    assert!(classes.contains(&"button_small".to_string()));
    assert!(classes.contains(&"button_medium".to_string()));
    assert!(classes.contains(&"button_large".to_string()));
    assert!(classes.contains(&"button_disabled".to_string()));
    
    println!("✅ Button component test passed - found {} classes", classes.len());
}
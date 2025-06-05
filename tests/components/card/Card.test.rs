// tests/components/card/Card.test.rs

use rustbrother::{extract_css_references_with_css_context, AnalysisConfig};
use std::fs;

#[test]
fn test_card_component() {
    let component_content = fs::read_to_string("tests/components/card/Card.tsx").unwrap();
    
    let css_classes = vec![
        // Base classes
        "card".to_string(),
        "cardHeader".to_string(),
        "cardBody".to_string(),
        "cardFooter".to_string(),
        
        // Theme variants
        "card_theme_dark".to_string(),
        "card_theme_light".to_string(),
        "card_theme_colorful".to_string(),
        
        // Elevation variants
        "card_elevation_low".to_string(),
        "card_elevation_medium".to_string(),
        "card_elevation_high".to_string(),
    ];

    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
    };

    let classes = extract_css_references_with_css_context(&component_content, &config, &css_classes);
    
    println!("🔍 Card analysis found {} classes:", classes.len());
    for class in &classes {
        println!("  - {}", class);
    }

    // Base classes from destructuring
    assert!(classes.contains(&"card".to_string()), "Missing: card");
    assert!(classes.contains(&"cardHeader".to_string()), "Missing: cardHeader");
    assert!(classes.contains(&"cardBody".to_string()), "Missing: cardBody");
    assert!(classes.contains(&"cardFooter".to_string()), "Missing: cardFooter");
    
    // Dynamic theme variants
    assert!(classes.contains(&"card_theme_dark".to_string()), "Missing: card_theme_dark");
    assert!(classes.contains(&"card_theme_light".to_string()), "Missing: card_theme_light");
    assert!(classes.contains(&"card_theme_colorful".to_string()), "Missing: card_theme_colorful");
    
    // Dynamic elevation variants
    assert!(classes.contains(&"card_elevation_low".to_string()), "Missing: card_elevation_low");
    assert!(classes.contains(&"card_elevation_medium".to_string()), "Missing: card_elevation_medium");
    assert!(classes.contains(&"card_elevation_high".to_string()), "Missing: card_elevation_high");
    
    println!("✅ Card component test passed");
}
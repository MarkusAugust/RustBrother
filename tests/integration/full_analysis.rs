use rustbrother::{analyze_directory, AnalysisConfig};
use std::path::Path;

#[test]
fn test_full_component_analysis() {
    let test_path = Path::new("tests/components");
    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
        ..Default::default()
    };

    let result = analyze_directory(test_path, &config).unwrap();
    
    println!("📊 Full Analysis Results:");
    println!("  Used classes: {}", result.used_classes.len());
    println!("  Unused classes: {}", result.unused_classes.len());
    
    // Should find some classes
    assert!(result.used_classes.len() > 0, "Should find used classes");
    
    println!("✅ Full analysis test passed");
}
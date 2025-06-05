// tests/components/panel/Panel.test.rs

use rustbrother::{extract_css_references_with_css_context, AnalysisConfig};
use std::fs;

#[test]
fn test_panel_component_with_mixed_valid_and_invalid_classes() {
    let component_content = fs::read_to_string("tests/components/panel/Panel.tsx").unwrap();
    
    // ONLY include classes that ACTUALLY exist in the SCSS file
    let actual_css_classes = vec![
        // Base classes (✅ EXIST)
        "panel".to_string(),
        "panelGraphic".to_string(),
        "panelArticle".to_string(),
        "panelImage".to_string(),
        "panelResponsive".to_string(),
        "panelGraphicHide".to_string(),
        "panelFooter".to_string(),  // Added in SCSS
        "srOnly".to_string(),
        
        // Variant classes (✅ EXIST)
        "panel_filled".to_string(),
        "panel_outline".to_string(),
        
        // Color classes (✅ EXIST)
        "panel_ochre".to_string(),
        "panel_forest".to_string(),
        "panel_burgundy".to_string(),
        "panel_denim".to_string(),
        "panel_graphite".to_string(),
        
        // Padding classes (✅ EXIST)
        "panel_paddingS".to_string(),
        "panel_paddingM".to_string(),
        "panel_paddingL".to_string(),
        "panel_paddingXL".to_string(),
        "panel_paddingXXL".to_string(),
        "panel_paddingMEGA".to_string(),
        
        // Spacing classes (✅ EXIST)
        "panel_spacingXXS".to_string(),
        "panel_spacingXS".to_string(),
        "panel_spacingS".to_string(),
        "panel_spacingM".to_string(),
        "panel_spacingL".to_string(),
        "panel_spacingXL".to_string(),
        "panel_spacingXXL".to_string(),
        "panel_spacingMEGA".to_string(),
        
        // Graphic classes that EXIST (✅ EXIST)
        "panel_graphicImage".to_string(),
        
        // ❌ INTENTIONALLY EXCLUDED (these don't exist in SCSS):
        // "panel_graphicIcon" - MISSING FROM CSS
        // "panel_graphicIconHide" - MISSING FROM CSS  
        // "panel_iconContainer" - MISSING FROM CSS
        // "panel_wrongClass" - MISSING FROM CSS
    ];

    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
    };

    let classes = extract_css_references_with_css_context(&component_content, &config, &actual_css_classes);
    
    println!("🔍 Panel analysis found {} classes:", classes.len());
    for class in &classes {
        println!("  - {}", class);
    }

    // ✅ SHOULD FIND: Classes that exist in both TSX and SCSS
    assert!(classes.contains(&"panel".to_string()), "Should find: panel");
    assert!(classes.contains(&"panelGraphic".to_string()), "Should find: panelGraphic");
    assert!(classes.contains(&"panelArticle".to_string()), "Should find: panelArticle");
    assert!(classes.contains(&"panelImage".to_string()), "Should find: panelImage");
    assert!(classes.contains(&"panelGraphicHide".to_string()), "Should find: panelGraphicHide");
    assert!(classes.contains(&"panelFooter".to_string()), "Should find: panelFooter");
    assert!(classes.contains(&"srOnly".to_string()), "Should find: srOnly");

    // ✅ SHOULD FIND: Variant classes (ternary)
    assert!(classes.contains(&"panel_filled".to_string()), "Should find: panel_filled");
    assert!(classes.contains(&"panel_outline".to_string()), "Should find: panel_outline");

    // ✅ SHOULD FIND: Dynamic classes that exist
    assert!(classes.contains(&"panel_ochre".to_string()), "Should find: panel_ochre");
    assert!(classes.contains(&"panel_paddingS".to_string()), "Should find: panel_paddingS");
    assert!(classes.contains(&"panel_spacingM".to_string()), "Should find: panel_spacingM");
    assert!(classes.contains(&"panel_graphicImage".to_string()), "Should find: panel_graphicImage");

    // ❌ SHOULD NOT FIND: Classes used in TSX but missing from SCSS
    assert!(!classes.contains(&"panel_graphicIcon".to_string()), 
        "Should NOT find: panel_graphicIcon (missing from CSS!)");
    assert!(!classes.contains(&"panel_graphicIconHide".to_string()), 
        "Should NOT find: panel_graphicIconHide (missing from CSS!)");
    assert!(!classes.contains(&"panel_iconContainer".to_string()), 
        "Should NOT find: panel_iconContainer (missing from CSS!)");
    assert!(!classes.contains(&"panel_wrongClass".to_string()), 
        "Should NOT find: panel_wrongClass (missing from CSS!)");

    println!("✅ Panel mixed test passed!");
    println!("   Found {} existing classes", classes.len());
    println!("   Correctly excluded {} non-existent classes", 4);
}

#[test]
fn test_verify_missing_classes_behavior() {
    let component_content = fs::read_to_string("tests/components/panel/Panel.tsx").unwrap();
    
    // Test with ALL classes (including the ones that don't exist)
    let all_classes_including_fake = vec![
        "panel".to_string(),
        "panelGraphic".to_string(),
        "panel_graphicImage".to_string(),
        // Add the fake ones
        "panel_graphicIcon".to_string(),
        "panel_graphicIconHide".to_string(),
        "panel_iconContainer".to_string(),
        "panel_wrongClass".to_string(),
    ];

    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
    };

    let classes_with_fake = extract_css_references_with_css_context(&component_content, &config, &all_classes_including_fake);
    
    // Now test with only real classes
    let only_real_classes = vec![
        "panel".to_string(),
        "panelGraphic".to_string(),
        "panel_graphicImage".to_string(),
    ];

    let classes_real_only = extract_css_references_with_css_context(&component_content, &config, &only_real_classes);

    println!("🔍 With fake classes included: {} found", classes_with_fake.len());
    println!("🔍 With only real classes: {} found", classes_real_only.len());

    // The fake classes should be found when included in CSS list
    assert!(classes_with_fake.contains(&"panel_graphicIcon".to_string()), 
        "When fake class is in CSS list, parser should find it in TSX");
    
    // But not found when excluded from CSS list
    assert!(!classes_real_only.contains(&"panel_graphicIcon".to_string()), 
        "When fake class is NOT in CSS list, parser should NOT find it");

    println!("✅ Missing classes behavior test passed!");
    println!("   Parser correctly respects the CSS classes list");
}

#[test]
fn test_panel_component_with_wrong_class_names() {
    // This test uses a component that tries to use classes that DON'T exist in the CSS
    let wrong_component_content = r#"
import styles from './Panel.module.scss';

export const PanelWrong = ({
  imageSource,
  renderIcon,
  hideGraphicMobile,
  children,
}) => {
  let panelGraphicClassName = '';
  if (imageSource) {
    panelGraphicClassName = styles.panel_graphicImage;  // ✅ This exists
  } else if (renderIcon) {
    // ❌ These classes DON'T exist in the actual SCSS:
    panelGraphicClassName = `${styles.panel_graphicIcon} ${
      hideGraphicMobile ? styles.panel_graphicIconHide : ''
    }`.trim();
  }

  return (
    <div className={`${styles.panel} ${panelGraphicClassName}`}>
      <div className={styles.panelGraphic}>
        <img className={styles.panelImage} />
      </div>
      <div className={styles.panelArticle}>
        {children}
      </div>
    </div>
  );
};
"#;
    
    // Only include classes that ACTUALLY exist in the CSS
    let actual_css_classes = vec![
        "panel".to_string(),
        "panelGraphic".to_string(),
        "panelArticle".to_string(),
        "panelImage".to_string(),
        "panel_graphicImage".to_string(),  // This exists
        // panel_graphicIcon - INTENTIONALLY EXCLUDED (doesn't exist)
        // panel_graphicIconHide - INTENTIONALLY EXCLUDED (doesn't exist)
    ];

    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
    };

    let classes = extract_css_references_with_css_context(wrong_component_content, &config, &actual_css_classes);
    
    println!("🔍 Wrong component analysis found {} classes:", classes.len());
    for class in &classes {
        println!("  - {}", class);
    }

    // Classes that exist should be found
    assert!(classes.contains(&"panel".to_string()), "Should find: panel");
    assert!(classes.contains(&"panelGraphic".to_string()), "Should find: panelGraphic");
    assert!(classes.contains(&"panelArticle".to_string()), "Should find: panelArticle");
    assert!(classes.contains(&"panelImage".to_string()), "Should find: panelImage");
    assert!(classes.contains(&"panel_graphicImage".to_string()), "Should find: panel_graphicImage (exists)");
    
    // ❌ CRITICAL TEST: Classes that DON'T exist should NOT be found
    assert!(!classes.contains(&"panel_graphicIcon".to_string()), 
        "Should NOT find: panel_graphicIcon (this class doesn't exist in CSS!)");
    assert!(!classes.contains(&"panel_graphicIconHide".to_string()), 
        "Should NOT find: panel_graphicIconHide (this class doesn't exist in CSS!)");
    
    println!("✅ Wrong component test passed - correctly identified missing classes");
}

#[test]
fn test_panel_component_correct_vs_wrong_usage() {
    // Test the difference between correct and incorrect class usage
    
    let correct_component = r#"
export const PanelCorrect = ({ imageSource, hideGraphicMobile }) => {
  const graphicClassName = `${styles.panelGraphic} ${hideGraphicMobile ? styles.panelGraphicHide : ''}`;
  
  return (
    <div className={styles.panel}>
      {imageSource && (
        <div className={graphicClassName}>
          <img className={styles.panelImage} />
        </div>
      )}
    </div>
  );
};
"#;

    let wrong_component = r#"
export const PanelWrong = ({ renderIcon, hideGraphicMobile }) => {
  // ❌ Trying to use classes that don't exist:
  const iconClassName = `${styles.panel_graphicIcon} ${hideGraphicMobile ? styles.panel_graphicIconHide : ''}`;
  
  return (
    <div className={styles.panel}>
      {renderIcon && <div className={iconClassName}>Icon</div>}
    </div>
  );
};
"#;

    let css_classes = vec![
        "panel".to_string(),
        "panelGraphic".to_string(),
        "panelGraphicHide".to_string(),
        "panelImage".to_string(),
        // Note: panel_graphicIcon and panel_graphicIconHide are NOT included
    ];

    let config = AnalysisConfig {
        include_css_modules: true,
        include_styled_components: false,
        ignore_patterns: vec![],
    };

    // Test correct component
    let correct_classes = extract_css_references_with_css_context(correct_component, &config, &css_classes);
    println!("🔍 Correct component found {} classes:", correct_classes.len());
    
    // Test wrong component  
    let wrong_classes = extract_css_references_with_css_context(wrong_component, &config, &css_classes);
    println!("🔍 Wrong component found {} classes:", wrong_classes.len());

    // Correct component should find existing classes
    assert!(correct_classes.contains(&"panel".to_string()));
    assert!(correct_classes.contains(&"panelGraphic".to_string()));
    assert!(correct_classes.contains(&"panelGraphicHide".to_string()));
    assert!(correct_classes.contains(&"panelImage".to_string()));

    // Wrong component should only find base panel class
    assert!(wrong_classes.contains(&"panel".to_string()));
    // But NOT the non-existent classes
    assert!(!wrong_classes.contains(&"panel_graphicIcon".to_string()));
    assert!(!wrong_classes.contains(&"panel_graphicIconHide".to_string()));

    println!("✅ Correct vs Wrong component test passed");
    println!("   Correct component uses {} existing classes", correct_classes.len());
    println!("   Wrong component only finds {} classes (missing non-existent ones)", wrong_classes.len());
}
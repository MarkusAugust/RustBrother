// tests/components/panel/Panel.tsx
import styles from './Panel.module.scss';

export const Panel = ({
  color,
  padding,
  spacing,
  variant,
  hasResponsivePadding,
  hideGraphicMobile,
  imageSource,
  renderIcon,
  hideTitle,
  hideSubtitle,
  children,
}) => {
  const panelVariantClassName = variant === 'filled' ? styles.panel_filled : styles.panel_outline;
  const panelColorClassName = styles[`panel_${color}`];
  const panelPaddingClassName = styles[`panel_padding${padding.toUpperCase()}`];
  const panelSpacingClassName = styles[`panel_spacing${spacing.toUpperCase()}`];

  let panelGraphicClassName = '';
  if (imageSource) {
    panelGraphicClassName = styles.panel_graphicImage;
  } else if (renderIcon) {
    // ❌ WRONG PATTERN: These classes don't exist in SCSS
    panelGraphicClassName = `${styles.panel_graphicIcon} ${
      hideGraphicMobile ? styles.panel_graphicIconHide : ''
    }`.trim();
  }
  
  const panelPaddingResponsiveClassName = hasResponsivePadding ? styles.panelResponsive : '';
  const panelClassName = `${styles.panel} ${panelVariantClassName} ${panelColorClassName} ${panelPaddingClassName} ${panelPaddingResponsiveClassName} ${panelSpacingClassName} ${panelGraphicClassName}`.trim();
  
  // ✅ CORRECT PATTERN: These classes DO exist in SCSS
  const graphicClassName = `${styles.panelGraphic} ${hideGraphicMobile ? styles.panelGraphicHide : ''}`.trim();

  // ❌ WRONG PATTERN: Trying to use another non-existent class
  const wrongIconClassName = styles.panel_iconContainer; // This doesn't exist

  return (
    <div className={panelClassName}>
      {imageSource && (
        <div className={graphicClassName}>
          <img
            src={imageSource}
            className={styles.panelImage}
          />
        </div>
      )}
      {renderIcon && (
        <div className={graphicClassName}>{renderIcon?.()}</div>
      )}
      <div className={styles.panelArticle}>
        <h3 className={hideTitle ? styles.srOnly : ''}>Title</h3>
        <h5 className={hideSubtitle ? styles.srOnly : ''}>Subtitle</h5>
        {/* ❌ WRONG: Another non-existent class */}
        <div className={styles.panel_wrongClass}>
          {children}
        </div>
      </div>
      {/* ✅ CORRECT: Using a class that exists */}
      <div className={styles.panelFooter}>Footer</div>
    </div>
  );
};
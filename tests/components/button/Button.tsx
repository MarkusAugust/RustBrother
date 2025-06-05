
// tests/components/button/Button.tsx
import styles from './Button.module.scss';

export const Button = ({ variant, size, disabled, children }) => {
  const variantClass = variant ? styles[`button_${variant}`] : '';
  const sizeClass = size ? styles[`button_${size}`] : '';
  const disabledClass = disabled ? styles.button_disabled : '';
  
  return (
    <button 
      className={`${styles.button} ${variantClass} ${sizeClass} ${disabledClass}`.trim()}
    >
      <span className={styles.buttonText}>{children}</span>
      <span className={styles.iconWrapper}>
        <Icon className={styles.icon} />
      </span>
    </button>
  );
};
// tests/components/card/Card.tsx
import styles from './Card.module.scss';

export const Card = ({ theme, elevation, children }) => {
  const { card, cardHeader, cardBody, cardFooter } = styles;
  const themeClass = theme ? styles[`card_theme_${theme}`] : '';
  const elevationClass = elevation ? styles[`card_elevation_${elevation}`] : '';
  
  return (
    <div className={`${card} ${themeClass} ${elevationClass}`}>
      <header className={cardHeader}>Header</header>
      <main className={cardBody}>{children}</main>
      <footer className={cardFooter}>Footer</footer>
    </div>
  );
};
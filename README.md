# RustBrother

⚠️ WARNING: THIS IS FIRST AND FOREMOST A PERSONAL TOOL, AND STILL UNDER DEVELOPMENT ⚠️

⚠️ MAY NOT WORK AS EXPECTED ⚠️

_"In the shadows of your codebase, unused CSS lurks like corruption in the Forbidden Lands. But darker forces also dwell within - complex patterns that defy analysis and maintainability. RustBrother hunts down these remnants AND the dark sorcery that breeds them, cleansing your stylesheets with the methodical precision of Zygofer's enforcers."_

A fast CLI tool that finds unused CSS in your React components **and detects complex patterns that harm maintainability**. Clean up your stylesheets by identifying classes, custom properties, and dark sorcery patterns that corrupt your codebase.

## Why Use RustBrother?

- **🧹 Purge the corruption**: Remove unused CSS that festers in your codebase over time
- **⚠️ Detect dark sorcery**: Find complex patterns that make static analysis difficult
- **📦 Reduce bundle size**: Smaller CSS files mean faster page loads
- **🚀 Improve maintainability**: Keep your stylesheets organized and purposeful
- **📊 Map the wasteland**: Understand which parts of your design system are actually used
- **🔍 Code quality insights**: Get warnings about overly complex CSS class usage patterns

## Installation

### Homebrew (Recommended)

```bash
# Add our tap and install
brew tap MarkusAugust/rustbrother
brew install rustbrother
```

### Cargo (From Source)

```bash
# Requires Rust toolchain
cargo install rustbrother
```

### Manual Download

Visit the [releases page](https://github.com/MarkusAugust/RustBrother/releases) and download the appropriate binary for your platform:

```bash
# Example for macOS (Intel)
curl -L https://github.com/MarkusAugust/RustBrother/releases/latest/download/rustbrother-darwin-amd64.tar.gz | tar xz
sudo mv rustbrother /usr/local/bin/

# Example for macOS (Apple Silicon)
curl -L https://github.com/MarkusAugust/RustBrother/releases/latest/download/rustbrother-darwin-arm64.tar.gz | tar xz
sudo mv rustbrother /usr/local/bin/

# Example for Linux
curl -L https://github.com/MarkusAugust/RustBrother/releases/latest/download/rustbrother-linux-amd64.tar.gz | tar xz
sudo mv rustbrother /usr/local/bin/
```

## Quick Start

Hunt down unused CSS and detect complex patterns:

```bash
rustbrother --path ./src/components
```

Generate a detailed HTML purge report with complexity analysis:

```bash
rustbrother --path ./src --format html --output rustbrother-report.html --verbose
```

## Usage Examples

### Basic Analysis (Console Output)

```bash
# Scan your components directory (includes complexity warnings)
rustbrother --path ./src/components

# Scan with verbose progress
rustbrother --path ./src/components --verbose

# Scan an entire project
rustbrother --path ./src
```

### Generate HTML Report

```bash
# Visual HTML report with RustBrother theme (includes complexity analysis)
rustbrother --path ./src --format html --output report.html

# HTML report with verbose progress
rustbrother --path ./src --format html --output rustbrother-analysis.html --verbose

# Scan specific directory with HTML output
rustbrother --path ./src/components --format html --output components-report.html
```

### Generate JSON Report (for CI/CD)

```bash
# JSON output for automation (includes complexity metrics)
rustbrother --path ./src --format json --output analysis.json

# JSON with verbose logging
rustbrother --path ./src --format json --output results.json --verbose

# Scan and save JSON for CI pipeline
rustbrother --path ./src/components --format json --output css-analysis.json
```

### Advanced Usage

```bash
# Disable CSS modules analysis
rustbrother --path ./src --css-modules false

# Multiple output formats
rustbrother --path ./src --format json --output data.json
rustbrother --path ./src --format html --output visual.html

# Help and version
rustbrother --help
rustbrother --version
```

### Options

```bash
rustbrother --help

Options:
  -p, --path <DIR>           Directory to analyze
  -f, --format <FORMAT>      Output format: text, json, html [default: text]
  -o, --output <FILE>        Output file (stdout if not specified)
      --css-modules          Include CSS modules analysis [default: true]
  -v, --verbose             Show detailed progress
  -h, --help                Print help
```

## What RustBrother Hunts

### ✅ Living CSS Classes

Classes that serve their purpose - defined in stylesheets AND referenced in your React components:

```css
.button {
  padding: 8px 16px;
} /* ✅ Actively used in Button.jsx */
```

### 🗑️ Corrupted Remnants

Classes that have outlived their purpose - defined but never referenced:

```css
.old-button {
  /* ... */
} /* 🗑️ Abandoned relic */
.legacy-style {
  /* ... */
} /* 🗑️ Forgotten fragment */
```

### ⚠️ Dark Sorcery Patterns

Complex CSS usage patterns that harm maintainability and static analysis:

#### 🔴 High Severity - Forbidden Dark Arts

```javascript
// Multi-variable templates are very hard to analyze
const className = styles[`${base}_${variant}_${size}`]

// Function calls make static analysis impossible
const className = styles[getClassName(props)]

// String concatenation defeats analysis
const className = styles[prefix + '_' + suffix]
```

#### 🟡 Medium Severity - Cursed Patterns

```javascript
// Dynamic class construction with variables
const className = styles[`${variant}_${size}`] // Your Icon component!

// Complex conditional assignments
const colorClass = color !== 'none' ? styles[`accordion_${color}`] : ''

// Template expressions that are hard to track
const className = `${styles.base} ${variantClass}`
```

#### 🟢 Low Severity - Tainted Code

```javascript
// Single variable templates
const className = styles[`button_${variant}`]

// Simple dynamic access
const className = styles[variantName]
```

### 🎨 CSS Custom Properties (Detected)

CSS variables found in your stylesheets:

```css
:root {
  --primary-color: #007bff; /* 🔍 Found and tracked */
  --secondary-color: #6c757d; /* 🔍 Usage analysis included */
}
```

## Example Analysis Report

```
⚔️  RustBrother CSS Analysis Report
====================================

📊 Territory Analysis:
  Total CSS classes found: 47
  Active classes: 32
  Corrupted remnants: 15 (32%)
  Files patrolled: 12
  ⚠️  Dark sorcery detected: 8 (🔴 2 forbidden, 🟡 4 cursed, 🟢 2 tainted)

⚠️  Dark Sorcery Detected:
-------------------------

📄 src/components/Icon/Icon.tsx:
  🟡 Multi-variable dark arts (line 15)
     Spell pattern: styles[`${variant}_${size}`]
     💡 The ${variable}_${variable} pattern is hard to analyze statically. Consider explicit class mapping: CLASS_MAP[variant][size]

📄 src/components/Accordion/Accordion.tsx:
  🟡 Dark conditional magic (line 23)
     Spell pattern: const colorClassName = color !== 'none' ? styles[`accordion_${color}`] : '';
     💡 Consider using a function to handle conditional class logic: getClassName(condition, variant)

🗑️ Remnants Marked for Purging:
-------------------------------

📄 src/components/Button/Button.css:
  • .btn-large (line 34)
  • .btn-outline-dark (line 67)
  • .legacy-button (line 89)

📄 src/components/Card/Card.css:
  • .card-old-style (line 12)
  • .card-deprecated (line 23)

💡 Arcane Knowledge for Cleansing Dark Sorcery:
• Forge explicit class mapping: CLASS_MAP[variant][size]
• Banish complex logic into helper functions
• Consider CSS-in-JS libraries with better static analysis
• Break down large template expressions into smaller incantations

⚔️  Enforcement complete!
```

## HTML Report Features

The HTML report now includes:

- **📊 Visual metrics dashboard** with corruption and complexity statistics
- **⚠️ Interactive complexity warnings** with collapsible sections
- **🗑️ Unused CSS breakdown** by file
- **🎨 Epic RustBrother theme** with forbidden lands aesthetic
- **📱 Responsive design** for mobile and desktop viewing
- **🔍 Searchable content** and organized sections

## Integration

### CI/CD Pipeline Enforcement

```yaml
# .github/workflows/rustbrother-patrol.yml
- name: RustBrother CSS Patrol
  run: |
    rustbrother --path ./src --format json --output css-analysis.json
    # Fail if corruption levels too high
    UNUSED_COUNT=$(jq '.summary.unused_classes' css-analysis.json)
    COMPLEXITY_HIGH=$(jq '.summary.complexity_warnings.high' css-analysis.json)

    if [ $UNUSED_COUNT -gt 20 ]; then
      echo "Too much CSS corruption detected: $UNUSED_COUNT unused classes!"
      exit 1
    fi

    if [ $COMPLEXITY_HIGH -gt 5 ]; then
      echo "Too much dark sorcery detected: $COMPLEXITY_HIGH high-severity patterns!"
      exit 1
    fi
```

### Pre-commit Hook

```bash
#!/bin/sh
# RustBrother patrol before each commit
rustbrother --path ./src --complexity-threshold medium
if [ $? -ne 0 ]; then
  echo "RustBrother patrol failed! Fix the issues above."
  exit 1
fi
```

### NPM Script

```json
{
  "scripts": {
    "css:patrol": "rustbrother --path ./src --verbose",
    "css:report": "rustbrother --path ./src --format html --output rustbrother-report.html"
  }
}
```

## Supported Territories

**CSS Files**: `.css`, `.scss`, `.sass`
**React Files**: `.js`, `.jsx`, `.ts`, `.tsx`

**Hunting Patterns**:

- `className="my-class"`
- `className={'my-class'}`
- `className={styles.myClass}` (CSS modules)
- `styles[`template\_${variable}`]` ⚠️ (complexity warning)
- `styles[`${var1}_${var2}`]` ⚠️ (complexity warning)
- CSS custom properties (`--variable-name`)

**Complexity Detection**:

- Dynamic class construction patterns
- Multi-variable template literals
- Complex conditional assignments
- Untrackable dynamic patterns
- Template expression nesting

## Code Quality Benefits

RustBrother's complexity analysis helps you:

- **🔍 Identify maintenance pain points** before they become problems
- **📚 Improve code readability** by flagging overly complex patterns
- **🚀 Enable better tooling** by using patterns that static analysis can understand
- **👥 Guide team standards** with concrete examples of what to avoid
- **⚡ Prevent analysis failures** by catching patterns that break CSS detection

## Known Limitations

- **Static analysis only** - cannot detect dynamically conjured class names
- **CSS-in-JS libraries** require additional configuration
- **Complex template literals** may evade detection (and trigger complexity warnings!)
- **Runtime class generation** is not trackable
- **Complexity warnings are suggestions** - use your judgment for refactoring decisions

## Configuration Tips

### For Legacy Codebases

```bash
# Start with basic analysis to see overall health
rustbrother --path ./src --verbose

# Generate visual reports for team review
rustbrother --path ./src --format html --output legacy-analysis.html
```

### For New Projects

```bash
# Enforce clean patterns from the start
rustbrother --path ./src --verbose

# Generate comprehensive reports
rustbrother --path ./src --format html --output report.html --verbose
```

### For CI/CD

```bash
# Generate JSON for automated checks
rustbrother --path ./src --format json --output analysis.json
```

## Join the Order

Found corruption we missed, want to enhance our enforcement, or have ideas for new complexity patterns to detect? Check out our [GitHub stronghold](https://github.com/your-org/rustbrother).

## License

MIT License - see LICENSE file for details.

---

_"The RustBrothers patrol the forgotten corners of your codebase, ensuring no corruption goes unnoticed. We hunt not only the abandoned remnants, but also the dark sorcery that breeds complexity and chaos. Trust in the rust."_

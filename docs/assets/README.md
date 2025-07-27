# MindVault Assets

This folder contains visual assets for the MindVault project.

## 🎨 Available Assets

### Favicon
- **`favicon.svg`** - Main favicon/logo for the project (64x64 optimized)
- **`site.webmanifest`** - Web app manifest for PWA support
- **`favicon.html`** - HTML snippet with favicon links for Jekyll

### Social Preview
- **`social-preview.svg`** - GitHub Social Preview Card (1200x630)
- **`social-preview-preview.html`** - Preview page for the social card
- **`generate_social_preview.py`** - Generator script for social assets

### Generators
- **`generate_favicon.py`** - Script to generate favicon assets
- **`create_simple_favicon.py`** - Simple favicon creation utility

## 🔐 Favicon Design

The MindVault favicon represents the core concept of secure knowledge storage:

- **🔒 Vault/Safe**: Central secure storage metaphor
- **🧠 Brain Symbol**: Personal knowledge and notes (red)
- **✅ Checkmark**: Task management (green)
- **📄 Document**: Notes and reminders (orange)
- **👥 People**: Reportee reviews and team management (purple)

## 📱 Social Preview Card

The GitHub Social Preview Card (1200x630 pixels) includes:

- **MindVault Logo**: Scaled version of the favicon
- **Project Title**: Clear branding and tagline
- **Feature Highlights**: Four main features with color coding
- **Integration Options**: UI, LLM, and API access
- **Tech Stack**: Rust, MongoDB, MIT license badges
- **Links**: GitHub repository and documentation URLs

## 🚀 Usage Instructions

### Setting up Favicon
1. The favicon is automatically included in Jekyll via `_includes/head.html`
2. Modern browsers will use the SVG version
3. For broader compatibility, convert to PNG formats:
   - 16x16 pixels: `favicon-16x16.png`
   - 32x32 pixels: `favicon-32x32.png`
   - 180x180 pixels: `apple-touch-icon.png`

### Setting up GitHub Social Preview
1. Convert `social-preview.svg` to PNG (1200x630 pixels)
2. Go to GitHub repository Settings → Social preview
3. Upload the PNG file (must be < 1MB)
4. The image will appear when the repo is shared on social media

## 🛠️ Conversion Tools

### Online Converters
- [CloudConvert](https://cloudconvert.com/svg-to-png)
- [Convertio](https://convertio.co/svg-png/)
- [FreeConvert](https://www.freeconvert.com/svg-to-png)

### Command Line Tools
```bash
# Using Inkscape
inkscape social-preview.svg --export-png=social-preview.png --export-width=1200 --export-height=630

# Using ImageMagick
convert social-preview.svg -resize 1200x630 social-preview.png
```

### Desktop Software
- **Inkscape** (free): Export PNG at specific dimensions
- **Adobe Illustrator**: Export for screens
- **Figma**: Export as PNG 2x

## 🎨 Design Guidelines

### Colors
- **Primary**: #3498db (blue)
- **Secondary**: #2c3e50 (dark blue-gray)
- **Accent Colors**:
  - Notes: #e74c3c (red)
  - Tasks: #27ae60 (green)
  - Documents: #f39c12 (orange)
  - Reviews: #9b59b6 (purple)

### Typography
- **Primary Font**: Arial, sans-serif (for maximum compatibility)
- **Weights**: Regular, Bold
- **Hierarchy**: Clear size differentiation for titles, subtitles, and body text

### Layout
- **Clean Design**: Minimal, professional appearance
- **Consistent Spacing**: Proper margins and padding
- **Visual Hierarchy**: Clear information organization
- **Brand Consistency**: Consistent use of colors and typography

## 📐 Technical Specifications

### Favicon
- **Format**: SVG (primary), PNG (fallback)
- **Dimensions**: 64x64 pixels (scalable)
- **Colors**: Full color with transparency support

### Social Preview
- **Format**: SVG (source), PNG (GitHub)
- **Dimensions**: 1200x630 pixels
- **Aspect Ratio**: 1.91:1
- **File Size**: < 1MB (GitHub requirement)

## 🔄 Updating Assets

When updating the visual assets:

1. **Maintain Consistency**: Keep the same design language
2. **Test Compatibility**: Ensure assets work across different platforms
3. **Optimize File Size**: Keep files as small as possible
4. **Update Documentation**: Update this README with any changes
5. **Regenerate Variants**: Create all necessary format variants

## 📄 License

These assets are part of the MindVault project and follow the same MIT license terms.

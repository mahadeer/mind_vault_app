# MindVault Documentation

This directory contains the documentation for MindVault Task Management System, built with Jekyll and deployed to GitHub Pages.

## 🌐 Live Documentation

Visit the live documentation at: `https://yourusername.github.io/mind_vault_app/`

## 📁 Documentation Structure

- **index.md** - Homepage with overview and navigation
- **getting_started.md** - Installation and setup guide
- **features.md** - Complete feature overview
- **architecture_overview.md** - System design and components
- **tech_stack.md** - Technologies and frameworks
- **implementation.md** - Development progress and milestones
- **usage_examples.md** - Practical examples and LLM integration
- **project_layout.md** - Code organization and structure
- **how_to_run.md** - Running and testing the application

## 🚀 Local Development

### Prerequisites

- Ruby 3.1 or higher
- Bundler gem

### Setup

1. Navigate to the docs directory:
   ```bash
   cd docs
   ```

2. Install dependencies:
   ```bash
   bundle install
   ```

3. Serve the site locally:
   ```bash
   bundle exec jekyll serve
   ```

4. Open your browser to `http://localhost:4000`

### Live Reload

The site will automatically reload when you make changes to the markdown files or configuration.

## 🎨 Customization

### Styling

Custom styles are defined in `_sass/custom.scss` and include:
- Grid layout for documentation sections
- Enhanced code blocks and tables
- Responsive design
- Dark mode support
- Alert boxes and badges

### Configuration

Site configuration is in `_config.yml`:
- Site title and description
- Navigation structure
- Plugin configuration
- SEO settings

### Layouts

Custom layouts are in `_layouts/`:
- `default.html` - Base layout with custom CSS

## 📝 Adding New Pages

1. Create a new markdown file in the docs directory
2. Add front matter with layout, title, nav_order, and description
3. Update `_config.yml` header_pages if needed for navigation

Example front matter:
```yaml
---
layout: page
title: New Page
nav_order: 10
description: Description of the new page
---
```

## 🚀 Deployment

The site is automatically deployed to GitHub Pages using GitHub Actions when changes are pushed to the main branch.

### Manual Deployment

If you need to deploy manually:

1. Build the site:
   ```bash
   bundle exec jekyll build
   ```

2. The built site will be in `_site/` directory

### GitHub Pages Setup

1. Go to your repository settings
2. Navigate to "Pages" section
3. Set source to "GitHub Actions"
4. The site will be available at `https://yourusername.github.io/repositoryname/`

## 🔧 Configuration Notes

### Base URL

Update the `baseurl` in `_config.yml` to match your repository name:
```yaml
baseurl: "/your-repository-name"
```

### Site URL

Update the `url` in `_config.yml` to match your GitHub username:
```yaml
url: "https://yourusername.github.io"
```

## 📚 Jekyll Resources

- [Jekyll Documentation](https://jekyllrb.com/docs/)
- [Minima Theme](https://github.com/jekyll/minima)
- [GitHub Pages Documentation](https://docs.github.com/en/pages)

## 🤝 Contributing

When adding or updating documentation:

1. Follow the existing structure and style
2. Add appropriate front matter to new pages
3. Test locally before committing
4. Update navigation if needed
5. Ensure responsive design works on mobile

## 📄 License

This documentation is part of the MindVault project and follows the same license terms.

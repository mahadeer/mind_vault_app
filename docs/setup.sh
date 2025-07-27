#!/bin/bash

# MindVault Documentation Setup Script
# This script sets up the Jekyll environment for local development

echo "🚀 Setting up MindVault Documentation"
echo "====================================="

# Check if Ruby is installed
if ! command -v ruby &> /dev/null; then
    echo "❌ Ruby is not installed. Please install Ruby 3.1 or higher."
    echo "   Visit: https://www.ruby-lang.org/en/documentation/installation/"
    exit 1
fi

# Check Ruby version
RUBY_VERSION=$(ruby -v | cut -d' ' -f2 | cut -d'.' -f1,2)
REQUIRED_VERSION="3.1"

if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$RUBY_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
    echo "❌ Ruby version $RUBY_VERSION is too old. Please install Ruby 3.1 or higher."
    exit 1
fi

echo "✅ Ruby version $RUBY_VERSION detected"

# Check if Bundler is installed
if ! command -v bundle &> /dev/null; then
    echo "📦 Installing Bundler..."
    gem install bundler
fi

echo "✅ Bundler is available"

# Install Jekyll dependencies
echo "📦 Installing Jekyll and dependencies..."
bundle install

if [ $? -eq 0 ]; then
    echo "✅ Dependencies installed successfully"
else
    echo "❌ Failed to install dependencies"
    exit 1
fi

# Create .gitignore for Jekyll if it doesn't exist
if [ ! -f .gitignore ]; then
    echo "📝 Creating .gitignore for Jekyll..."
    cat > .gitignore << EOF
_site/
.sass-cache/
.jekyll-cache/
.jekyll-metadata
vendor/
.bundle/
Gemfile.lock
EOF
    echo "✅ .gitignore created"
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "To start the development server:"
echo "  bundle exec jekyll serve"
echo ""
echo "Then open your browser to:"
echo "  http://localhost:4000"
echo ""
echo "The site will automatically reload when you make changes."
echo ""
echo "Happy documenting! 📚"

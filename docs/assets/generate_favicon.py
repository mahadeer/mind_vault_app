#!/usr/bin/env python3
"""
Generate favicon files from SVG for MindVault.
This script creates various favicon formats needed for web compatibility.
"""

import os
import sys

def create_favicon_html():
    """Create HTML snippet for favicon links."""
    return '''<!-- Favicon -->
<link rel="icon" type="image/svg+xml" href="{{ '/assets/favicon.svg' | relative_url }}">
<link rel="icon" type="image/png" sizes="32x32" href="{{ '/assets/favicon-32x32.png' | relative_url }}">
<link rel="icon" type="image/png" sizes="16x16" href="{{ '/assets/favicon-16x16.png' | relative_url }}">
<link rel="apple-touch-icon" sizes="180x180" href="{{ '/assets/apple-touch-icon.png' | relative_url }}">
<link rel="manifest" href="{{ '/assets/site.webmanifest' | relative_url }}">
<meta name="theme-color" content="#3498db">'''

def create_webmanifest():
    """Create web app manifest for PWA support."""
    return '''{
    "name": "MindVault - Personal Knowledge Management",
    "short_name": "MindVault",
    "description": "Store and categorize all your personal notes, tasks, reportee reviews, and reminders for easy access",
    "icons": [
        {
            "src": "/mindvault/assets/favicon-16x16.png",
            "sizes": "16x16",
            "type": "image/png"
        },
        {
            "src": "/mindvault/assets/favicon-32x32.png",
            "sizes": "32x32",
            "type": "image/png"
        },
        {
            "src": "/mindvault/assets/apple-touch-icon.png",
            "sizes": "180x180",
            "type": "image/png"
        }
    ],
    "theme_color": "#3498db",
    "background_color": "#ffffff",
    "display": "standalone",
    "start_url": "/mindvault/"
}'''

def main():
    """Generate favicon files and related assets."""
    
    # Create assets directory if it doesn't exist
    assets_dir = os.path.dirname(os.path.abspath(__file__))
    
    print("Generating favicon assets for MindVault...")
    
    # Create favicon HTML snippet
    favicon_html_path = os.path.join(assets_dir, 'favicon.html')
    with open(favicon_html_path, 'w') as f:
        f.write(create_favicon_html())
    print("Created favicon.html")
    
    # Create web manifest
    manifest_path = os.path.join(assets_dir, 'site.webmanifest')
    with open(manifest_path, 'w') as f:
        f.write(create_webmanifest())
    print("Created site.webmanifest")
    
    print("\nFavicon setup complete!")
    print("\nTo complete the setup:")
    print("1. Convert favicon.svg to PNG formats using an online converter or image editor:")
    print("   - favicon-16x16.png (16x16 pixels)")
    print("   - favicon-32x32.png (32x32 pixels)")
    print("   - apple-touch-icon.png (180x180 pixels)")
    print("2. Add the favicon.html content to your Jekyll layout head section")
    print("3. The SVG favicon will work in modern browsers")
    
    # Instructions for manual conversion
    print("\nOnline converters you can use:")
    print("- https://favicon.io/favicon-converter/")
    print("- https://realfavicongenerator.net/")
    print("- https://www.favicon-generator.org/")

if __name__ == "__main__":
    main()

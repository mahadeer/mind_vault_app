# -*- coding: utf-8 -*-
import os
import sys

def get_excluded_items(ignore_file_path):
    """
    Reads excluded items from a .layoutIgnore file.
    Each non-empty, non-comment line is treated as an item to exclude.
    """
    excluded = set()
    try:
        with open(ignore_file_path, 'r') as f:
            for line in f:
                stripped_line = line.strip()
                if stripped_line and not stripped_line.startswith('#'):
                    excluded.add(stripped_line)
    except IOError:
        sys.stderr.write("Warning: .layoutIgnore file not found at '{}'. No exclusions will be applied.\n".format(ignore_file_path))
    except Exception as e:
        sys.stderr.write("Error reading .layoutIgnore file '{}': {}\n".format(ignore_file_path, e))
    return excluded

def generate_simple_tree(root_dir, output_file_path, ignore_file_path):
    """
    Generates a simplified directory tree showing root level files/folders,
    and then contents of specific folders (core, mcp, server, shared) down to their src/.
    """
    if not os.path.isdir(root_dir):
        sys.stderr.write("Error: Root directory '{}' does not exist.\n".format(root_dir))
        return False

    excluded_items = get_excluded_items(ignore_file_path)

    tree_lines = []
    tree_lines.append("```") # Start code block for plain text tree

    # Get immediate contents of the root directory
    # Filter out 'docs' folder from this initial listing if it's the root for the output
    root_items_raw = os.listdir(root_dir)
    root_items = sorted([item for item in root_items_raw if item not in excluded_items], key=str.lower)

    # Specific folders to drill into (core, mcp, server, shared)
    project_sub_folders_to_detail = [
    'mindvault-core',
    'mindvault-mcp',
    'mindvault-api',
    'mindvault-shared',
    'mindvault-ui'
    ]

    for item in root_items:
        full_path = os.path.join(root_dir, item)

        if os.path.isdir(full_path):
            if item in project_sub_folders_to_detail:
                tree_lines.append("📁 {}/".format(item)) # Main project folder
                sub_items = sorted(os.listdir(full_path), key=str.lower)
                for sub_item in sub_items:
                    sub_full_path = os.path.join(full_path, sub_item)

                    if sub_item in excluded_items:
                        continue # Skip excluded items even within sub-folders

                    if os.path.isdir(sub_full_path):
                        if sub_item == 'src': # Special treatment for 'src' folder
                            tree_lines.append("  ├── 📂 {}/".format(sub_item)) # src folder icon
                            src_items = sorted(os.listdir(sub_full_path), key=str.lower)
                            for src_item in src_items:
                                if src_item in excluded_items:
                                    continue
                                # Just list items in src, don't recurse further
                                if os.path.isdir(os.path.join(sub_full_path, src_item)):
                                     tree_lines.append("  │   ├── 📁 {}/".format(src_item))
                                else:
                                     tree_lines.append("  │   ├── 📄 {}".format(src_item))
                        else: # Other sub-folders of core/mcp/server/shared (not src)
                            tree_lines.append("  ├── 📁 {}/".format(sub_item))
                    elif os.path.isfile(sub_full_path):
                        tree_lines.append("  ├── 📄 {}".format(sub_item))
            else: # Other top-level directories (e.g., docs/, but excluded)
                tree_lines.append("📁 {}/".format(item)) # Default folder icon for others
        elif os.path.isfile(full_path):
            tree_lines.append("📄 {}".format(item)) # File icon for top-level files

    tree_lines.append("```") # End code block

    # Add a header for the section
    markdown_content = "## Project Layout\n\n"
    markdown_content += "Here's a simplified overview of the project's folder structure:\n\n"
    markdown_content += "\n".join(tree_lines) + "\n"

    try:
        with open(output_file_path, 'w') as f:
            f.write(markdown_content)
        print("Successfully generated project layout in '{}'".format(output_file_path))
        return True
    except Exception as e:
        sys.stderr.write("Error writing to file '{}': {}\n".format(output_file_path, e))
        return False

if __name__ == "__main__":
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.abspath(os.path.join(script_dir, os.pardir, os.pardir))

    output_file = os.path.join(project_root, "docs", "project_layout.md")
    ignore_file = os.path.join(script_dir, ".layoutIgnore") # This path should be correct now

    print("Generating simple tree for project root: '{}'".format(project_root))
    print("Outputting to: '{}'".format(output_file))
    print("Reading exclusions from: '{}'".format(ignore_file))

    if not generate_simple_tree(project_root, output_file, ignore_file):
        sys.exit(1)
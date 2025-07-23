import os
import sys

def get_excluded_items(ignore_file_path):
    """
    Reads excluded items from a .layoutIgnore file.
    Each non-empty, non-comment line is treated as an item to exclude.
    """
    excluded = set()
    try:
        with open(ignore_file_path, 'r', encoding='utf-8') as f:
            for line in f:
                stripped_line = line.strip()
                if stripped_line and not stripped_line.startswith('#'):
                    excluded.add(stripped_line)
    except FileNotFoundError:
        print(f"Warning: .layoutIgnore file not found at '{ignore_file_path}'. No exclusions will be applied.", file=sys.stderr)
    except Exception as e:
        print(f"Error reading .layoutIgnore file '{ignore_file_path}': {e}", file=sys.stderr)
    return excluded

def generate_simple_tree(root_dir, output_file_path, ignore_file_path):
    """
    Generates a simplified directory tree showing root level files/folders,
    and then contents of specific folders (core, mcp, server, shared) down to their src/.
    """
    if not os.path.isdir(root_dir):
        print(f"Error: Root directory '{root_dir}' does not exist.", file=sys.stderr)
        return False

    excluded_items = get_excluded_items(ignore_file_path)

    tree_lines = []
    tree_lines.append("```") # Start code block for plain text tree

    # Get immediate contents of the root directory
    # Filter out 'docs' folder from this initial listing if it's the root for the output
    root_items_raw = os.listdir(root_dir)
    root_items = sorted([item for item in root_items_raw if item not in excluded_items], key=str.lower)

    # Specific folders to drill into (core, mcp, server, shared)
    project_sub_folders_to_detail = ['core', 'mcp', 'server', 'shared']

    for item in root_items:
        full_path = os.path.join(root_dir, item)

        if os.path.isdir(full_path):
            if item in project_sub_folders_to_detail:
                tree_lines.append(f"📁 {item}/") # Main project folder
                sub_items = sorted(os.listdir(full_path), key=str.lower)
                for sub_item in sub_items:
                    sub_full_path = os.path.join(full_path, sub_item)

                    if sub_item in excluded_items:
                        continue # Skip excluded items even within sub-folders

                    if os.path.isdir(sub_full_path):
                        if sub_item == 'src': # Special treatment for 'src' folder
                            tree_lines.append(f"  ├── 📂 {sub_item}/") # src folder icon
                            src_items = sorted(os.listdir(sub_full_path), key=str.lower)
                            for src_item in src_items:
                                if src_item in excluded_items:
                                    continue
                                # Just list items in src, don't recurse further
                                if os.path.isdir(os.path.join(sub_full_path, src_item)):
                                     tree_lines.append(f"  │   ├── 📁 {src_item}/")
                                else:
                                     tree_lines.append(f"  │   ├── 📄 {src_item}")
                        else: # Other sub-folders of core/mcp/server/shared (not src)
                            tree_lines.append(f"  ├── 📁 {sub_item}/")
                    elif os.path.isfile(sub_full_path):
                        tree_lines.append(f"  ├── 📄 {sub_item}")
            else: # Other top-level directories (e.g., docs/, but excluded)
                tree_lines.append(f"📁 {item}/") # Default folder icon for others
        elif os.path.isfile(full_path):
            tree_lines.append(f"📄 {item}") # File icon for top-level files

    tree_lines.append("```") # End code block

    # Add a header for the section
    markdown_content = "## Project Layout\n\n"
    markdown_content += "Here's a simplified overview of the project's folder structure:\n\n"
    markdown_content += "\n".join(tree_lines) + "\n"

    try:
        with open(output_file_path, 'w', encoding='utf-8') as f:
            f.write(markdown_content)
        print(f"Successfully generated project layout in '{output_file_path}'")
        return True
    except Exception as e:
        print(f"Error writing to file '{output_file_path}': {e}", file=sys.stderr)
        return False

if __name__ == "__main__":
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.abspath(os.path.join(script_dir, os.pardir, os.pardir))

    output_file = os.path.join(project_root, "docs", "project_layout.md")
    ignore_file = os.path.join(script_dir, ".layoutIgnore") # This path should be correct now

    print(f"Generating simple tree for project root: '{project_root}'")
    print(f"Outputting to: '{output_file}'")
    print(f"Reading exclusions from: '{ignore_file}'")

    if not generate_simple_tree(project_root, output_file, ignore_file):
        sys.exit(1)
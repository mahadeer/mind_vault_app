# Generate by Google Gemini
import os
import re
import sys

def build_readme(template_path, output_path, include_dir):
    """
    Builds the README.md file by combining a template and included Markdown files.

    Args:
        template_path (str): The path to the README.md.template file, relative to where the script is run.
        output_path (str): The desired output path for the combined README.md, relative to where the script is run.
        include_dir (str): The directory containing the partial Markdown files to be included,
                           relative to where the script is run.
    """
    try:
        with open(template_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except FileNotFoundError:
        print(f"Error: Template file not found at '{template_path}'")
        return False
    except Exception as e:
        print(f"Error reading template file '{template_path}': {e}")
        return False

    pattern = re.compile(r'\{\{@import\s*(.*?)\s*\}\}')

    def replace_include(match):
        include_file_relative_to_include_dir = match.group(1)
        # Construct the full path to the included file based on the provided include_dir
        full_include_path = os.path.join(include_dir, include_file_relative_to_include_dir)
        try:
            with open(full_include_path, 'r', encoding='utf-8') as inc_f:
                return inc_f.read()
        except FileNotFoundError:
            error_msg = f"**ERROR: Could not find include file: '{full_include_path}' (referenced by '{match.group(0)}' in template)**"
            print(error_msg) # Print to console
            return error_msg # Also embed in output
        except Exception as e:
            error_msg = f"**ERROR: Could not read include file '{full_include_path}': {e} (referenced by '{match.group(0)}' in template)**"
            print(error_msg) # Print to console
            return error_msg # Also embed in output

    combined_content = pattern.sub(replace_include, content)

    try:
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write(combined_content)
        print(f"README built successfully: '{output_path}'")
        return True
    except Exception as e:
        print(f"Error writing output file '{output_path}': {e}")
        return False

if __name__ == "__main__":
    # Get the directory where the script itself is located
    # This is crucial for correctly resolving paths relative to the script's location
    script_dir = os.path.dirname(os.path.abspath(__file__))

    # Define paths relative to the project root
    # We assume this script is run from the project root.
    # So, paths are relative to the project root.

    # Path to the template file
    # If script is in docs/doc_builder, template is in docs/doc_builder
    template_file = os.path.join(script_dir, "README.md.template")

    # Path to the output README.md file (at the project root)
    # To go up two levels from script_dir to reach the project root.
    project_root = os.path.abspath(os.path.join(script_dir, os.pardir, os.pardir))
    output_file = os.path.join(project_root, "README.md")

    # Directory containing the partial markdown files (now in 'docs/')
    # This needs to be relative to the *project root* if the script is run from there.
    # OR, relative to the *script_dir* if the template specifies paths relative to its own location.
    # Given your image, the partial files (architecture_overview.md, features.md, etc.) are directly in 'docs/'
    # So, 'docs' directory itself is the include_dir.
    # From the script's perspective (in docs/doc_builder), 'docs/' is '../'
    # From the project root's perspective, 'docs/' is 'docs/'

    # The template file has `{{include: introduction.md}}` (etc.)
    # This means the included files are relative to the `include_dir` *parameter* passed to `build_readme`.
    # So, `include_dir` needs to be the `docs` folder path.
    include_sections_dir = os.path.join(project_root, "docs")

    print("--- Generating Project Layout ---")
    # Import and call the new script's function
    # We need to add the doc_builder directory to sys.path temporarily to import it
    sys.path.append(script_dir)
    from generate_project_layout import generate_simple_tree # Import the function

    # Determine project_root for generate_simple_tree to start from
    # This is the same project_root as calculated earlier
    project_layout_output_path = os.path.join(project_root, "docs", "project_layout.md")

    # Path to the .layoutIgnore file in the same directory as this script
    ignore_file = os.path.join(script_dir, ".layoutIgnore")

    if not generate_simple_tree(project_root, project_layout_output_path, ignore_file):
        print("Failed to generate project layout. Aborting README build.")
        sys.exit(1)
    sys.path.remove(script_dir) # Clean up sys.path

    print("\n--- Building Main README ---")
    print(f"Building README from template: {template_file}")
    print(f"Outputting to: {output_file}")
    print(f"Including sections from: {include_sections_dir}")

    if not build_readme(template_file, output_file, include_sections_dir):
        print("README build failed.")
        sys.exit(1)
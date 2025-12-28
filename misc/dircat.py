#!/usr/bin/env python3
"""
Directory content concatenator - recursively cats files matching glob patterns
Usage: dircat.py <directory> <patterns> [--exclude <pattern>...]
Example: dircat.py ~/dev/projecta "*.py,*.md,*.html" --exclude .git --exclude node_modules
"""

import sys
import os
import argparse
from pathlib import Path
import fnmatch

def get_language_hint(filename):
    """Map file extensions to markdown code block language hints"""
    ext_map = {
        '.py': 'python',
        '.js': 'javascript',
        '.ts': 'typescript',
        '.jsx': 'jsx',
        '.tsx': 'tsx',
        '.java': 'java',
        '.c': 'c',
        '.cpp': 'cpp',
        '.cs': 'csharp',
        '.php': 'php',
        '.rb': 'ruby',
        '.go': 'go',
        '.rs': 'rust',
        '.kt': 'kotlin',
        '.swift': 'swift',
        '.m': 'objectivec',
        '.scala': 'scala',
        '.sh': 'bash',
        '.bash': 'bash',
        '.zsh': 'zsh',
        '.fish': 'fish',
        '.ps1': 'powershell',
        '.r': 'r',
        '.R': 'r',
        '.sql': 'sql',
        '.html': 'html',
        '.htm': 'html',
        '.xml': 'xml',
        '.css': 'css',
        '.scss': 'scss',
        '.sass': 'sass',
        '.less': 'less',
        '.json': 'json',
        '.yaml': 'yaml',
        '.yml': 'yaml',
        '.toml': 'toml',
        '.ini': 'ini',
        '.cfg': 'ini',
        '.conf': 'conf',
        '.md': 'markdown',
        '.markdown': 'markdown',
        '.rst': 'rst',
        '.tex': 'latex',
    }

    ext = Path(filename).suffix.lower()
    return ext_map.get(ext, '')

def matches_any(text, patterns):
    """Check if text matches any of the glob patterns"""
    for pattern in patterns:
        if fnmatch.fnmatch(text, pattern):
            return True
    return False

def process_directory(directory, include_patterns, exclude_patterns=None):
    """Walk directory and output matching files in markdown format"""
    if exclude_patterns is None:
        exclude_patterns = []

    base_path = Path(directory).resolve()
    files_found = []

    # Collect all matching files
    for root, dirs, files in os.walk(base_path):
        root_path = Path(root)

        # 1. Prune directories based on exclude patterns
        # We modify 'dirs' in-place to prevent os.walk from descending into them
        dirs_to_remove = []
        for d in dirs:
            dir_full_path = root_path / d
            try:
                rel_path = str(dir_full_path.relative_to(base_path))
            except ValueError:
                rel_path = d # Fallback

            # Check if directory name OR relative path matches exclude patterns
            # This handles both "node_modules" and "src/temp" style exclusions
            if d.startswith('.') or \
               matches_any(d, exclude_patterns) or \
               matches_any(rel_path, exclude_patterns) or \
               matches_any(f"./{rel_path}", exclude_patterns): # Handle ./tools syntax
                dirs_to_remove.append(d)

        for d in dirs_to_remove:
            dirs.remove(d)

        # 2. Process Files
        for file in files:
            full_path = root_path / file
            rel_path_obj = full_path.relative_to(base_path)
            rel_path_str = str(rel_path_obj)

            # Check inclusions
            if matches_any(file, include_patterns):
                # Check exclusions (files can be excluded specifically too)
                if not (matches_any(file, exclude_patterns) or
                        matches_any(rel_path_str, exclude_patterns)):
                    files_found.append((full_path, rel_path_obj))

    # Sort files by relative path for consistent output
    files_found.sort(key=lambda x: x[1])

    # Process and output each file
    for i, (full_path, rel_path) in enumerate(files_found):
        try:
            with open(full_path, 'r', encoding='utf-8', errors='replace') as f:
                content = f.read()

            # Output markdown formatted content
            print(f"### {rel_path}")
            print()

            lang_hint = get_language_hint(str(rel_path))
            print(f"```{lang_hint}")
            print(content)
            if not content.endswith('\n'):
                print()  # Ensure there's a newline before closing
            print("```")

            # Add separator unless it's the last file
            if i < len(files_found) - 1:
                print()
                print("---")
                print()

        except Exception as e:
            print(f"Error reading {rel_path}: {e}", file=sys.stderr)
            print(f"```")
            print(f"[Error reading file: {e}]")
            print(f"```")
            if i < len(files_found) - 1:
                print()
                print("---")
                print()

def main():
    parser = argparse.ArgumentParser(
        description="Recursively concatenates files matching glob patterns into a single Markdown output."
    )

    parser.add_argument(
        "directory",
        help="The base directory to search in"
    )

    parser.add_argument(
        "patterns",
        help="Comma-separated list of file patterns to include (e.g. '*.py,*.md')"
    )

    parser.add_argument(
        "--exclude",
        action="append",
        default=[],
        help="Pattern to exclude (can be used multiple times). Matches directory names or relative paths."
    )

    args = parser.parse_args()

    # Expand home directory if present
    directory = os.path.expanduser(args.directory)

    if not os.path.isdir(directory):
        print(f"Error: {directory} is not a valid directory", file=sys.stderr)
        sys.exit(1)

    # Split patterns by comma and strip whitespace
    patterns = [p.strip() for p in args.patterns.split(',')]

    if not patterns or all(not p for p in patterns):
        print("Error: No patterns specified", file=sys.stderr)
        sys.exit(1)

    process_directory(directory, patterns, args.exclude)

if __name__ == "__main__":
    main()

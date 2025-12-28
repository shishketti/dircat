use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};

/// Maps file extensions to Markdown code block language hints
fn get_language_hint(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
    {
        Some(ext) => match ext.as_str() {
            "py" => "python",
            "js" => "javascript",
            "ts" => "typescript",
            "jsx" => "jsx",
            "tsx" => "tsx",
            "java" => "java",
            "c" => "c",
            "cpp" => "cpp",
            "cs" => "csharp",
            "php" => "php",
            "rb" => "ruby",
            "go" => "go",
            "rs" => "rust",
            "kt" => "kotlin",
            "swift" => "swift",
            "m" => "objectivec",
            "scala" => "scala",
            "sh" => "bash",
            "bash" => "bash",
            "zsh" => "zsh",
            "fish" => "fish",
            "ps1" => "powershell",
            "r" => "r",
            "sql" => "sql",
            "html" | "htm" => "html",
            "xml" => "xml",
            "css" => "css",
            "scss" => "scss",
            "sass" => "sass",
            "less" => "less",
            "json" => "json",
            "yaml" | "yml" => "yaml",
            "toml" => "toml",
            "ini" | "cfg" => "ini",
            "conf" => "conf",
            "md" | "markdown" => "markdown",
            "rst" => "rst",
            "tex" => "latex",
            _ => "",
        },
        None => "",
    }
}

/// Build a GlobSet from string patterns
fn build_globset(patterns: &[String]) -> Result<GlobSet, globset::Error> {
    let mut builder = GlobSetBuilder::new();
    for pat in patterns {
        builder.add(Glob::new(pat)?);
    }
    builder.build()
}

/// Decide whether to prune a directory (skip recursion)
fn should_prune_dir(entry: &DirEntry, base: &Path, exclude: &GlobSet) -> bool {
    let name = entry.file_name();

    // Skip dot-directories
    if name.to_string_lossy().starts_with('.') {
        return true;
    }

    let full_path = entry.path();

    let rel_path = match full_path.strip_prefix(base) {
        Ok(p) => p,
        Err(_) => return false,
    };

    let dot_rel = PathBuf::from(".").join(rel_path);

    exclude.is_match(name)
        || exclude.is_match(rel_path)
        || exclude.is_match(&dot_rel)
}

/// Collect matching files
fn collect_files(
    base_dir: &Path,
    include: &GlobSet,
    exclude: &GlobSet,
) -> Vec<(PathBuf, PathBuf)> {
    let mut results = Vec::new();

    let walker = WalkDir::new(base_dir).into_iter().filter_entry(|e| {
        if e.file_type().is_dir() {
            !should_prune_dir(e, base_dir, exclude)
        } else {
            true
        }
    });

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let full_path = entry.path();

        let rel_path = match full_path.strip_prefix(base_dir) {
            Ok(p) => p.to_path_buf(),
            Err(_) => continue,
        };

        let file_name = entry.file_name();

        if include.is_match(file_name)
            && !exclude.is_match(file_name)
            && !exclude.is_match(&rel_path)
        {
            results.push((full_path.to_path_buf(), rel_path));
        }
    }

    results.sort_by(|a, b| a.1.cmp(&b.1));
    results
}

/// Output Markdown
fn output_markdown(files: &[(PathBuf, PathBuf)]) {
    let mut stdout = io::stdout();

    for (i, (full_path, rel_path)) in files.iter().enumerate() {
        writeln!(stdout, "### {}", rel_path.display()).ok();
        writeln!(stdout).ok();

        let lang = get_language_hint(rel_path);
        writeln!(stdout, "```{}", lang).ok();

        match fs::read_to_string(full_path) {
            Ok(content) => {
                write!(stdout, "{}", content).ok();
                if !content.ends_with('\n') {
                    writeln!(stdout).ok();
                }
            }
            Err(err) => {
                eprintln!("Error reading {}: {}", rel_path.display(), err);
                writeln!(stdout, "[Error reading file: {}]", err).ok();
            }
        }

        writeln!(stdout, "```").ok();

        if i + 1 < files.len() {
            writeln!(stdout).ok();
            writeln!(stdout, "---").ok();
            writeln!(stdout).ok();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: dircat <directory> <patterns> [--exclude <pattern>...]");
        std::process::exit(1);
    }

    let base_dir = PathBuf::from(&args[1]);
    if !base_dir.is_dir() {
        eprintln!("Error: {} is not a directory", base_dir.display());
        std::process::exit(1);
    }

    let include_patterns: Vec<String> = args[2]
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if include_patterns.is_empty() {
        eprintln!("Error: no include patterns specified");
        std::process::exit(1);
    }

    let mut exclude_patterns = Vec::new();
    let mut i = 3;
    while i < args.len() {
        if args[i] == "--exclude" {
            if i + 1 >= args.len() {
                eprintln!("Error: --exclude requires a pattern");
                std::process::exit(1);
            }
            exclude_patterns.push(args[i + 1].clone());
            i += 2;
        } else {
            eprintln!("Unknown argument: {}", args[i]);
            std::process::exit(1);
        }
    }

    let include_glob = match build_globset(&include_patterns) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Invalid include pattern: {}", e);
            std::process::exit(1);
        }
    };

    let exclude_glob = match build_globset(&exclude_patterns) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Invalid exclude pattern: {}", e);
            std::process::exit(1);
        }
    };

    let files = collect_files(&base_dir, &include_glob, &exclude_glob);
    output_markdown(&files);
}


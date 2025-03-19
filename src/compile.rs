// Basic types needed for TypeScript compilation

use crate::cli::*;

#[derive(Clone)]
pub struct SourceFile {
    pub file_name: String,
    pub text: String,
    pub line_map: Vec<usize>, // Line start positions for error reporting
}

// Compilation result diagnostics
#[derive(Debug)]
pub(crate) struct Diagnostic {
    pub(crate) file_name: Option<String>,
    pub(crate) line: usize,
    pub(crate) character: usize,
    pub(crate) message: String,
    pub(crate) code: u32,
    pub(crate) category: DiagnosticCategory,
}

#[derive(Debug)]
pub(crate) enum DiagnosticCategory {
    Error,
    Warning,
    Suggestion,
    Message,
}

// Program represents the entire TypeScript program being compiled
pub(crate) struct Program {
    pub(crate) source_files: Vec<SourceFile>,
    pub(crate) diagnostics: Vec<Diagnostic>,
    // Will eventually contain more state like:
    // - Symbol tables
    // - Type checker results
    // - etc.
}

// Abstraction for file system operations
pub trait CompilerHost {
    fn read_file(&self, path: &str) -> Option<String>;
    fn write_file(&self, path: &str, data: &str) -> bool;
    fn file_exists(&self, path: &str) -> bool;
    fn get_current_directory(&self) -> String;
    // Additional filesystem operations as needed
}

// Implement a basic filesystem-based compiler host
struct FileSystemCompilerHost;

impl CompilerHost for FileSystemCompilerHost {
    fn read_file(&self, path: &str) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    fn write_file(&self, path: &str, data: &str) -> bool {
        if let Some(parent) = std::path::Path::new(path).parent() {
            if let Err(_) = std::fs::create_dir_all(parent) {
                return false;
            }
        }
        std::fs::write(path, data).is_ok()
    }

    fn file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn get_current_directory(&self) -> String {
        std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| ".".to_string())
    }
}

pub fn create_compiler_host() -> impl CompilerHost {
    FileSystemCompilerHost
}

pub fn read_source_files(file_paths: &[String], host: &impl CompilerHost) -> Vec<SourceFile> {
    file_paths
        .iter()
        .filter_map(|path| {
            let text = host.read_file(path)?;
            let line_map = compute_line_map(&text);

            Some(SourceFile {
                file_name: path.clone(),
                text,
                line_map,
            })
        })
        .collect()
}

// Compute line start positions for error reporting
pub fn compute_line_map(text: &str) -> Vec<usize> {
    let mut positions = vec![0];
    for (i, c) in text.char_indices() {
        if c == '\n' {
            positions.push(i + 1);
        }
    }
    positions
}

pub fn create_program(
    source_files: &[SourceFile],
    compiler_options: &CompilerOptions,
    host: &impl CompilerHost,
) -> Program {
    // In a real implementation, this would parse files, create AST, etc.
    Program {
        source_files: source_files.to_vec(),
        diagnostics: Vec::new(),
    }
}

pub fn type_check(program: &mut Program) {
    // In a real implementation, this would perform type checking
    // and populate program.diagnostics with any type errors
    println!("Type checking...");
}

pub fn emit_files(program: &Program, options: &CompilerOptions, host: &impl CompilerHost) {
    // In a real implementation, this would output JavaScript files,
    // declaration files, and source maps based on compiler options

    if let Some(out_dir) = &options.out_dir {
        println!("Emitting files to: {}", out_dir);

        for source_file in &program.source_files {
            let base_name = std::path::Path::new(&source_file.file_name)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            let js_path = format!("{}/{}.js", out_dir, base_name);

            // This is where we'd emit transformed JavaScript
            let js_content = "console.log('Hello from TypeScript!');\n";
            host.write_file(&js_path, js_content);

            if options.declaration {
                let dts_path = format!("{}/{}.d.ts", out_dir, base_name);
                let dts_content = "// Type definitions\nexport {};\n";
                host.write_file(&dts_path, dts_content);
            }
        }
    }
}

pub fn report_diagnostics(diagnostics: &[Diagnostic], pretty: bool) {
    if diagnostics.is_empty() {
        println!("Compilation completed successfully.");
        return;
    }

    let mut error_count = 0;
    let mut warning_count = 0;

    for diagnostic in diagnostics {
        match diagnostic.category {
            DiagnosticCategory::Error => {
                error_count += 1;
                print_diagnostic(diagnostic, pretty);
            }
            DiagnosticCategory::Warning => {
                warning_count += 1;
                print_diagnostic(diagnostic, pretty);
            }
            _ => {}
        }
    }

    println!(
        "Found {} error(s), {} warning(s)",
        error_count, warning_count
    );
}

pub fn print_diagnostic(diagnostic: &Diagnostic, pretty: bool) {
    let color_start = if pretty { "\x1b[31m" } else { "" };
    let color_end = if pretty { "\x1b[0m" } else { "" };

    if let Some(file_name) = &diagnostic.file_name {
        println!(
            "{}{}({}:{}) - error TS{}: {}{}",
            color_start,
            file_name,
            diagnostic.line,
            diagnostic.character,
            diagnostic.code,
            diagnostic.message,
            color_end
        );
    } else {
        println!(
            "{}error TS{}: {}{}",
            color_start, diagnostic.code, diagnostic.message, color_end
        );
    }
}

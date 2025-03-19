mod cli;
mod compile;

use clap::Parser;

use crate::cli::*;
use crate::compile::*;

fn main() {
    // Parse the CLI args
    let cli = Cli::parse();

    // Handle command dispatch based on args
    if cli.help {
        print_help(cli.all);
    } else if cli.version {
        print_version();
    } else if cli.init {
        create_tsrsonfig();
    } else if cli.build {
        build_project(&cli);
    } else if cli.show_config {
        show_config(&cli);
    } else if !cli.files.is_empty() {
        compile_files(&cli);
    } else if let Some(project) = &cli.project {
        compile_project(&cli);
    } else {
        compile_current_project(&cli);
    }
}

fn print_version() {
    println!("Version 5.8.2");
}

fn create_tsrsonfig() {
    println!("Creating tsrsonfig.json");
    // Implementation for creating tsrsonfig.json
}

fn build_project(cli: &Cli) {
    println!("Building project");
    // Implementation for building project
}

fn show_config(cli: &Cli) {
    println!("Showing configuration");
    // Implementation for showing configuration
}

fn compile_files(cli: &Cli) {
    println!("Compiling files: {:?}", cli.files);
    // 1. Set up compiler options from CLI arguments
    let compiler_options = create_compiler_options(cli);

    // 2. Create a compiler host (filesystem abstraction)
    let host = create_compiler_host();

    // 3. Read and parse the input files
    let source_files = read_source_files(&cli.files, &host);

    // 4. Initialize the compilation process
    let mut program = create_program(&source_files, &compiler_options, &host);

    // 5. Perform type checking if needed
    if !compiler_options.skip_type_checking {
        type_check(&mut program);
    }

    // 6. Emit the output files (JS, declaration files, sourcemaps)
    if !compiler_options.no_emit {
        emit_files(&program, &compiler_options, &host);
    }

    // 7. Report any diagnostics
    report_diagnostics(&program.diagnostics, compiler_options.pretty);
}

fn compile_project(cli: &Cli) {
    println!("Compiling project");
    // Implementation for compiling current project
}

fn compile_current_project(cli: &Cli) {
    println!("Compiling current project");
    // Implementation for compiling current project
}

// use clap::{Args, Parser, Subcommand};
// use std::path::PathBuf;

// /// A TypeScript compiler implementation in Rust
// #[derive(Parser)]
// #[command(name = "tsrs")]
// #[command(author, version, about, long_about = None)]
// pub struct Cli {
//     #[command(subcommand)]
//     command: Option<Commands>,

//     #[command(flatten)]
//     tsrs_options: tsrsOptions,

//     #[command(flatten)]
//     devel_options: DevelOptions,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Run the TypeScript compiler
//     tsrs {
//         /// Arguments to pass to the TypeScript compiler
//         #[arg(trailing_var_arg = true)]
//         args: Vec<String>,
//     },

//     /// Run the Language Server Protocol server
//     Lsp {
//         /// Arguments to pass to the LSP server
//         #[arg(trailing_var_arg = true)]
//         args: Vec<String>,
//     },
// }

// /// TypeScript compiler options
// #[derive(Args)]
// struct tsrsOptions {
//     /// Compile the project given the path to its configuration file or to a folder with a tsrsonfig.json
//     #[arg(short = 'p', long = "project")]
//     project: Option<PathBuf>,

//     /// Specify an output folder for all emitted files
//     #[arg(long = "outDir")]
//     out_dir: Option<PathBuf>,

//     /// Disable emitting files from a compilation
//     #[arg(long = "noEmit")]
//     no_emit: Option<bool>,

//     /// Disable including any library files, including the default lib.d.ts
//     #[arg(long = "noLib")]
//     no_lib: Option<bool>,

//     /// Disable full type checking, only critical parse and emit errors will be reported
//     #[arg(long = "noCheck")]
//     no_check: Option<bool>,

//     /// Skip type checking all .d.ts files
//     #[arg(long = "skipLibCheck")]
//     skip_lib_check: Option<bool>,

//     /// Enable color and formatting in TypeScript's output to make compiler errors easier to read
//     #[arg(long = "pretty", default_value_t = true)]
//     pretty: bool,

//     /// Print all of the files read during the compilation
//     #[arg(long = "listFiles")]
//     list_files: Option<bool>,

//     /// Print names of files that are part of the compilation and then stop processing
//     #[arg(long = "listFilesOnly")]
//     list_files_only: Option<bool>,

//     /// Print the final configuration instead of building
//     #[arg(long = "showConfig")]
//     show_config: Option<bool>,
// }

// /// Development options
// #[derive(Args)]
// struct DevelOptions {
//     /// Do not print diagnostics
//     #[arg(short = 'q', long = "quiet")]
//     quiet: bool,

//     /// Run in single threaded mode
//     #[arg(long = "singleThreaded")]
//     single_threaded: bool,

//     /// Print types defined in 'main.ts'
//     #[arg(long = "printTypes")]
//     print_types: bool,

//     /// Generate pprof CPU/memory profiles to the given directory
//     #[arg(long = "pprofDir")]
//     pprof_dir: Option<PathBuf>,
// }

// impl tsrsOptions {
//     /// Convert to compiler options with resolved paths
//     pub fn to_compiler_options(&self, current_directory: &PathBuf) -> CompilerOptions {
//         let mut options = CompilerOptions {
//             no_emit: self
//                 .no_emit
//                 .map(Tristate::from_bool)
//                 .unwrap_or(Tristate::Unknown),
//             no_lib: self
//                 .no_lib
//                 .map(Tristate::from_bool)
//                 .unwrap_or(Tristate::Unknown),
//             no_check: self
//                 .no_check
//                 .map(Tristate::from_bool)
//                 .unwrap_or(Tristate::Unknown),
//             skip_lib_check: self
//                 .skip_lib_check
//                 .map(Tristate::from_bool)
//                 .unwrap_or(Tristate::Unknown),
//             pretty: if self.pretty {
//                 Tristate::True
//             } else {
//                 Tristate::False
//             },
//             list_files: self
//                 .list_files
//                 .map(Tristate::from_bool)
//                 .unwrap_or(Tristate::Unknown),
//             list_files_only: self
//                 .list_files_only
//                 .map(Tristate::from_bool)
//                 .unwrap_or(Tristate::Unknown),
//             show_config: self
//                 .show_config
//                 .map(Tristate::from_bool)
//                 .unwrap_or(Tristate::Unknown),
//             out_dir: None,
//         };

//         if let Some(out_dir) = &self.out_dir {
//             options.out_dir = Some(resolve_path(current_directory, out_dir));
//         }

//         options
//     }
// }

// /// Represents a three-state value (true, false, or unknown)
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum Tristate {
//     True,
//     False,
//     Unknown,
// }

// impl Tristate {
//     fn from_bool(value: bool) -> Self {
//         if value {
//             Tristate::True
//         } else {
//             Tristate::False
//         }
//     }

//     pub fn is_true_or_unknown(&self) -> bool {
//         matches!(self, Tristate::True | Tristate::Unknown)
//     }

//     pub fn is_false_or_unknown(&self) -> bool {
//         matches!(self, Tristate::False | Tristate::Unknown)
//     }

//     pub fn is_true(&self) -> bool {
//         matches!(self, Tristate::True)
//     }
// }

// /// TypeScript compiler options
// #[derive(Debug)]
// pub struct CompilerOptions {
//     pub no_emit: Tristate,
//     pub no_lib: Tristate,
//     pub no_check: Tristate,
//     pub skip_lib_check: Tristate,
//     pub pretty: Tristate,
//     pub list_files: Tristate,
//     pub list_files_only: Tristate,
//     pub show_config: Tristate,
//     pub out_dir: Option<PathBuf>,
// }

// fn resolve_path(base: &PathBuf, path: &PathBuf) -> PathBuf {
//     if path.is_absolute() {
//         path.clone()
//     } else {
//         base.join(path)
//     }
// }

// fn main() {
//     let cli = Cli::parse();

//     match cli.command {
//         Some(Commands::tsrs { args }) => {
//             // Handle TypeScript compiler command with args
//             println!("Running tsrs with args: {:?}", args);
//             // Equivalent to execute.CommandLine(newSystem(), nil, args[1:])
//         }
//         Some(Commands::Lsp { args }) => {
//             // Handle LSP command
//             println!("Running LSP server with args: {:?}", args);
//             // Equivalent to runLSP(args[1:])
//         }
//         None => {
//             // Process main compiler functionality using cli.tsrs_options and cli.devel_options
//             println!("Running TypeScript compiler with standard options");
//             // Implementation of the main TypeScript compilation logic
//         }
//     }
// }

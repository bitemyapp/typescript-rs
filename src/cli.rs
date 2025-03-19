use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// The TypeScript Compiler
#[derive(Parser)]
#[command(name = "tsrs")]
#[command(version = "Version 0.0.1")]
#[command(about = "The TypeScript Compiler...in Rust!", long_about = None)]
#[command(after_help = "You can learn about all of the compiler options at https://aka.ms/tsrs")]
#[command(disable_help_flag = true)]
#[command(disable_version_flag = true)]
pub struct Cli {
    /// Print this message.
    #[arg(short = 'h', long = "help")]
    pub help: bool,

    /// Watch input files.
    #[arg(short = 'w', long = "watch")]
    pub watch: bool,

    /// Show all compiler options.
    #[arg(long = "all")]
    pub all: bool,

    /// Print the compiler's version.
    #[arg(short = 'v', long = "version")]
    pub version: bool,

    /// Initializes a TypeScript project and creates a tsrsonfig.json file.
    #[arg(long = "init")]
    pub init: bool,

    /// Compile the project given the path to its configuration file, or to a folder with a 'tsrsonfig.json'.
    #[arg(short = 'p', long = "project")]
    pub project: Option<PathBuf>,

    /// Print the final configuration instead of building.
    #[arg(long = "showConfig")]
    pub show_config: bool,

    /// Build one or more projects and their dependencies, if out of date
    #[arg(short = 'b', long = "build")]
    pub build: bool,

    // COMMON COMPILER OPTIONS
    /// Enable color and formatting in TypeScript's output to make compiler errors easier to read.
    #[arg(long = "pretty", default_value_t = true)]
    pub pretty: bool,

    /// Generate .d.ts files from TypeScript and JavaScript files in your project.
    #[arg(short = 'd', long = "declaration")]
    pub declaration: bool,

    /// Create sourcemaps for d.ts files.
    #[arg(long = "declarationMap")]
    pub declaration_map: bool,

    /// Only output d.ts files and not JavaScript files.
    #[arg(long = "emitDeclarationOnly")]
    pub emit_declaration_only: bool,

    /// Create source map files for emitted JavaScript files.
    #[arg(long = "sourceMap")]
    pub source_map: bool,

    /// Disable emitting files from a compilation.
    #[arg(long = "noEmit")]
    pub no_emit: bool,

    /// Set the JavaScript language version for emitted JavaScript and include compatible library declarations.
    #[arg(short = 't', long = "target", value_enum)]
    pub target: Option<Target>,

    /// Specify what module code is generated.
    #[arg(short = 'm', long = "module", value_enum)]
    pub module: Option<Module>,

    /// Specify a set of bundled library declaration files that describe the target runtime environment.
    #[arg(long = "lib")]
    pub lib: Vec<String>,

    /// Allow JavaScript files to be a part of your program. Use the 'checkJS' option to get errors from these files.
    #[arg(long = "allowJs")]
    pub allow_js: bool,

    /// Enable error reporting in type-checked JavaScript files.
    #[arg(long = "checkJs")]
    pub check_js: bool,

    /// Specify what JSX code is generated.
    #[arg(long = "jsx", value_enum)]
    pub jsx: Option<JsxMode>,

    /// Specify a file that bundles all outputs into one JavaScript file.
    /// If 'declaration' is true, also designates a file that bundles all .d.ts output.
    #[arg(long = "outFile")]
    pub out_file: Option<PathBuf>,

    /// Specify an output folder for all emitted files.
    #[arg(long = "outDir")]
    pub out_dir: Option<PathBuf>,

    /// Disable emitting comments.
    #[arg(long = "removeComments")]
    pub remove_comments: bool,

    /// Enable all strict type-checking options.
    #[arg(long = "strict")]
    pub strict: bool,

    /// Specify type package names to be included without being referenced in a source file.
    #[arg(long = "types")]
    pub types: Vec<String>,

    /// Emit additional JavaScript to ease support for importing CommonJS modules.
    /// This enables 'allowSyntheticDefaultImports' for type compatibility.
    #[arg(long = "esModuleInterop")]
    pub es_module_interop: bool,

    /// Input files to compile
    #[arg(value_name = "FILES")]
    pub files: Vec<String>,
}

// #[derive(Parser)]
// #[command(name = "tsrs")]
// #[command(version = "Version 0.0.1")]
// #[command(about = "The TypeScript Compiler...in Rust!", long_about = None)]
// #[command(after_help = "You can learn about all of the compiler options at https://aka.ms/tsrs")]
// #[command(disable_help_flag = true)]
// #[command(disable_version_flag = true)]
// pub struct Cli {
//     /// Print this message.
//     #[arg(short = 'h', long = "help")]
//     help: bool,

//     /// Watch input files.
//     #[arg(short = 'w', long = "watch")]
//     watch: bool,

//     /// Show all compiler options.
//     #[arg(long = "all")]
//     all: bool,

//     /// Print the compiler's version.
//     #[arg(short = 'v', long = "version")]
//     version: bool,

//     /// Initializes a TypeScript project and creates a tsrsonfig.json file.
//     #[arg(long = "init")]
//     init: bool,

//     /// Compile the project given the path to its configuration file, or to a folder with a 'tsrsonfig.json'.
//     #[arg(short = 'p', long = "project")]
//     project: Option<PathBuf>,

//     /// Print the final configuration instead of building.
//     #[arg(long = "showConfig")]
//     show_config: bool,

//     /// Build one or more projects and their dependencies, if out of date
//     #[arg(short = 'b', long = "build")]
//     build: bool,

//     // COMMON COMPILER OPTIONS
//     /// Enable color and formatting in TypeScript's output to make compiler errors easier to read.
//     #[arg(long = "pretty", default_value_t = true)]
//     pretty: bool,

//     /// Generate .d.ts files from TypeScript and JavaScript files in your project.
//     #[arg(short = 'd', long = "declaration")]
//     declaration: bool,

//     /// Create sourcemaps for d.ts files.
//     #[arg(long = "declarationMap")]
//     declaration_map: bool,

//     /// Only output d.ts files and not JavaScript files.
//     #[arg(long = "emitDeclarationOnly")]
//     emit_declaration_only: bool,

//     /// Create source map files for emitted JavaScript files.
//     #[arg(long = "sourceMap")]
//     source_map: bool,

//     /// Disable emitting files from a compilation.
//     #[arg(long = "noEmit")]
//     no_emit: bool,

//     /// Set the JavaScript language version for emitted JavaScript and include compatible library declarations.
//     #[arg(short = 't', long = "target", value_enum)]
//     target: Option<Target>,

//     /// Specify what module code is generated.
//     #[arg(short = 'm', long = "module", value_enum)]
//     module: Option<Module>,

//     /// Specify a set of bundled library declaration files that describe the target runtime environment.
//     #[arg(long = "lib")]
//     lib: Vec<String>,

//     /// Allow JavaScript files to be a part of your program. Use the 'checkJS' option to get errors from these files.
//     #[arg(long = "allowJs")]
//     allow_js: bool,

//     /// Enable error reporting in type-checked JavaScript files.
//     #[arg(long = "checkJs")]
//     check_js: bool,

//     /// Specify what JSX code is generated.
//     #[arg(long = "jsx", value_enum)]
//     jsx: Option<JsxMode>,

//     /// Specify a file that bundles all outputs into one JavaScript file.
//     /// If 'declaration' is true, also designates a file that bundles all .d.ts output.
//     #[arg(long = "outFile")]
//     out_file: Option<PathBuf>,

//     /// Specify an output folder for all emitted files.
//     #[arg(long = "outDir")]
//     out_dir: Option<PathBuf>,

//     /// Disable emitting comments.
//     #[arg(long = "removeComments")]
//     remove_comments: bool,

//     /// Enable all strict type-checking options.
//     #[arg(long = "strict")]
//     strict: bool,

//     /// Specify type package names to be included without being referenced in a source file.
//     #[arg(long = "types")]
//     types: Vec<String>,

//     /// Emit additional JavaScript to ease support for importing CommonJS modules.
//     /// This enables 'allowSyntheticDefaultImports' for type compatibility.
//     #[arg(long = "esModuleInterop")]
//     es_module_interop: bool,

//     /// Input files to compile
//     #[arg(value_name = "FILES")]
//     files: Vec<String>,
// }

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Target {
    #[value(name = "es5")]
    Es5,
    #[value(name = "es6")]
    Es2015,
    #[value(name = "es2016")]
    Es2016,
    #[value(name = "es2017")]
    Es2017,
    #[value(name = "es2018")]
    Es2018,
    #[value(name = "es2019")]
    Es2019,
    #[value(name = "es2020")]
    Es2020,
    #[value(name = "es2021")]
    Es2021,
    #[value(name = "es2022")]
    Es2022,
    #[value(name = "es2023")]
    Es2023,
    #[value(name = "es2024")]
    Es2024,
    #[value(name = "esnext")]
    EsNext,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Module {
    #[value(name = "none")]
    None,
    #[value(name = "commonjs")]
    CommonJs,
    #[value(name = "amd")]
    Amd,
    #[value(name = "umd")]
    Umd,
    #[value(name = "system")]
    System,
    #[value(name = "es6")]
    Es2015,
    #[value(name = "es2020")]
    Es2020,
    #[value(name = "es2022")]
    Es2022,
    #[value(name = "esnext")]
    EsNext,
    #[value(name = "node16")]
    Node16,
    #[value(name = "node18")]
    Node18,
    #[value(name = "nodenext")]
    NodeNext,
    #[value(name = "preserve")]
    Preserve,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum JsxMode {
    #[value(name = "preserve")]
    Preserve,
    #[value(name = "react")]
    React,
    #[value(name = "react-native")]
    ReactNative,
    #[value(name = "react-jsx")]
    ReactJsx,
    #[value(name = "react-jsxdev")]
    ReactJsxDev,
}

// Compiler options derived from CLI arguments
pub struct CompilerOptions {
    pub target: String, // ES5, ES2015, etc.
    pub module: String, // CommonJS, ESNext, etc.
    pub source_map: bool,
    pub declaration: bool,
    pub out_dir: Option<String>,
    pub no_emit: bool,
    pub skip_type_checking: bool,
    pub pretty: bool,
    // Additional options as needed
}

pub fn create_compiler_options(cli: &Cli) -> CompilerOptions {
    CompilerOptions {
        target: match cli.target {
            Some(Target::Es5) => "ES5".to_string(),
            Some(Target::Es2015) => "ES2015".to_string(),
            Some(Target::Es2016) => "ES2016".to_string(),
            // Add other targets
            _ => "ES2022".to_string(), // Default
        },
        module: match cli.module {
            Some(Module::CommonJs) => "CommonJS".to_string(),
            Some(Module::Es2015) => "ES2015".to_string(),
            // Add other module types
            _ => "ESNext".to_string(), // Default
        },
        source_map: cli.source_map,
        declaration: cli.declaration,
        out_dir: cli
            .out_dir
            .as_ref()
            .map(|p| p.to_string_lossy().to_string()),
        no_emit: cli.no_emit,
        skip_type_checking: false, // Implement based on cli options
        pretty: cli.pretty,
    }
}

pub fn print_help(all: bool) {
    println!("tsrs: The TypeScript Compiler - Version 5.8.2");
    println!(
        "                                                                                                               TS "
    );
    println!("COMMON COMMANDS");
    println!();
    println!("  tsrs");
    println!("  Compiles the current project (tsrsonfig.json in the working directory.)");
    println!();
    println!("  tsrs app.ts util.ts");
    println!(
        "  Ignoring tsrsonfig.json, compiles the specified files with default compiler options."
    );
    println!();
    println!("  tsrs -b");
    println!("  Build a composite project in the working directory.");
    println!();
    println!("  tsrs --init");
    println!("  Creates a tsrsonfig.json with the recommended settings in the working directory.");
    println!();
    println!("  tsrs -p ./path/to/tsrsonfig.json");
    println!("  Compiles the TypeScript project located at the specified path.");
    println!();
    println!("  tsrs --help --all");
    println!("  An expanded version of this information, showing all possible compiler options");
    println!();
    println!("  tsrs --noEmit");
    println!("  tsrs --target esnext");
    println!("  Compiles the current project, with additional settings.");
    println!();

    println!("COMMAND LINE FLAGS");
    println!();
    println!("     --help, -h  Print this message.");
    println!();
    println!("    --watch, -w  Watch input files.");
    println!();
    println!("          --all  Show all compiler options.");
    println!();
    println!("  --version, -v  Print the compiler's version.");
    println!();
    println!(
        "         --init  Initializes a TypeScript project and creates a tsrsonfig.json file."
    );
    println!();
    println!(
        "  --project, -p  Compile the project given the path to its configuration file, or to a folder with a 'tsrsonfig.json'."
    );
    println!();
    println!("   --showConfig  Print the final configuration instead of building.");
    println!();
    println!("    --build, -b  Build one or more projects and their dependencies, if out of date");
    println!();

    println!("COMMON COMPILER OPTIONS");
    println!();
    println!(
        "               --pretty  Enable color and formatting in TypeScript's output to make compiler errors easier to read."
    );
    println!("                  type:  boolean");
    println!("               default:  true");
    println!();
    println!(
        "      --declaration, -d  Generate .d.ts files from TypeScript and JavaScript files in your project."
    );
    println!("                  type:  boolean");
    println!("               default:  `false`, unless `composite` is set");
    println!();

    if all {
        // Print all compiler options when --all is used
        println!("... [additional compiler options would be shown here] ...");
    } else {
        // Continue with standard options
        println!("       --declarationMap  Create sourcemaps for d.ts files.");
        println!("                  type:  boolean");
        println!("               default:  false");
        println!();
        // ... rest of the options
    }

    println!("You can learn about all of the compiler options at https://aka.ms/tsrs");
}

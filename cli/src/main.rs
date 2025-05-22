use clap::Subcommand;
use clap::{Parser, ValueHint};
use std::fs;
use std::path::PathBuf;
use tweed::core::logger::STDLogger;
use tweed::{initialize, instantiate, invoke, LevelFilter, Value};
#[derive(Parser, Debug)]
#[command(name = "tweed")]
#[command(about = "A tiny CLI for tweed", long_about = None)]
#[command(next_display_order = None)]
struct Cli {
    /// Path to the .wasm file    
    #[arg(value_name = "PATH", value_hint = ValueHint::FilePath)]
    path: PathBuf,
    /// Enable verbose trace-level logging
    #[arg(long, help = "Enable trace logging for debugging")]
    trace: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Invoke a function in the WASM module
    Invoke {
        /// Function to invoke
        #[arg(value_name = "FUNCTION")]
        function: String,

        /// Parameters to pass to the function (currently only i32 supported)
        #[arg(value_name = "PARAMS", trailing_var_arg = true)]
        params: Vec<i32>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let wasm_bytes = fs::read(&cli.path)
        .map_err(|e| anyhow::anyhow!("Failed to read '{}': {}", cli.path.display(), e))?;

    if cli.trace {
        STDLogger::init(LevelFilter::Trace);
    } else {
        STDLogger::init(LevelFilter::Info);
    }

    let module = initialize(&wasm_bytes)
        .map_err(|e| anyhow::anyhow!("Failed to initialize the module: {:?}", e))?;

    let mut instance = instantiate(module)
        .map_err(|e| anyhow::anyhow!("Failed to instantiate the module: {:?}", e))?;

    match cli.command {
        Commands::Invoke { function, params } => {
            let values: Vec<Value> = params.into_iter().map(Value::I32).collect();

            let result = invoke(&mut instance, function, values)
                .map_err(|e| anyhow::anyhow!("Function invocation failed: {:?}", e))?;

            println!("Result: {:?}", result);
        }
    }

    Ok(())
}

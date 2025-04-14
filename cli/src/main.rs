use clap::Subcommand;
use clap::{Parser, ValueHint};
use inplace_wasm_interpreter::{execution::instance::instantiate, initialize, invoke, Value};
use std::fs;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(name = "tweed")]
#[command(about = "A tiny CLI for tweed", long_about = None)]
#[command(next_display_order = None)]
struct Cli {
    /// Path to the .wasm file    
    #[arg(value_name = "PATH", value_hint = ValueHint::FilePath)]
    path: PathBuf,
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

    let module =
        initialize(&wasm_bytes).map_err(|_| anyhow::anyhow!("Failed to initialize the module"))?;

    let instance =
        instantiate(module).map_err(|_| anyhow::anyhow!("Failed to instantiate the module"))?;

    match cli.command {
        Commands::Invoke { function, params } => {
            let values: Vec<Value> = params.into_iter().map(Value::I32).collect();

            let result = invoke(instance, function, values)
                .map_err(|_| anyhow::anyhow!("Function invocation failed"))?;

            println!("Result: {:?}", result);
        }
    }

    Ok(())
}

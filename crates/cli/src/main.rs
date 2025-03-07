mod config;

use std::process::ExitCode;

fn main() -> ExitCode {
    // Run the main process and receive its result
    let result = run();

    // Attempt to gracefully exit with result and output `ExitCode`
    match result {
        Ok(code) => return code,
        Err(e) => {
            // For now print error to console and exit gracefully with general error exit code.
            // NOTE: This should be altered later to actually be useful
            eprintln!("Error: {e}");
            return ExitCode::from(2);
        }
    }
}

fn run() -> Result<ExitCode, Box<dyn std::error::Error>> {
    Ok(ExitCode::SUCCESS)
}

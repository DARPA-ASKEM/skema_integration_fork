// First, try ingesting a file into the database.

use clap::Parser;
use rsmgclient::{ConnectParams, Connection, MgError, Value};

#[derive(Parser, Debug)]
struct Cli {
    /// Path to file to ingest
    filepath: String,

    /// Database host
    #[arg(long, default_value_t = String::from("localhost"))]
    db_host: String,
}

fn ingest_file(filepath: &str, db_host: String) -> Result<(), MgError> {
    let connect_params = ConnectParams {
        host: Some(String::from(db_host)),
        ..Default::default()
    };
    let mut connection = Connection::connect(&connect_params)?;

    let contents = std::fs::read_to_string(filepath)
        .unwrap_or_else(|_| panic!("{}", "Unable to read file {filepath}!"));

    let query = format!("CREATE (file:File {{contents: '{}'}})", contents);
    connection.execute_without_results(&query);

    Ok(())
}

fn main() {
    // Read file contents into string
    let args = Cli::parse();
    ingest_file(&args.filepath, args.db_host);
}

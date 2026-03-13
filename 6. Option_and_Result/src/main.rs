mod cli;
use std::{
    collections::HashMap,
    io::{self, Write},
};

use clap::Parser;
use cli::{Cli, Commands};

use crate::connection::Connection;
mod connection;
pub mod connection_state;

fn main() {
    let mut connections_map: HashMap<String, Connection> = HashMap::new();
    let cli = Cli::parse();

    match cli.command {
        None => run_repl(&mut connections_map),
        Some(command) => handle_command(command, &mut connections_map),
    }
}

fn run_repl(connections_map: &mut HashMap<String, Connection>) {
    println!("connSim REPL started. Type commands like: create conn1 10.0.0.1");
    println!("Type 'exit' or 'quit' to stop.");

    loop {
        print!("connSim> ");
        if io::stdout().flush().is_err() {
            println!("Failed to flush stdout.");
            continue;
        }

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("Failed to read input.");
            continue;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.eq_ignore_ascii_case("exit") || line.eq_ignore_ascii_case("quit") {
            println!("Exiting connSim.");
            break;
        }

        let argv = std::iter::once("connSim")
            .chain(line.split_whitespace())
            .collect::<Vec<_>>();

        match Cli::try_parse_from(argv) {
            Ok(parsed) => {
                if let Some(command) = parsed.command {
                    handle_command(command, connections_map);
                } else {
                    println!("Please enter a subcommand.");
                }
            }
            Err(err) => println!("{err}"),
        }
    }
}

fn handle_command(command: Commands, connections_map: &mut HashMap<String, Connection>) {
    match command {
        Commands::Create { id, address } => {
            if connections_map.contains_key(&id) {
                println!("Connection with this id already exists.");
                return;
            }
            connections_map.insert(id.to_owned(), Connection::new(id, &address));
        }
        Commands::Activate { id, address } => {
            let activate_connection = |c: &mut Connection| c.activate_connection();
            with_connection(connections_map, &id, &address, activate_connection);
        }
        Commands::Connect { id, address } => {
            let start_connecting = |c: &mut Connection| c.start_connecting();
            with_connection(connections_map, &id, &address, start_connecting);
        }
        Commands::Succeed { id, address } => {
            let mark_connected = |c: &mut Connection| c.mark_connected();
            with_connection(connections_map, &id, &address, mark_connected);
        }
        Commands::Fail { id, address } => {
            let fail = |c: &mut Connection| c.fail();
            with_connection(connections_map, &id, &address, fail);
        }
        Commands::Disconnect { id, address } => {
            let disconnect = |c: &mut Connection| c.disconnect();
            with_connection(connections_map, &id, &address, disconnect);
        }
        Commands::Inspect { id, address } => {
            let print_summary = |c: &mut Connection| c.print_summary();
            with_connection(connections_map, &id, &address, print_summary);
        }
        Commands::List => {
            println!("List of connections:\n");
            for (k, v) in connections_map.iter() {
                println!("key: {}, value: {:?}", k, v);
            }
        }
    }
}

fn with_connection<F>(
    connections_map: &mut HashMap<String, Connection>,
    id: &str,
    address: &str,
    mut op: F,
) where
    F: FnMut(&mut Connection) -> Result<String, String>,
{
    if let Some(connection) = connections_map.get_mut(id) {
        if connection.get_address() == address {
            let result = op(connection);
            match result {
                Ok(success_message) => println!("{}", success_message),
                Err(error_message) => println!("{}", error_message),
            }
        } else {
            println!(
                "The address cannot be reached for this connection. Please specify the correct address."
            );
        }
    } else {
        println!("No connection with this id exists");
    }
}

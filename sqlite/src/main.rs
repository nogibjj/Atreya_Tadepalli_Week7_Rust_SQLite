mod args;

/// A simple tool that prints back the argument it receives
///#[derive(Parser)]
///struct Args {
    /// The comment to print
    ///comment: String,
///}

use args::Sqliteargs;
use clap::Parser;
use args::UserSubcommand;
use args::EntityType;
use sqlite::{create_table, load_csv, update_table, read_query, drop_table};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Sqliteargs::parse();
    match args.entity_type {
        EntityType::User(user_command) => {
            match user_command.command {
                UserSubcommand::CreateTable(create_user) => {
                    create_table(&create_user.table_name)?;
                }
                UserSubcommand::LoadCsv(load_data) => {
                    load_csv(&load_data.table_name, &load_data.file_path)?;
                }
                UserSubcommand::ReadQuery(read_data) => {
                    read_query(&read_data.table_name)?;
                }
                UserSubcommand::UpdateTable(update_data) => {
                    update_table(&update_data.table_name, &update_data.set_clause, &update_data.condition)?;
                }
                UserSubcommand::DropTable(delete_data) => {
                    drop_table(&delete_data.table_name)?;
                }
            }
        }
    }
    Ok(())  // Ensure to return Ok at the end of the main function
}


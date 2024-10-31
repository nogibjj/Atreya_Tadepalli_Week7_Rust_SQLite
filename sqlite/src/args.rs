use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Debug,Parser)]
#[clap(author,version,about)]
pub struct Sqliteargs{
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug,Subcommand)]
pub enum EntityType{
    // /Create, Load, Update or Delete Table
    User(UserCommand)
}

#[derive(Debug,Args)]
pub struct UserCommand{
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug,Subcommand)]
pub enum UserSubcommand{
    // /Create a new table s
    CreateTable(CreateUser),

    // /Load new data
    LoadCsv(LoadData),

    // /read data
    ReadQuery(ReadData),

    // /Update data
    UpdateTable(UpdateData),

    // /Delete table
    DropTable(DeleteData),
}

#[derive(Debug,Args)]
pub struct CreateUser{
    ///Table Name
    pub table_name: String,
}

#[derive(Debug,Args)]
pub struct LoadData{
    // /Table Name
    pub table_name: String,
    // /File Path
    pub file_path: String,
}

#[derive(Debug,Args)]
pub struct ReadData{
    // /Table Name
    pub table_name: String,
}

#[derive(Debug,Args)]
pub struct UpdateData{
    // /Table Name
    pub table_name: String,
    // /Set Clause - Will involve Wins
    pub set_clause: String,
    // /Condition - may use and operator here
    pub condition: String,
}

#[derive(Debug,Args)]
pub struct DeleteData{
    // /Table Name
    pub table_name: String,
}
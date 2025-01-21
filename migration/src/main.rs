use std::env;

use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    let config = configurator::Settings::new().expect("Error: Unable to populate config structs");

    let db_url = config.database.url;

    // Required for sea-orm to recognize our db and schema, their run_cli_with_connection seems to be a bit bugged atm
    env::set_var("DATABASE_URL", &db_url);

    cli::run_cli(migration::Migrator).await
}

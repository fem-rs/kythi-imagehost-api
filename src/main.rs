use api;

#[actix_web::main]
async fn main() {
    let config = configurator::Settings::new().expect("Error: Unable to populate config structs");

    api::run(config).await.expect("Error: Failed to start api")
}

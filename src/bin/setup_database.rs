use rocket::figment::providers::Format;
use std::path::Path;
use sqlx;
use rocket::tokio;
use rocket::figment;

fn create_database_folder(database_path: &str) -> Result<(), String> {
    let database_path = &database_path.replace("sqlite://", "");
    let path = Path::new(database_path);
    let parent_dir = path
        .parent()
        .ok_or_else(|| format!("Unable to get the database parent directory."))?;
    if path.exists() {
        println!("File already exists, skiping creation.");
        return Ok(())
    }
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir)
            .map_err(|err| format!("Could not create the database parent directory. Err: {err}"))?;
    }

    std::fs::File::create(path)
        .map_err(|err| format!("Could not create the database file. Err: {err}"))?;
    Ok(())
}

async fn run_migrations(database_path: &str) -> Result<(), String>{
    let pool = sqlx::SqlitePool::connect(database_path).await
        .map_err(|err| format!("Could not create a new sqlite pool connection. Err: {err}"))?;
    sqlx::migrate!("database/migrations")
        .run(&pool)
        .await
        .map_err(|err| format!("Could not run the migations. Err: {err}"))?;
    Ok(())
}

fn get_database_path() -> Result<String, String>{
    let figment = figment::Figment::from(
        figment::providers::Toml::file("Rocket.toml")
    );
    figment.extract_inner::<String>("default.databases.urls.url")
        .map_err(|err| format!("Could not get the database path value from Rocket.toml. Err: {err}"))
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let database_path = get_database_path()?;
    create_database_folder(&database_path)?;
    run_migrations(&database_path).await?;
    Ok(())
}

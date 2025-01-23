use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect_db(env_var: &str) -> Result<DatabaseConnection, DbErr> {
  let db_url = std::env::var(env_var).unwrap();
  let db = Database::connect(&db_url).await?;
  Ok(db)
}

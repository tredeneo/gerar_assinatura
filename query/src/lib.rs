use std::path::Path;

use sqlx::{prelude::FromRow, sqlite::SqlitePool, Pool, Sqlite};

pub const SELECT_USER: &str = r#"
            SELECT users.name,login,email,departments.name as 'department',extension,phone_number,users.id
            FROM users
			join departments
			on departments.id = users.department        
			where users.id = ?1
"#;
pub const SELECT_USERS: &str = r#"
            SELECT users.name,login,email,departments.name as 'department',extension,phone_number,users.id
            FROM users
			join departments
			on departments.id = users.department
            where users.name like ?1
"#;
#[derive(Debug, FromRow, Clone)]
pub struct Funcionario {
    pub name: String,
    pub login: String,
    pub department: String,
    pub email: String,
    pub extension: String,
    pub phone_number: String,
    pub id: i32,
}

async fn get_sql_pool() -> anyhow::Result<Pool<Sqlite>> {
    let path = Path::new("./").join("database.sqlite3");
    Ok(SqlitePool::connect(&path.to_str().unwrap_or_default()).await?)
}
pub async fn buscar_no_banco_pelo_id(id: &i32) -> anyhow::Result<Funcionario> {
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, Funcionario>(SELECT_USER)
        .bind(id)
        .fetch_all(&pool)
        .await?;
    let tmp = recs.first().unwrap().clone();
    Ok(tmp)
}
pub async fn buscar_no_banco_pelo_nome(name: &str) -> anyhow::Result<Vec<Funcionario>> {
    let name = format!("{}%", name);
    let pool = get_sql_pool().await?;
    let recs = sqlx::query_as::<_, Funcionario>(SELECT_USERS)
        .bind(name)
        .fetch_all(&pool)
        .await?;
    Ok(recs)
}

use sqlx::{query, PgExecutor, Result};

pub mod provider;
pub mod user;
pub mod vps;

async fn delete<'a>(c: impl PgExecutor<'a>, table: &'a str, id: &'a str) -> Result<u64> {
    let sql = format!("DELETE FROM {table} WHERE id=$1");
    let r = query(&sql).bind(id).execute(c).await?;
    Ok(r.rows_affected())
}

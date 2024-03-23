use sqlx::{query, PgExecutor, QueryBuilder, Result};

use crate::{filter, id, model};

pub async fn create<'a>(c: impl PgExecutor<'a>, m: &'a model::User) -> Result<String> {
    let id = id::new();

    query("INSERT INTO users (id, username, password, dateline) VALUES ($1, $2, $3, $4)")
        .bind(&id)
        .bind(&m.username)
        .bind(&m.password)
        .bind(&m.dateline)
        .execute(c)
        .await?;

    Ok(id)
}

pub async fn update<'a>(c: impl PgExecutor<'a>, m: &'a model::User) -> Result<u64> {
    let r = query("UPDATE users SET username=$1, password=$2 WHERE id=$3")
        .bind(&m.username)
        .bind(&m.password)
        .bind(&m.id)
        .execute(c)
        .await?;

    Ok(r.rows_affected())
}

pub async fn delete<'a>(c: impl PgExecutor<'a>, id: &'a str) -> Result<u64> {
    super::delete(c, "users", id).await
}

pub async fn find<'a>(
    c: impl PgExecutor<'a>,
    by: &'a filter::UserFindBy<'a>,
) -> Result<Option<model::User>> {
    let mut q = QueryBuilder::new("SELECT id, username, password, dateline FROM users WHERE 1=1");

    match by {
        &filter::UserFindBy::ID(id) => q.push(" AND id=").push_bind(id),
        &filter::UserFindBy::Username(username) => q.push(" AND username=").push_bind(username),
    };

    let m = q.build_query_as().fetch_optional(c).await?;

    Ok(m)
}

#[cfg(test)]
mod test {
    use crate::{id, model, password};

    #[tokio::test]
    async fn test_create_user() {
        let dsn =
            std::env::var("PG_DSN").unwrap_or("postgres://vps:vps@127.0.0.1:5432/vps".to_string());
        let pool = sqlx::PgPool::connect(&dsn).await.unwrap();
        let id = id::new();
        let password = password::hash("axum.rs").unwrap();
        let m = model::User {
            id,
            username: "axum.rs".to_string(),
            password,
            dateline: chrono::Local::now(),
        };
        let id = super::create(&pool, &m).await.unwrap();
        println!("{id}");
    }
}

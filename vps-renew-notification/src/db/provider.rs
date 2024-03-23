use sqlx::{query, query_as, PgExecutor, QueryBuilder, Result};

use crate::{filter, id, model};

pub async fn create<'a>(c: impl PgExecutor<'a>, m: &'a model::Provider) -> Result<String> {
    let id = id::new();

    query("INSERT INTO providers (id, name, renew_days, notify_days, dateline) VALUES ($1, $2, $3, $4, $5)")
    .bind(&id)
    .bind(&m.name)
    .bind(&m.renew_days)
    .bind(&m.notify_days)
    .bind(&m.dateline)
    .execute(c)
    .await?;

    Ok(id)
}

pub async fn update<'a>(c: impl PgExecutor<'a>, m: &'a model::Provider) -> Result<u64> {
    let r = query("UPDATE providers SET name=$1, renew_days = $2, notify_days = $3 WHERE id=$4")
        .bind(&m.id)
        .bind(&m.name)
        .bind(&m.renew_days)
        .bind(&m.notify_days)
        .execute(c)
        .await?;
    Ok(r.rows_affected())
}

pub async fn delete<'a>(c: impl PgExecutor<'a>, id: &'a str) -> Result<u64> {
    super::delete(c, "providers", id).await
}

pub async fn find<'a>(c: impl PgExecutor<'a>, id: &'a str) -> Result<Option<model::Provider>> {
    let m =
        query_as("SELECT id, name, renew_days, notify_days, dateline FROM providers WHERE id=$1")
            .bind(id)
            .fetch_optional(c)
            .await?;
    Ok(m)
}

pub async fn list<'a>(
    c: impl PgExecutor<'a>,
    f: &'a filter::ProviderListFilter,
) -> Result<Vec<model::Provider>> {
    let mut q = QueryBuilder::new(
        "SELECT id, name, renew_days, notify_days, dateline FROM providers WHERE 1=1",
    );

    if let Some(name) = &f.name {
        q.push(" AND name ILIKE ").push_bind(format!("%{name}%"));
    }

    if let Some(sort) = &f.sort {
        q.push(format!(" ORDER BY {sort}"));
    } else {
        q.push(" ORDER BY id DESC");
    }

    let ls = q.build_query_as().fetch_all(c).await?;

    Ok(ls)
}

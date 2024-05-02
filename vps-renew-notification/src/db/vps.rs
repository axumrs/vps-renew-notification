use sqlx::{query, query_as, PgExecutor, QueryBuilder, Result};

use crate::{filter, id, model};

pub async fn create<'a>(c: impl PgExecutor<'a>, m: &'a model::VPS) -> Result<String> {
    let id = id::new();

    query("INSERT INTO vpss (id, provider_id, name, expire, dateline) VALUES ($1, $2, $3, $4, $5)")
        .bind(&id)
        .bind(&m.provider_id)
        .bind(&m.name)
        .bind(&m.expire)
        .bind(&m.dateline)
        .execute(c)
        .await?;

    Ok(id)
}

pub async fn update<'a>(c: impl PgExecutor<'a>, m: &'a model::VPS) -> Result<u64> {
    let r = query("UPDATE vpss SET  provider_id=$1, name=$2, expire=$3 WHERE id=$4")
        .bind(&m.provider_id)
        .bind(&m.name)
        .bind(&m.expire)
        .bind(&m.id)
        .execute(c)
        .await?;
    Ok(r.rows_affected())
}

pub async fn delete<'a>(c: impl PgExecutor<'a>, id: &'a str) -> Result<u64> {
    super::delete(c, "vpss", id).await
}

pub async fn find<'a>(c: impl PgExecutor<'a>, id: &'a str) -> Result<Option<model::VPS>> {
    let m = query_as("SELECT id, provider_id, name, expire, dateline FROM vpss WHERE id=$1")
        .bind(id)
        .fetch_optional(c)
        .await?;
    Ok(m)
}

pub async fn list<'a>(
    c: impl PgExecutor<'a>,
    f: &'a filter::VpsListFilter,
) -> Result<Vec<model::VPSWithProvider>> {
    let mut q =
        QueryBuilder::new("SELECT id, provider_id, name, expire, dateline,provider_name,renew_days,notify_days FROM v_vps_proiders WHERE 1=1");
    if let Some(name) = &f.name {
        q.push(" AND name ILIKE ").push_bind(format!("%{name}%"));
    }

    if let Some(provider_id) = &f.provider_id {
        q.push(" AND provider_id=").push_bind(provider_id);
    }

    if let Some(sort) = &f.sort {
        q.push(format!(" ORDER BY {sort}"));
    } else {
        q.push(" ORDER BY id DESC");
    }

    let ls = q.build_query_as().fetch_all(c).await?;

    Ok(ls)
}

pub async fn batch_find<'a>(
    c: impl PgExecutor<'a>,
    ids: &[&str],
) -> Result<Vec<model::VPSWithProvider>> {
    let mut q =
        QueryBuilder::new("SELECT id, provider_id, name, expire, dateline,provider_name,renew_days,notify_days FROM v_vps_proiders WHERE id IN");
    q.push_tuples(ids, |mut b, id| {
        b.push_bind(id);
    });
    q.build_query_as().fetch_all(c).await
}

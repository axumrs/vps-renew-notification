use std::sync::Arc;

use axum::Router;
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use vps_renew_notification::{config, db, filter, model, route, tg, AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cfg = config::Config::from_env().unwrap();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg.db.max_conns)
        .connect(&cfg.db.dsn)
        .await
        .unwrap();

    let addr = cfg.web.addr.clone();

    let state = Arc::new(AppState {
        pool: Arc::new(pool),
        cfg: Arc::new(cfg),
    });

    tokio::spawn(renew_notify(state.clone()));

    let app = Router::new()
        .nest("/auth", route::auth(state.clone()))
        .nest("/", route::manage(state))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let tcp_listener = TcpListener::bind(&addr).await.unwrap();

    tracing::info!("监听于 {}", addr);

    axum::serve(tcp_listener, app.into_make_service())
        .await
        .unwrap();
}

async fn renew_notify(state: Arc<AppState>) {
    let pool = state.pool.clone();
    loop {
        tracing::debug!("新一轮提示检测");
        // 服务商
        let provider_list = db::provider::list(
            &*pool,
            &filter::ProviderListFilter {
                name: None,
                sort: None,
            },
        )
        .await
        .unwrap();
        // vps 列表
        let vps_list = db::vps::list(
            &*pool,
            &filter::VpsListFilter {
                name: None,
                provider_id: None,
                sort: None,
            },
        )
        .await
        .unwrap();

        for vps in vps_list.iter() {
            let provider_list: Vec<&model::Provider> = provider_list
                .iter()
                .filter(|p| p.id == vps.provider_id)
                .collect();
            if provider_list.len() != 1 {
                tracing::info!("没有找到符合条件的服务商");
                continue;
            }
            let provider = provider_list.first().unwrap().to_owned();
            // 提醒时间
            let notify_days = provider.notify_days;
            // 过期天数
            let expire = vps.expire;

            let expect_notify_date =
                chrono::Local::now() + chrono::TimeDelta::try_days(notify_days as i64).unwrap();
            if expect_notify_date >= expire {
                // println!(
                //     "{}: {}, {}, {} 发送通知",
                //     vps.name, notify_days, expire, expect_notify_date
                // )
                // 发送通知
                let text = format!(
                    "{}@{}即将到期({})，请及时续期！\n{}",
                    vps.name,
                    vps.provider_name,
                    expire.format("%Y/%m/%d"),
                    chrono::Local::now().format("%Y/%m/%d %H:%M:%S")
                );
                tracing::debug!("{}", &text);
                tokio::spawn(send_msg(state.clone(), text.clone()));
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(
                state.cfg.bot.sleep_duration as u64,
            ))
            .await;
        }
    }
}

async fn send_msg(state: Arc<AppState>, text: String) {
    tg::send_message(&state.cfg.bot.token, &state.cfg.bot.chat_id, &text)
        .await
        .unwrap();
}

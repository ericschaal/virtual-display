use actix::clock::interval;
use actix::{spawn, Actor, Addr};
use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpServer};
use display_server::actor::{DisplayActor, FlushMsg, RefreshMsg};
use std::time::Duration;

struct AppState {
    addr: Addr<DisplayActor>,
}

#[post("/flush")]
async fn flush(body: web::Payload, data: web::Data<AppState>) -> actix_web::Result<String> {
    let bytes = body.to_bytes().await?;
    data.addr
        .try_send(FlushMsg {
            buffer: bytes.into(),
        })
        .unwrap();
    Ok(format!("OK"))
}

#[post("/refresh")]
async fn refresh(data: web::Data<AppState>) -> actix_web::Result<String> {
    data.addr.try_send(RefreshMsg {}).unwrap();
    Ok(format!("OK"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let display_actor = DisplayActor::default();
    let addr = display_actor.start();

    let state = web::Data::new(AppState { addr });

    spawn({
        let state = state.clone();
        async move {
            let mut interval = interval(Duration::from_secs(1));

            loop {
                state.clone().addr.do_send(RefreshMsg {});
                interval.tick().await;
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(Logger::default())
            .service(flush)
            .service(refresh)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

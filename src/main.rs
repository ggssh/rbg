mod common;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use log::info;

// 日志打印
fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    //设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized .");
}

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}!", &name)
}

// TODO 将actix-web版本升至latest
fn main() {
    init_logger();
    info!("hello world");

    let binding_address = "0.0.0.0:8000";
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(binding_address).expect("Can not bind to port 8000");

    server.run().unwrap();
}

#[test]
fn logger_test() {
    init_logger();
    info!("hello world");
}

#[test]
fn greet_test() {
    let binding_address = "0.0.0.0:8000";
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(binding_address).expect("Can not bind to port 8000");

    server.run().unwrap();
}

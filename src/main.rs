use actix_web::{web, App, HttpServer, Responder};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

async fn index() -> impl Responder {
    info!("Handling request for index");
    "Hello, world!"
}

async fn pipeline() -> impl Responder {
    info!("CodePipeline TEST!!");
    "CodePipeline!!"
}


async fn health_check() -> impl Responder {
    info!("Health Check is OK!!!");
    "OK!!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // トレーサーの初期化
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // サーバーの起動
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .route("/pipeline", web::get().to(pipeline))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
use actix_web::{get, web, App, HttpServer, HttpRequest, Responder, middleware};

mod dice;

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

// #[get("/dice/roll")]
// async fn roll_dice() -> impl Responder {
//     let mut rng = rand::thread_rng();
//     return web::Json(Dice{rolled_value: rng.gen_range(1..6)})
// }

async fn ping(_req: HttpRequest) -> impl Responder {
    format!(
        "I am healthy: {} v{}", env!("CARGO_PKG_DESCRIPTION"), env!("CARGO_PKG_VERSION")
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| 
        App::new()
            .wrap(middleware::Logger::default())
            .service(dice::roll_dice)
            .service(index)
            .route("/ping", web::get().to(ping))
        )
        .bind("127.0.0.1:4000")?
        .run()
        .await
}
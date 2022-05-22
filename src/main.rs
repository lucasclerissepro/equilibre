use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use tracing_actix_web::TracingLogger;
use tracing::{info, Level};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

<<<<<<< Updated upstream
fn main() {
    let cli = CLI {
        name: "Lucas".to_string(),
    };

    println!("Hello, {}", cli);
}
=======
    info!("Starting listening on port 8080");
    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
>>>>>>> Stashed changes

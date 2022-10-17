use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::env;

#[get("/ping")]
async fn ping(req: HttpRequest) -> impl Responder {
    let mut map = HashMap::new();

    for header in req.headers().iter() {
        map.insert(header.0.to_string(), header.1.to_str().unwrap());
        // println!("je suis ici {}", header_loop_string);
    }

    let serialized_data = serde_json::to_string(&map).unwrap();
    // println!("{}", serialized_data);
    // println!("Ça debug fort : {:#?}", header_loop_string);

    HttpResponse::Ok()
        .append_header(("content-type", "application/json"))
        .body(serialized_data)
}

async fn other(_path: web::Path<String>) -> impl Responder {
    HttpResponse::NotFound()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key: &str = "PING_LISTEN_PORT";
    let port = match env::var(key) {
        Ok(val) => val,
        Err(_) => 8080.to_string(),
    };

    HttpServer::new(|| {
        App::new()
            .service(ping)
            .route("/{name}", web::get().to(other))
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}

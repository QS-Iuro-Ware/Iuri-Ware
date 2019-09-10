use actix::prelude::*;
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer};

use iuro_server::{iuro_route, IuroServer};

fn main() {
    env_logger::init();
    let sys = System::new("iuro-server");

    // Start iuro server actor
    let server = IuroServer::default().start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            // redirect to websocket.html
            .service(web::resource("/").route(web::get().to(|| {
                HttpResponse::Found()
                    .header("LOCATION", "/static/websocket.html")
                    .finish()
            })))
            // websocket
            .service(web::resource("/ws/").to(iuro_route))
            // static resources
            .service(fs::Files::new("/static/", "static/"))
    })
    .bind("0.0.0.0:8080")
    .expect("Unable to bind server to port 8080")
    .start();

    sys.run().expect("Failed to run system");
}

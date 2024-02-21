use actix_web::{web, App, HttpResponse, HttpServer};

fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
               <title>GCD Calculator</title>
               <form action="/gcd" method="post">
               <input type="text" name="n"/>
               <input type="text" name="m"/>
               <button type="submit">Compute</button>
               </form>
            "#,
        )
}

fn main() {
    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(get_index))
    });

    println!("Server running on http://localhost:3000 ...");
    match server.bind("127.0.0.1:3000") {
        Ok(server) => {
            if let Err(e) = server.run() {
                eprintln!("Error occurred while running server: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error occurred while binding server to given address: {}", e);
        }
    }
}


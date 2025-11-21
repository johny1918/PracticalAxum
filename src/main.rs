mod models;
mod handlers;
mod routes;
mod app;


#[tokio::main]
async fn main() {

    let app = routes::routes();

    //Setting up listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on {}", listener.local_addr().unwrap());

    //Starting axum server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}


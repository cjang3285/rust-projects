use axum::{
	routing::get,
	Router,
};
use std::net::SocketAddr;

async fn root() -> &'static str {
	"Hello from Rust + Axum!\n"
}

#[tokio::main]
async fn main(){
	let app = Router::new().route("/", get(root));

	let addr = SocketAddr::from(([127,0,0,1], 3000));
	println!("listening on http://{}", addr);

	
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	axum::serve(listener,app).await.unwrap();
}

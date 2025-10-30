use axum::{
	routing::get,
	Router,
};
use std::net::SocketAddr;

use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn root() -> &'static str {
	"Hello from Rust + Axum!\n"
}

#[tokio::main]
async fn main(){

	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer())
		.init();


	let app = Router::new()
			.route("/", get(root))
			.layer(TraceLayer::new_for_http());
			

	let addr = SocketAddr::from(([127,0,0,1], 3000));

	println!("listening on http://{}", addr);

	
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	axum::serve(listener,app).await.unwrap();
}

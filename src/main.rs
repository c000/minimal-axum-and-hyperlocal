use hyperlocal::UnixListenerExt;
use tower::Service;

#[tokio::main]
async fn main() {
    let app = axum::Router::new();

    let listener = tokio::net::UnixListener::bind("/tmp/axum.sock").unwrap();

    listener
        .serve(move || {
            let s = app.clone();
            move |r| s.clone().call(r)
        })
        .await
        .unwrap();
}

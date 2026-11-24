mod router;
mod entities;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("impossible d'ouvrir le port 3000");

    println!("Serveur démarré sur http://127.0.0.1:3000");

    axum::serve(listener, router::app())
        .await
        .expect("erreur du serveur");
}

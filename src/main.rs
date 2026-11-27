mod database;
mod entities;
mod router;

use database::Database;

/// Fichier SQLite de l'application (le dossier est créé s'il n'existe pas)
const DATABASE_PATH: &str = "data/planning.db";

#[tokio::main]
async fn main() {
    let database = Database::connect(DATABASE_PATH)
        .await
        .expect("impossible d'ouvrir la base de données");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("impossible d'ouvrir le port 3000");

    println!("Serveur démarré sur http://127.0.0.1:3000");

    axum::serve(listener, router::app(database))
        .await
        .expect("erreur du serveur");
}

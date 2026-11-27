use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

use crate::database::Database;

/// Construit le router de l'application.
/// Séparé de `main` pour pouvoir le tester sans ouvrir de port
pub fn app(database: Database) -> Router {
    Router::new()
        .route("/salles", get(lister_salles).post(creer_salle))
        .route(
            "/salles/{id}",
            get(obtenir_salle)
                .put(modifier_salle)
                .delete(supprimer_salle),
        )
        .route("/salles/{id}/reservations", get(reservations_de_salle))
        .route(
            "/reservations",
            post(creer_reservation).get(reservations_du_jour),
        )
        .with_state(database)
}

async fn lister_salles() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn creer_salle() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn obtenir_salle() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn modifier_salle() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn supprimer_salle() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn reservations_de_salle() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn creer_reservation() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

async fn reservations_du_jour() -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

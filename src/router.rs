use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};

/// Construit le router de l'application.
/// Séparé de `main` pour pouvoir le tester sans ouvrir de port
/// (via `tower::ServiceExt::oneshot`).
pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        // 1. Gestion des salles
        .route("/salles", get(lister_salles).post(creer_salle))
        .route(
            "/salles/{id}",
            get(obtenir_salle).put(modifier_salle).delete(supprimer_salle),
        )
        // 4. Réservations d'une salle
        .route("/salles/{id}/reservations", get(reservations_de_salle))
        // 2. Réserver une salle / 4. Réservations d'un jour (?date=YYYY-MM-DD)
        .route(
            "/reservations",
            post(creer_reservation).get(reservations_du_jour),
        )
}

async fn health() -> &'static str {
    "ok"
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

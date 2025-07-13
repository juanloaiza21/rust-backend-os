use crate::data::pagination::Pagination;
use crate::data::{get_trips_by_destination, get_trips_by_index, get_trips_by_price_range};
use crate::router_local::AppState;
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{Router, get},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    page: Option<usize>,
    per_page: Option<usize>,
}

impl From<PaginationQuery> for Pagination {
    fn from(query: PaginationQuery) -> Self {
        Self {
            page: query.page.unwrap_or(1),
            per_page: query.per_page.unwrap_or(50),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PriceRangeQuery {
    min: Option<f64>,
    max: Option<f64>,
    page: Option<usize>,
    per_page: Option<usize>,
}

//Viaje por ID
async fn get_trip_by_id(
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    // Código existente sin cambios
    match get_trips_by_index(&id) {
        Ok(Some(trip)) => {
            let json_trip = serde_json::to_value(trip).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error de serialización: {}", e),
                )
            })?;
            Ok(Json(json_trip))
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, "Viaje no encontrado".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))),
    }
}

//Por rango de precio
async fn get_trips_by_price(
    Query(query): Query<PriceRangeQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let min = query.min.unwrap_or(0.0);
    let max = query.max.unwrap_or(f64::MAX);
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(50),
    };

    match get_trips_by_price_range(min, max, pagination) {
        Ok(result) => {
            let json_result = serde_json::to_value(result).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error de serialización: {}", e),
                )
            })?;
            Ok(Json(json_result))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))),
    }
}

//Por destino
async fn get_trips_by_dest(
    Path(destination): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let pagination = Pagination::from(pagination);

    match get_trips_by_destination(&destination, pagination) {
        Ok(result) => {
            let json_result = serde_json::to_value(result).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Error de serialización: {}", e),
                )
            })?;
            Ok(Json(json_result))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e))),
    }
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/{id}", get(get_trip_by_id))
        .route("/price", get(get_trips_by_price))
        .route("/destination/{dest}", get(get_trips_by_dest))
}

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
// use time::Date;

use crate::AppState;

#[derive(Deserialize, Serialize)]
pub struct DashboardData {
    sales_deals_info: SalesDealsInfo,
    sales_per_day_info: Vec<SalesPerDayInfo>,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct SalesDealsInfo {
    open: i64,
    ready: i64,
    awaitingresponse: i64,
    closed: i64,
    total_amt_closed: Option<i64>,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct SalesPerDayInfo {
    date: String,
    sales_total: i64,
}

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct CustomerInfo {
    status: String,
    recordcount: i32,
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub email: String,
}

pub async fn get_dashboard_data(
    State(state): State<AppState>,
    Json(req): Json<UserRequest>,
) -> Result<Json<DashboardData>, impl IntoResponse> {
    let sales_deals_info = match sqlx::query_as::<_, SalesDealsInfo>(
        "SELECT 
        COUNT(*) FILTER (WHERE status = 'open') AS open,
        COUNT(*) FILTER (WHERE status = 'ready') AS ready,
        COUNT(*) FILTER (WHERE status = 'awaitingresponse') AS awaitingresponse,
        COUNT(*) FILTER (WHERE status = 'closed') AS closed,
        SUM(estimate_worth) FILTER (where status = 'closed') AS total_amt_closed
        FROM deals
        WHERE owner_id = (SELECT id FROM users WHERE email = $1)",
    )
    .bind(&req.email.to_string())
    .fetch_one(&state.postgres)
    .await
    {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    let sales_per_day_info = match sqlx::query_as::<_, SalesPerDayInfo>(
        "SELECT
        TO_CHAR(DATE(deals.last_updated), 'yyyy-mm-dd') AS date, 
        SUM(estimate_worth) AS sales_total
        FROM deals
        WHERE status = 'closed'
        GROUP BY deals.last_updated
        ",
    )
    .bind(req.email)
    .fetch_all(&state.postgres)
    .await
    {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    Ok(Json(DashboardData {
        sales_deals_info,
        sales_per_day_info,
    }))
}

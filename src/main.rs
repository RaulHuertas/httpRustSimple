use axum::{
    Json, Router,
    routing::post,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::fmt;

#[derive(Debug, Deserialize)]
struct ShowBigMessageRequest {
    message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum LedColor {
    Red,
    Green,
    Yellow,
    Purple,
    Black,
    White,
}

impl fmt::Display for LedColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = match self {
            LedColor::Red => "red",
            LedColor::Green => "green",
            LedColor::Yellow => "yellow",
            LedColor::Purple => "purple",
            LedColor::White => "white",
            LedColor::Black => "black",
        };

        write!(f, "{color}")
    }
}

fn parse_led_color(color: &str) -> LedColor {
    match color {
        "red" => LedColor::Red,
        "green" => LedColor::Green,
        "yellow" => LedColor::Yellow,
        "purple" => LedColor::Purple,
        "white" => LedColor::White,
        _ => LedColor::Black,
    }
}

#[derive(Debug, Deserialize)]
struct SetLedColorRequest {
    color: String,
}

#[derive(Debug, Serialize)]
struct OkResponse {
    status: &'static str,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/showBigMessage", post(show_big_message))
        .route("/setLedColor", post(set_led_color));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

async fn show_big_message(Json(payload): Json<ShowBigMessageRequest>) -> Json<OkResponse> {
    let message = payload.message;

    println!("BIG message received: {}", message);
    Json(OkResponse { status: "ok" })
}

async fn set_led_color(Json(payload): Json<SetLedColorRequest>) -> Json<OkResponse> {
    let color = parse_led_color(&payload.color);

    println!("Color message received: {}", color);
    Json(OkResponse { status: "ok" })
}

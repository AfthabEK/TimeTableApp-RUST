
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::Files;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Course {
    name: String,
    slot: String,
    plus: bool,
}

#[derive(Serialize, Deserialize)]
struct BatchSem {
    batch: usize,
    sem: usize,
}

#[derive(Serialize, Deserialize)]
struct Slot {
    day: String,
    start_time: String,
    end_time: String,
}

async fn validate_form(course: web::Json<Course>) -> impl Responder {
    // Replace logic to validate form here
    HttpResponse::Ok().json(course.into_inner())
}

async fn switch_batch_sem(batch_sem: web::Json<BatchSem>) -> impl Responder {
    // Logic to switch batch and semester
    HttpResponse::Ok().json(batch_sem.into_inner())
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn get_formatted_date(sem: usize) -> String {
    let now = Utc::now();
    let year = now.year();
    let month = if sem == 1 { now.month() } else { 8 };
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();

    format!(
        "{:04}{:02}{:02}T{:02}{:02}{:02}",
        year, month, day, hour, minute, second
    )
}

fn get_start(t: &Slot, sem: usize) -> String {
    let year = if sem == 1 { 2024 } else { 2024 };
    let month = if sem == 1 { "01" } else { "08" };
    let day = match t.day.as_str() {
        "SU" => "07",
        "MO" => "01",
        "TU" => "02",
        "WE" => "03",
        "TH" => "04",
        "FR" => "05",
        "SA" => "06",
        _ => "01",
    };
    format!("{}{}{}T{}0000", year, month, day, t.start_time)
}

fn get_end(t: &Slot, sem: usize) -> String {
    let year = if sem == 1 { 2024 } else { 2024 };
    let month = if sem == 1 { "01" } else { "08" };
    let day = match t.day.as_str() {
        "SU" => "07",
        "MO" => "01",
        "TU" => "02",
        "WE" => "03",
        "TH" => "04",
        "FR" => "05",
        "SA" => "06",
        _ => "01",
    };
    format!("{}{}{}T{}5000", year, month, day, t.end_time)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "./static").index_file("index.html"))
            .route("/validate", web::post().to(validate_form))
            .route("/switch_batch_sem", web::post().to(switch_batch_sem))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


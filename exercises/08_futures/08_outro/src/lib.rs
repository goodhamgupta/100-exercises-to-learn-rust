use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct Ticket {
    id: u32,
    title: String,
    description: String,
}

struct AppState {
    tickets: Mutex<Vec<Ticket>>,
}

async fn create_ticket(ticket: web::Json<Ticket>, data: web::Data<AppState>) -> impl Responder {
    let mut tickets = data.tickets.lock().unwrap();
    tickets.push(ticket.into_inner());
    HttpResponse::Ok().json("Ticket created successfully")
}

async fn get_ticket(id: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let tickets = data.tickets.lock().unwrap();
    if let Some(ticket) = tickets.iter().find(|t| t.id == *id) {
        HttpResponse::Ok().json(ticket)
    } else {
        HttpResponse::NotFound().json("Ticket not found")
    }
}

async fn patch_ticket(
    id: web::Path<u32>,
    ticket: web::Json<Ticket>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut tickets = data.tickets.lock().unwrap();
    if let Some(t) = tickets.iter_mut().find(|t| t.id == *id) {
        t.title = ticket.title.clone();
        t.description = ticket.description.clone();
        HttpResponse::Ok().json("Ticket updated successfully")
    } else {
        HttpResponse::NotFound().json("Ticket not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        tickets: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/tickets", web::post().to(create_ticket))
            .route("/tickets/{id}", web::get().to(get_ticket))
            .route("/tickets/{id}", web::patch().to(patch_ticket))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

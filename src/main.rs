use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use chrono::Local;
use serde_json::Value;

#[get("/")]
async fn root_route(meals: web::Data<Value>) -> impl Responder {
    println!("root triggered");

    HttpResponse::Ok()
        .body(meals.to_string())
        .customize()
        .insert_header(("content-type", "application/json"))
}

#[get("/day/{day}")]
async fn day_route(meals: web::Data<Value>, path: web::Path<String>) -> impl Responder {
    println!("day triggered");

    let date: String = path.into_inner();

    if meals[&date].as_object().is_none() || meals[&date].as_object().unwrap().is_empty() {
        return HttpResponse::InternalServerError()
            .body("{\"Error\": \"Requested day data is unavailable.\"}")
            .customize()
            .insert_header(("content-type", "application/json"));
    } else {
        return HttpResponse::Ok()
            .body(meals[&date].to_string())
            .customize()
            .insert_header(("content-type", "application/json"));
    }
}

#[get("/today")]
async fn today_route(meals: web::Data<Value>) -> impl Responder {
    println!("today triggered");

    let today = Local::now();
    let today_str = today.format("%d.%m.%Y").to_string();

    if meals[&today_str].as_object().is_none() {
        return HttpResponse::InternalServerError()
            .body("{\"Error\": \"Requested day data is unavailable.\"}")
            .customize()
            .insert_header(("content-type", "application/json"));
    } else {
        return HttpResponse::Ok()
            .body(meals[&today_str].to_string())
            .customize()
            .insert_header(("content-type", "application/json"));
    }
}

#[get("/tomorrow")]
async fn tomorrow_route(meals: web::Data<Value>) -> impl Responder {
    println!("tomorrow triggered");

    let tomorrow = Local::now() + chrono::Duration::days(1);
    let tomorrow_str = tomorrow.format("%d.%m.%Y").to_string();

    if meals[&tomorrow_str].as_object().is_none() {
        return HttpResponse::InternalServerError()
            .body("{\"Error\": \"Requested day data is unavailable.\"}")
            .customize()
            .insert_header(("content-type", "application/json"));
    } else {
        return HttpResponse::Ok()
            .body(meals[&tomorrow_str].to_string())
            .customize()
            .insert_header(("content-type", "application/json"));
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server is starting..");

    let resp: String = reqwest::get("https://yemekhane.cu.edu.tr/yemeklistejson.asp")
        .await
        .unwrap()
        .text_with_charset("ISO-8859-9")
        .await
        .unwrap()
        .replace(
            "<meta http-equiv=\"Content-Type\" content=\"text/html; charset=windows-1254\">",
            "",
        )
        .trim()
        .to_string();

    let meals: Value = serde_json::from_str(&resp).unwrap();

    // pass meals to the routes with web::Data
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(meals.clone()))
            .service(root_route)
            .service(today_route)
            .service(tomorrow_route)
            .service(day_route)
    })
    .bind("0.0.0.0:2002")?
    .run()
    .await
}

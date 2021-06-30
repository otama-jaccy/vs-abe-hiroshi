extern crate image;
extern crate rand;

use std::{fs::File, io::Read, usize};

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, http::header::ContentType, web};
use image::{ImageFormat};

fn gacha_filter() -> [f32; 9]{
    let gausian: [f32; 9] = 
        [   
            1.0/9.0, 1.0/9.0, 1.0/9.0, 
            1.0/9.0, 1.0/9.0, 1.0/9.0,
            1.0/9.0, 1.0/9.0, 1.0/9.0
        ];

    let sharp =
        [
            -1.0, -1.0, -1.0,
            -1.0, 9.0, -1.0,
            -1.0, -1.0, -1.0
        ];

    let lap =
        [
            0.0, 1.0, 0.0,
            1.0, -4.0, 1.0,
            0.0, 1.0, 0.0
        ];
    let filters = [gausian, sharp, lap];
    let idx = rand::random::<usize>()%filters.len();
    filters[idx]
}


#[get("/images/icon")]
async fn get_image() -> impl Responder{
    let mut img = image::open("public/image.jpg").unwrap();
    img = img.filter3x3(&gacha_filter());
    let mut buffer = Vec::new();
    img.write_to(&mut buffer, ImageFormat::Jpeg).unwrap();

    let mut builder = HttpResponse::Ok();
    builder.set(
        ContentType::png()
    );
    builder
        .body(buffer) 
}

#[get("/")]
async fn top() -> impl Responder {
    let mut file = File::open("public/html/index.html").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    HttpResponse::Ok().header("Content-Type", "text/html; charset=utf-8").body(contents)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
            App::new()
                .service(get_image)
                .service(top)
    })    
    .bind("localhost:8080")?
    .run()
    .await
}
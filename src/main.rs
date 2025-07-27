use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

use env_logger::Env;
use std::env;
use serde::{Serialize, Deserialize};

use redis::AsyncCommands;

mod redis_client;
use redis_client::init_redis_pool;
use redis_client::RedisPool;

mod utils;
use utils::common::generate_random_number;

#[derive(Serialize)]
struct OtpResponse {
    phone: String,
    status: String,
    otp: u16,
}

#[derive(Deserialize)]
pub struct OtpQuery {
    phone: String
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("Received request to /");
    HttpResponse::Ok().body("Hello from Actix!")
}

#[get("/get-otp")]
async fn get_otp(query: web::Query<OtpQuery>, redis: web::Data<RedisPool>) -> impl Responder {
    let result = generate_random_number((1000, 9999));
    let phone = query.phone.to_string();
    //Store OTP in redis with 5 mins expiry
    {
        let mut conn = redis.lock().await;
        let _: () = conn.set_ex(phone.clone(), result, 300).await.unwrap();
    }
    let response = OtpResponse {
        phone: phone,
        status: "success".to_string(),
        otp: result
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    println!("🔍 Current working directory: {:?}", env::current_dir()?);
    println!("✅ Starting Actix server on port 8080...");
    let redis_url = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string());
    let redis_pool = init_redis_pool(&redis_url).await;
    
    match HttpServer::new(move || {
        println!("📦 Creating new App instance");
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(redis_pool.clone()))
            .service(hello)
            .service(get_otp)
    })
    .bind(("0.0.0.0", 8080)) {
        Ok(server) => {
            println!("🚀 Server bound successfully to 0.0.0.0:8080");
            server.workers(2).run().await
        },
        Err(e) => {
            eprintln!("❌ Failed to bind server: {}", e);
            Err(e)
        }
    }
}

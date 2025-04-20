use actix_web::{http::header, web, App, Error, HttpResponse, HttpServer, Responder};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::{Deserialize, Serialize};

//  Configuration for Kafka 
const KAFKA_BROKERS: &str = "127.0.0.1:9092";
const REGISTRATION_TOPIC: &str = "user_registrations";

// User struct for registration
#[derive(Serialize, Deserialize, Debug)]
struct UserRegistration {
    user_id: String,
    username: String,
    email: String,
}
//  Handler for user registration
async fn register_user(
    user: web::Json<UserRegistration>,
    producer: web::Data<FutureProducer>,
) -> Result<impl Responder, Error> {
    let user_id = String::from("fadfasfasdfasdfsafa");
    let registration = UserRegistration {
        user_id: user_id.clone(),
        username: user.username.clone(),
        email: user.email.clone(),
    };

    let payload = serde_json::to_string(&registration).map_err(|e| {
        actix_web::error::ErrorInternalServerError(e) 
    })?;

    let record = FutureRecord::to(REGISTRATION_TOPIC)
        .key(&user_id) 
        .payload(&payload);

    producer.send(record, std::time::Duration::from_secs(2))
        .await
        .map_err(|(e, _)| {
            eprintln!("Failed to send Kafka message: {:?}", e);
            actix_web::error::ErrorInternalServerError("Failed to send registration event")
        })?;

    Ok(HttpResponse::Ok().append_header(header::ContentType::json()).body(format!("User registered with ID: {}", user_id)))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure Kafka producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", KAFKA_BROKERS)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");


    let producer_data = web::Data::new(producer);

    HttpServer::new(move || {
        App::new()
            .app_data(producer_data.clone()) //  Share the Kafka producer
            .route("/api/health", web::get().to(health_check))
            .route("/api/register", web::post().to(register_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

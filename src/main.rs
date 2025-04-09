use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;

async fn render_react_component() -> Result<String, std::io::Error> {
    let output = Command::new("node")
        .arg("./app/render.js") 
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr),
        ))
    }
}

async fn index() -> impl Responder {
    let rendered_html = match render_react_component().await {
        Ok(html) => html,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Rendering error: {}", e)),
    };

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>React SSR with Rust</title>
        </head>
        <body>
            <div id="app">{}</div>
            <script src="/dist/bundle.js"></script>
        </body>
        </html>
        "#,
        rendered_html
    );

    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(actix_files::Files::new("/dist", "./dist").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

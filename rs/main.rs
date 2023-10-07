mod controllers;
mod models;

use tera::Tera;
use sqlx::MySqlPool;
use std::sync::Arc; 
use std::error::Error;

#[derive(Clone)]
pub struct AppState {
    template: Arc<tera::Tera>, 
    db: MySqlPool, 
} 

const TEMPLATES_PATH: &str = "templates";

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    use std::env;
    use controllers::staticpage::StaticPage;
    use actix_web::{web,App, HttpServer, web::Data};
    use actix_files as fs;
    
    dotenv::dotenv().ok();
    
    env_logger::init();
    let template = Arc::new(Tera::new(&format!("{}/**/*.html", TEMPLATES_PATH)).unwrap());
    let appstate = AppState {
            template,
            db: MySqlPool::connect(&env::var("DATABASE_URL").unwrap())
                .await
                .expect("Connection to database failed"),
    }; 

    HttpServer::new(move || {

        App::new()
            .app_data(Data::new(appstate.clone()))
            .service(fs::Files::new("/static","./public"))
            .route("/",web::get().to(StaticPage("index.html")))
            .service(controllers::portfolio::view)
            .service(controllers::portfolio::view_legacy)
            .route("/profdev", web::get().to(StaticPage("profdev.html")))
            .service(web::scope("/projects")
                .route("/inttech", web::get().to(StaticPage("inttech.html")))
            )


    })
    .bind(("0.0.0.0",8080))?
    .run()
    .await
}




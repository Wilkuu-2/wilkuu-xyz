use actix_web::{web, get, error, HttpResponse};
use crate::AppState;
use crate::models::{PortfolioEntry,PortfolioEntryVersion};
use tera::Context;
use tokio::fs::read_to_string;
use tokio::spawn; 

#[get("/portfolio")]
async fn view(state: web::Data<AppState>) -> Result<HttpResponse, error::Error>{
    let mut ctx = Context::new();
    
    let listing = PortfolioEntry::get_all_visible(&state.db, PortfolioEntryVersion::V1).await;

    println!("Found {} entries", listing.0.len());

    ctx.insert("portfolio_list",&listing);

    match state.template.render("portfolio.html", &ctx){
        Ok(body)  => Ok(HttpResponse::Ok().body(body)),
        Err(_err) => Err(error::ErrorInternalServerError(format!("Template Error\n{}\n",_err.to_string()))),
    }
}

#[get("/portfolio/legacy")] 
async fn view_legacy(state: web::Data<AppState>) -> Result<HttpResponse, error::Error>{
    let mut ctx = Context::new();
    let page_paths = PortfolioEntry::get_all_legacy(&state.db).await.0;

    let tasks: Vec<_> = page_paths
        .iter()
        .map(|page_name| { format!("templates/portfolio/{}", page_name.name) })
        .map(move |page_path| { spawn(read_to_string(page_path)) })
        .collect();

    let pages: Vec<String> = futures::future::join_all(tasks).await
        .iter()
        .map(move |res| {
            match res {
                Ok(ok1)   => {
                    match ok1 {
                       Ok(ok2) => ok2.clone(), 
                       Err(_) => "".to_string(),
                    }
                },
                Err(_) => "".to_string()
            }
        })
        .filter(|body| { body.len() > 0}) // Throw out all the rejected stuff 
        .collect();

    ctx.insert("portfolio_list", &pages);

    match state.template.render("portfolio_legacy.html", &ctx){
        Ok(body)  => Ok(HttpResponse::Ok().body(body)),
        Err(_err) => Err(error::ErrorInternalServerError("Template Error")),
    }
}


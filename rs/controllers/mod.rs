pub mod staticpage;
pub mod portfolio;



trait Controller {
    fn routes(root_path: &'static str) -> actix_web::Scope;
}


pub type PortfolioController = (); 

impl Controller for PortfolioController {
    fn routes(root_path: &'static str) -> actix_web::Scope {
        actix_web::web::scope(root_path)
    } 
}


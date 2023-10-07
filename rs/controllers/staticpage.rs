

use std::future;
use tera::Context;
use crate::AppState;
use actix_web::{web,HttpResponse,error, Handler};

#[derive(Clone)]
pub struct StaticPage (pub &'static str);

impl Handler<web::Data<AppState>> for StaticPage {
    type Output = Result<HttpResponse, error::Error>;
    type Future = future::Ready<Self::Output>;

    fn call(&self, args: web::Data<AppState>) -> Self::Future {
        future::ready(static_page(self.0 , args))
    }

} 

fn static_page(template_name: &'static str , state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    match state.template.render(template_name, &Context::new()){
        Ok(body)  => Ok(HttpResponse::Ok().body(body)),
        Err(_err) => Err(error::ErrorInternalServerError("Template Error")),
    }
}




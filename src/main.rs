extern crate actix;
extern crate actix_web;
mod models;
use std::env;
use dotenv::dotenv;
use env_logger;
use crate::models::config::ApplicationError;
use crate::models::user::User;
use chrono::prelude::*;
use actix_web::{http::header, web::{Bytes, post, Query,Data,resource}, http::Method,HttpRequest, middleware, App, HttpServer,dev::ServiceRequest,Error,error, HttpResponse};
use futures::StreamExt;
use std::string::ToString;

use std::convert::Into;
use serde_json::{Value};
use actix_web::http::header::ContentEncoding::Deflate;
use serde::{Serialize, Deserialize};


async fn userregistrationhandler(payload: actix_web::web::Json<User>,request : HttpRequest) ->  Result<HttpResponse, Error> {
    println!("----> function postexamplehandler");
    let userdata=payload.into_inner();
    println!("-----> user data :\n{:?}", userdata);

    Ok(HttpResponse::Ok().content_type("application/json").json(userdata))
}


async fn who(bytes : Bytes, request : HttpRequest) ->  Result<HttpResponse, Error> {
    println!("who function");
    Ok(HttpResponse::Ok().content_type("application/json").json("hello"))
}

async fn whohandler(request : HttpRequest) ->  Result<HttpResponse, Error> {
    println!("----> function whohandler");

    let name=request.match_info().get("name");

    Ok(HttpResponse::Ok().content_type("application/json").json(name))
}


//========================================MAIN==========================================================================================
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("---------> ActixWeb - API - Rust -  2022 --   running in 0.0.0.0:5555------Version 1.0000 <---------");

    HttpServer::new(move || {
        App::new()
            .wrap(
                actix_cors::Cors::new()
                    .allowed_methods(vec![Method::GET, Method::OPTIONS, Method::POST])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .finish()
            )
            .data(actix_web::web::JsonConfig::default().limit(1024 * 1024 * 500))
            .data(actix_web::web::PayloadConfig::new(1024 * 1024 * 500))
            .service(actix_web::web::resource("/servicesandintegration/who/{name}").route(actix_web::web::get().to(whohandler)))

            .service(actix_web::web::resource("/servicesandintegration/userregistration").route(actix_web::web::post().to(userregistrationhandler)))
    })
        .bind("0.0.0.0:5555")
        .unwrap()
        .run()
        .await
}
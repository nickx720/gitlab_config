use actix_files as fs;
use actix_web::{
    client::Client, error, middleware::Logger, web, App, Error, HttpResponse, HttpServer,
};
use async_trait::async_trait;
use dotenv::dotenv;
use env_logger::Env;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::fmt;

#[async_trait(?Send)]
trait JSONResponse {
    async fn generate_api_response(api: String) -> Result<Vec<Box<Self>>, Error>;
}

#[derive(Deserialize, Serialize, Debug)]
struct ProjectId {
    id: u32,
}
#[async_trait(?Send)]
impl JSONResponse for ProjectId {
    async fn generate_api_response(api: String) -> Result<Vec<Box<Self>>, Error> {
        let body = Client::default()
            .get(&api)
            .header("User-Agent", "actix-web/3.0")
            .send()
            .await?
            .json::<Vec<Self>>()
            .await?;
        Ok(body.into_iter().map(|x| Box::new(x)).collect())
    }
}
impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id.to_string())
    }
}

async fn index() -> Result<HttpResponse, Error> {
    let body = ProjectId::generate_api_response(String::from(
        "https://gitlab.com/api/v4/users/nick_thomas/projects",
    ))
    .await?;
    let mut final_view: Vec<Value> = Vec::new();
    for item in body {
        let body = Client::default()
            .get(format!(
                "https://gitlab.com/api/v4/projects/{}/languages",
                item.id
            ))
            .header("User-Agent", "actix-web/3.0")
            .send()
            .await?
            .json::<Value>()
            .await?;
        final_view.push(body);
    }
    let filtered_view = final_view
        .into_iter()
        .filter(|x| *x != json!({}))
        .collect::<Vec<Value>>();
    Ok(HttpResponse::Ok().json(filtered_view))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let addr = match env::var("PORT") {
        Ok(val) => val,
        Err(_) => "127.0.0.1".to_string(),
    };
    let port = match env::var("ADDRESS") {
        Ok(val) => val,
        Err(_) => "8080".to_string(),
    };
    let binded_port = format!("{}:{}", port, addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/summary", web::get().to(index))
            .service(fs::Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind(binded_port)?
    .run()
    .await
}

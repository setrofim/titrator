use actix_web::{http, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
struct Problem {
    title: String,
    status: u16,
    #[serde(skip_serializing_if="String::is_empty")]
    detail: String,
    #[serde(skip)]
    code: http::StatusCode,
}

fn problem(code: http::StatusCode, detail: String) -> Problem {
    Problem {
        title: match code.canonical_reason() {
            Some(reason) => reason.to_owned(),
            None => format!("Status {}", code.as_u16()),
        },
        detail: detail,
        status: code.as_u16(),
        code: code,
    }
}

#[derive(Deserialize)]
#[derive(Debug)]
struct NonceSpec {
    nonce: Option<String>,
    #[serde(rename = "nonceSize")]
    nonce_size: Option<u16>,
}

#[post("/newSession")]
async fn new_session(req: HttpRequest) -> impl Responder {
    let spec = web::Query::<NonceSpec>::from_query(req.query_string()).unwrap();

    let ctype = req.headers().get(http::header::CONTENT_TYPE).unwrap().to_str().unwrap();

    if ctype != "application/vnd.veraison.challenge-response-session+json" {
    }

    HttpResponse::Ok().body(format!("new session {:?}", spec))
}

#[post("/session/{session_id}")]
async fn session(session_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("session {session_id}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        let challege_response = web::scope("/challenge-response/v1")
            .service(new_session);

        App::new()
            .wrap(Logger::default())
            .service(challege_response)
    })
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}

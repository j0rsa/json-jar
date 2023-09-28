use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, post};
use serde_json::Value;
use std::io::{Read, Write};
use std::sync::Mutex;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/raw")]
async fn raw(payload: web::Json<Value>) -> impl Responder {
    match serde_json::to_string_pretty(&payload) {
        Ok(v) => {
            log::info!("Payload: {}", v);
            HttpResponse::Ok().body("OK")
        }
        _ => HttpResponse::BadRequest().body(""),
    }
}

#[post("/csv")]
async fn csv_write(payload: web::Json<Value>, config: web::Data<Mutex<CsvConfig>>) -> impl Responder {
    let config = config.lock().unwrap();
    let line = get_csv_line(&Some(payload.0), &config);
    match line {
        Err(chaim) => HttpResponse::BadRequest().body(chaim),
        Ok(l) => {
            log::info!("Line: {}", l);
            match config.file {
                Some(ref file) => {
                    let mut file = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(file)
                        .unwrap();
                    file.write_all(l.as_bytes()).unwrap();
                    file.write_all("\n".as_bytes()).unwrap();
                }
                None => {}
            }
            HttpResponse::Ok().body("OK")
        }
    }
}

#[get("/csv")]
async fn csv_get(config: web::Data<Mutex<CsvConfig>>) -> impl Responder {
    let config = config.lock().unwrap();
    match config.file {
        Some(ref file) => {
            match std::fs::OpenOptions::new().read(true).open(file) {
                Ok(mut file) => {
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    HttpResponse::Ok().body(contents)
                }
                Err(_) => {
                    HttpResponse::Ok().body("")
                }
            }
        }
        None => HttpResponse::Ok().body(""),
    }
}

fn get_csv_line(json: &Option<Value>, config: &CsvConfig) -> Result<String, String> {
    let mut line = Vec::new();
    for column_chain in &config.columns {
        let key = get_key(json, column_chain.to_vec());
        match key {
            None => return Err(format!("Key not found: {}", column_chain.join("."))),
            Some(k) => line.push(k),
        }
    }
    Ok(line.join(&config.delimiter))
}

fn get_key(json: &Option<Value>, key_chain: Vec<String>) -> Option<String> {
    match json {
        None => None,
        Some(j) => {
            if key_chain.len() == 0 {
                return if !j.is_array() && !j.is_object() {
                    if j.is_string() {
                        Some(j.as_str().unwrap().to_string())
                    } else {
                        Some(j.to_string())
                    }
                } else {
                    None
                };
            }
            let key = key_chain[0].clone();
            let sub_json = j.get(key).map(|s| s.to_owned());
            get_key(&sub_json, key_chain[1..].to_vec())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = web::Data::new(Mutex::new(CsvConfig::from_env()));
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let port = port.parse::<u16>().unwrap();
    HttpServer::new(move || {
        App::new()
            .service(raw)
            .app_data(config.clone())
            .service(csv_write)
            .service(csv_get)
            .service(health)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[derive(Clone)]
struct CsvConfig {
    pub columns: Vec<Vec<String>>,
    pub delimiter: String,
    #[allow(dead_code)]
    pub count: usize,
    pub file: Option<String>,
}

impl CsvConfig {
    pub fn from_env() -> Self {
        let count = std::env::var("COLUMNS").expect("COLUMNS is not set");
        let count = count.parse::<usize>().expect("COLUMNS is not a number");
        let delimiter = std::env::var("DELIMITER").unwrap_or(",".to_string());
        let columns = (0..count)
            .map(|i| {
                let key = format!("COLUMN_{}", i);
                let column_chain =
                    std::env::var(key.clone()).expect(&format!("{} is not set", key));
                column_chain
                    .split(".")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        let file = std::env::var("CSV_FILE").ok();

        Self {
            columns,
            delimiter,
            count,
            file,
        }
    }

    #[allow(dead_code)]
    pub fn new(
        columns: Vec<Vec<String>>,
        delimiter: String,
        count: usize,
        file: Option<String>,
    ) -> Self {
        Self {
            columns,
            delimiter,
            count,
            file,
        }
    }
}

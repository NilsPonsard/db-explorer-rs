#[macro_use]
extern crate rocket;

extern crate serde;
extern crate serde_json;

use rocket::State;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(FromFormField)]
enum Lang {
    #[field(value = "en")]
    English,
    #[field(value = "ru")]
    #[field(value = "ру")]
    Russian,
}

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}

// Note: without the `..` in `opt..`, we'd need to pass `opt.emoji`, `opt.name`.
//
// Try visiting:
//   http://127.0.0.1:8000/?emoji
//   http://127.0.0.1:8000/?name=Rocketeer
//   http://127.0.0.1:8000/?lang=ру
//   http://127.0.0.1:8000/?lang=ру&emoji
//   http://127.0.0.1:8000/?emoji&lang=en
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en
//   http://127.0.0.1:8000/?emoji&name=Rocketeer
//   http://127.0.0.1:8000/?name=Rocketeer&lang=en&emoji
//   http://127.0.0.1:8000/?lang=ru&emoji&name=Rocketeer
#[get("/?<lang>&<opt..>")]
fn hello(lang: Option<Lang>, opt: Options<'_>) -> String {
    let mut greeting = String::new();
    if opt.emoji {
        greeting.push_str("👋 ");
    }

    match lang {
        Some(Lang::Russian) => greeting.push_str("Привет"),
        Some(Lang::English) => greeting.push_str("Hello"),
        None => greeting.push_str("Hi"),
    }

    if let Some(name) = opt.name {
        greeting.push_str(", ");
        greeting.push_str(name);
    }

    greeting.push('!');
    greeting
}

#[get("/list")]
fn list(settings: &State<Settings>) -> String {
    let mut out: String = String::new();

    if settings.error {
        out.push_str("config error")
    }else{
        out.push_str(&settings.ah);
    }


    out
}

#[derive(Serialize, Deserialize)]
struct Settings {
    ah: String,
    error: bool,
}

#[launch]
fn rocket() -> _ {
    let file_path = std::path::Path::new("settings.json");
    let mut settings: Settings = Settings {
        ah: "".to_string(),
        error: true,
    };
    if !file_path.exists() {
        let res = fs::write(file_path, "{}");
        println!("{}", res.is_err())
    } else {
        let res = fs::read_to_string(file_path);

        if res.is_err() {
            println!("Error reading the file : {}", res.unwrap_err());
        } else {
            let content = res.unwrap();

            settings = serde_json::from_str(&content).unwrap_or(Settings {
                ah: "".to_string(),
                error: true,
            });

            println!("{} {}", settings.error, settings.ah)
        }
    }

    rocket::build()
        .manage(settings)
        .mount("/", routes![hello, list])
}

#[macro_use]
extern crate rocket;

use anyhow::Result;
use pge_status::request;
use rocket_dyn_templates::{context, Template};

#[get("/?<addr>")]
async fn index(addr: Option<String>) -> Result<Template, String> {
    match addr {
        Some(addr) => status_page(addr).await,
        None => landing_page(),
    }
}

fn landing_page() -> Result<Template, String> {
    Ok(Template::render("index", context! {}))
}

async fn status_page(addr: String) -> Result<Template, String> {
    match request(&addr).await {
        Ok(info) => Ok(Template::render(
            "status",
            context! {
                query: addr,
                info: info,
            },
        )),
        Err(e) => Err(format!("{}", e)),
    }
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
}

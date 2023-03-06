#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::request;
use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

pub mod api;
mod dev;

use api::count::{self, UserCount};

#[catch(401)]
fn not_authorized(_req: &request::Request) -> RawHtml<&'static str> {
	RawHtml(r#"<h1>haha lol no</h1>"#)
}

#[catch(404)]
fn not_found(req: &request::Request) -> RawHtml<&'static str> {
	let do_connecting = req.headers().get_one("do-connecting-ip");
	if let Some(ip) = do_connecting {
		println!("404 ip: {}", ip);
	}
	RawHtml(r#"<h1>404 page not found</h1>"#)
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.manage(UserCount::default())
		// .manage(UserIPs(vec![]))
		.register("/", catchers![not_authorized, not_found])
		.attach(Template::fairing())
		.mount("/", FileServer::from("public/"))
		.mount("/dev", dev::routes())
		.mount("/api", count::routes())
}

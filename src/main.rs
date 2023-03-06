#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::request;
use rocket_dyn_templates::Template;
use rocket::response::content::RawHtml;

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

#[post("/<_..>", rank = 2)]
fn skillissue() -> RawHtml<&'static str> {
	println!("SKILL ISSUE DETECTED");
	RawHtml(r#"<img src="images/skill-issue.gif">
skill issue."#)
}


#[launch]
fn rocket() -> _ {
	rocket::build()
		.manage(UserCount::default())
		// .manage(UserIPs(vec![]))
		.register("/", catchers![not_authorized, not_found])
		.attach(Template::fairing())
		.mount("/dev", dev::routes())
		.mount("/", FileServer::from("public/"))
		.mount("/", routes![skillissue])
		.mount(
			"/api",
			routes![count::count, count::inc_count, count::remove_ip, count::ips],
		)
}

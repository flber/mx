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


#[launch]
fn rocket() -> _ {
	rocket::build()
		.manage(UserCount::default())
		// .manage(UserIPs(vec![]))
		.register("/", catchers![not_authorized])
		.attach(Template::fairing())
		.mount("/dev", dev::routes())
		.mount("/", FileServer::from("public/"))
		.mount(
			"/api",
			routes![count::count, count::inc_count, count::remove_ip, count::ips],
		)
}

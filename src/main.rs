#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

pub mod api;
mod dev;

use api::count::{self, UserCount};

#[launch]
fn rocket() -> _ {
	rocket::build()
		.manage(UserCount::default())
		// .manage(UserIPs(vec![]))
		.attach(Template::fairing())
		.mount("/dev", dev::routes())
		.mount("/", FileServer::from("public/"))
		.mount(
			"/api",
			routes![count::count, count::inc_count, count::remove_ip, count::ips],
		)
}

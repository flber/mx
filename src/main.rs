#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::content::RawJson;
use rocket::State;
use rocket_dyn_templates::Template;
use std::net::SocketAddr;

// use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

mod dev;

struct UserCount(AtomicUsize);
// struct UserIPs(Vec<Mutex<IpAddr>>);

#[get("/count")]
fn count(user_count: &State<UserCount>) -> RawJson<String> {
	RawJson(format!(
		"{{\"user_count\": {}}}",
		user_count.0.load(Ordering::Relaxed)
	))
}

#[post("/count")]
fn inc_count(user_count: &State<UserCount>, sock: SocketAddr) {
	user_count.0.fetch_add(1, Ordering::SeqCst);
	println!("Remote Address: {:?}", sock.ip());
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.manage(UserCount(AtomicUsize::new(0)))
		// .manage(UserIPs(vec![]))
		.attach(Template::fairing())
		.mount("/dev", dev::routes())
		.mount("/", FileServer::from("public/"))
		.mount("/api", routes![count, inc_count])
}

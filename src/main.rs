#[macro_use]
extern crate rocket;
use rocket::fs::FileServer;
use rocket::response::content::RawJson;
use rocket::State;
use rocket_dyn_templates::Template;
use std::net::{self, SocketAddr};

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

mod dev;

struct UserCount {
	count: AtomicUsize,
	ips: Mutex<Vec<net::IpAddr>>,
}
// struct UserIPs(Vec<Mutex<IpAddr>>);

#[get("/count")]
fn count(user_count: &State<UserCount>) -> RawJson<String> {
	RawJson(format!(
		"{{\"user_count\": {}}}",
		user_count.count.load(Ordering::Relaxed)
	))
}

#[post("/count")]
fn inc_count(user_count: &State<UserCount>, sock: SocketAddr) {
	match user_count.ips.lock() {
		Ok(mut v) => {
			if v.contains(&sock.ip()) {
				println!("{} already said hi", &sock.ip());
			} else {
				user_count.count.fetch_add(1, Ordering::SeqCst);
				v.push(sock.ip());
				println!("new hello from {:?}", sock.ip());
			}
		},
		Err(e) => println!("error getting ips: {}", e),
	}
}

#[launch]
fn rocket() -> _ {
	rocket::build()
		.manage(UserCount{count: AtomicUsize::new(0), ips: Mutex::new(vec![])})
		// .manage(UserIPs(vec![]))
		.attach(Template::fairing())
		.mount("/dev", dev::routes())
		.mount("/", FileServer::from("public/"))
		.mount("/api", routes![count, inc_count])
}

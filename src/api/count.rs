use rocket::response::content::RawJson;
use rocket::serde::{Serialize, json::Json};
use rocket::State;
use std::net::{self, SocketAddr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::dev::User;

#[derive(Serialize)]
#[derive(Default)]
#[serde(crate = "rocket::serde")]
pub struct IPList {
	ips: Vec<net::IpAddr>,
}

pub struct UserCount {
	count: AtomicUsize,
	ips: Mutex<IPList>,
}

impl Default for UserCount {
	fn default() -> Self {
		UserCount {
			count: AtomicUsize::new(0),
			ips: Mutex::new(IPList::default()),
		}
	}
}

impl UserCount {
	pub fn count(&self) -> usize {
		self.count.load(Ordering::Relaxed)
	}

	pub fn ips(&self) -> Option<Vec<net::IpAddr>> {
		match self.ips.lock() {
			Ok(v) => Some(v.ips.to_vec()),
			Err(_) => None,
		}
	}
}

#[get("/count")]
pub fn count(user_count: &State<UserCount>) -> RawJson<String> {
	RawJson(format!(
		"{{\"user_count\": {}}}",
		user_count.count.load(Ordering::Relaxed)
	))
}

#[get("/count/ips")]
pub fn ips(_user: User, user_count: &State<UserCount>) -> Json<IPList> {
	match user_count.ips.lock() {
		Ok(v) => {
			Json(IPList {
				ips: v.ips.clone()
			})
		}
		_ => {
			Json(IPList {
				ips: vec![]
			})
		}
	}
}

#[post("/count")]
pub fn inc_count(user_count: &State<UserCount>, sock: SocketAddr) {
	match user_count.ips.lock() {
		Ok(mut v) => {
			if v.ips.contains(&sock.ip()) {
				println!("{} already said hi", &sock.ip());
				// can add ips more than once in debug
				if cfg!(debug_assertions) {
					user_count.count.fetch_add(1, Ordering::SeqCst);
					v.ips.push(sock.ip());
				}
			} else {
				user_count.count.fetch_add(1, Ordering::SeqCst);
				v.ips.push(sock.ip());
				println!("new hello from {:?}", sock.ip());
			}
		}
		Err(e) => println!("error getting ips: {}", e),
	}
}

#[post("/count/remove/<index>")]
pub fn remove_ip(_user: User, user_count: &State<UserCount>, index: usize) {
	match user_count.ips.lock() {
		Ok(mut v) => {
			v.ips.remove(index);
		}
		Err(e) => println!("error getting ips: {}", e),
	}
}

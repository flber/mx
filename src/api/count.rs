use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::response::content::RawJson;
use rocket::State;
use std::net::{self, SocketAddr};
use std::sync::Mutex;

pub struct UserCount {
	count: AtomicUsize,
	ips: Mutex<Vec<net::IpAddr>>,
}

impl Default for UserCount {
	fn default() -> Self {
		UserCount {
			count: AtomicUsize::new(0),
			ips: Mutex::new(vec![])
		}
	}
}

impl UserCount {
	pub fn count(&self) -> usize {
		self.count.load(Ordering::Relaxed)
	}

	pub fn ips(&self) -> Option<Vec<net::IpAddr>> {
		match self.ips.lock() {
			Ok(v) => Some(v.to_vec()),
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

#[post("/count")]
pub fn inc_count(user_count: &State<UserCount>, sock: SocketAddr) {
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

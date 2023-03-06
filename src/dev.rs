use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, FromRequest, Request};
use rocket::response::{Flash, Redirect};
use rocket::State;
use rocket::response::content::RawHtml;

use rocket_dyn_templates::{context, Template};

use crate::api::count::{UserCount, PublicIp};

use std::env;

#[derive(FromForm)]
struct Login<'r> {
	username: &'r str,
	password: &'r str,
}

#[derive(Debug)]
pub struct User(usize);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
	type Error = std::convert::Infallible;

	async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
		request
			.cookies()
			.get_private("user_id")
			.and_then(|cookie| cookie.value().parse().ok())
			.map(User)
			.or_forward(())
	}
}

#[macro_export]
macro_rules! session_uri {
    ($($t:tt)*) => (rocket::uri!("/dev", $crate::dev:: $($t)*))
}

pub use session_uri as uri;

#[get("/")]
fn index(_user: User, user_count: &State<UserCount>) -> Template {
	Template::render(
		"session",
		context! {
			user_count: user_count.count(),
			user_ips: match user_count.ips() {
				Some(ips) => ips,
				None => vec![],
			},
		},
	)
}

#[get("/", rank = 2)]
fn no_auth_index() -> Redirect {
	Redirect::to(uri!(login_page))
}

#[get("/login")]
fn login(_user: User) -> Redirect {
	Redirect::to(uri!(index))
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage<'_>>) -> Template {
	Template::render("login", &flash)
}
#[post("/login", data = "<login>")]
fn post_login(jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Result<Redirect, Flash<Redirect>> {
	if cfg!(debug_assertions) {
		jar.add_private(Cookie::new("user_id", 1.to_string()));
		Ok(Redirect::to(uri!(index)))
	} else {
		match (env::var("USER"), env::var("PASSWORD")) {
			(Ok(user), Ok(pw)) => {
				if login.username == user && login.password == pw {
					jar.add_private(Cookie::new("user_id", 1.to_string()));
					Ok(Redirect::to(uri!(index)))
				} else {
					Err(Flash::error(
						Redirect::to(uri!(login_page)),
						"Invalid username/password.",
					))
				}
			}
			_ => Err(Flash::error(
				Redirect::to(uri!(login_page)),
				"Failed to load environment variables.",
			)),
		}
	}
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> Flash<Redirect> {
	jar.remove_private(Cookie::named("user_id"));
	Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[post("/<_..>", rank = 2)]
fn skillissue(ip: PublicIp) -> RawHtml<&'static str> {
	let ip = match ip.0 {
		Some(public) => format!("{}", public),
		None => String::from(""),
	};
	println!("SKILL ISSUE DETECTED: {}", ip);

	RawHtml(
r#"<img src="images/skill-issue.gif">
skill issue."#,
	)
}

pub fn routes() -> Vec<rocket::Route> {
	routes![index, no_auth_index, login, login_page, post_login, logout, skillissue]
}

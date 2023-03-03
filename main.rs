use axum::{extract::State as AxumState, response::Html as AxumHtml, routing::get, Json, Router};
use std::{
	net::SocketAddr,
	sync::{Arc, Mutex},
};

mod html;
use html::*;

fn mp_parser() {
	let text = r#"
	head
		title My Website
		description Description of my website
		author Marko Petrlic
	body
		paragraph Welcome to My world
		button Refresh
			click /refresh
	"#
	.to_owned();

	println!("{text:?}");
}

pub struct StateData<T> {
	data: T,
	refresh: bool,
}

impl<T> StateData<T> {
	pub fn new(data: T) -> Self {
		Self { data, refresh: false }
	}
}

pub type State<T> = AxumState<Arc<Mutex<StateData<T>>>>;

#[tokio::main]
async fn main() {
	mp_parser();
	println!("Hello World!");

	let header = Header { title: String::from("My Title"), ..Default::default() };
	let mut body = Body::default();
	body.elements.push(Box::new(Button {
		text: String::from("This is a button"),
		action: String::from("/action"),
	}));
	let mut scripts = Scripts::default();
	scripts.elements.push(Box::new(EndpointScript));
	scripts.elements.push(Box::new(RefreshScript));

	let html = Html::new(header, body, scripts);

	let state = Arc::new(Mutex::from(StateData::new(html)));
	let app = Router::new()
		.route("/", get(root))
		.route("/action", get(action))
		.route("/refresh", get(refresh))
		.with_state(state);

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

// basic handler that responds with a static string
async fn root(state: State<Html>) -> AxumHtml<String> {
	state.lock().unwrap().data.build()
}

// basic handler that responds with a static string
async fn action(state: State<Html>) {
	dbg!("Hello World!");
	let mut state = state.lock().unwrap();
	state.refresh = true;
	state.data.header.title += &"aba";
}

// basic handler that responds with a static string
async fn refresh(state: State<Html>) -> Json<bool> {
	let mut state = state.lock().unwrap();
	let refresh = state.refresh;
	dbg!(state.refresh);
	state.refresh = false;
	Json::from(refresh)
}

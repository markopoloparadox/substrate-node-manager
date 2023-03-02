use axum::{extract::State, response::Html, routing::get, Json, Router};
use std::{
	net::SocketAddr,
	sync::{Arc, Mutex},
};

pub struct StateData<T> {
	data: T,
	refresh: bool,
}

impl<T> StateData<T> {
	pub fn new(data: T) -> Self {
		Self { data, refresh: false }
	}
}

pub struct ToHTML;
impl ToHTML {
	pub fn title(value: &str) -> String {
		format!("<title>{}</title>", value)
	}
	pub fn description(value: &str) -> String {
		format!(r#"<meta name="description" content="{}">"#, value)
	}
	pub fn author(value: &str) -> String {
		format!(r#"<meta name="author" content="{}">"#, value)
	}
	pub fn button(name: &str, action: &str) -> String {
		format!(r#"<button onclick="endpoint('{}')">{}</button>"#, action, name)
	}
	pub fn paragraph(value: &str) -> String {
		format!(r#"<p>{}</p>"#, value)
	}
	pub fn empty_line() -> String {
		format!(r#"<br/>"#)
	}
}

#[derive(Clone)]
pub struct Website {
	pub title: String,
	pub description: String,
	pub author: String,
	pub content: String,
}

fn add(input: &mut String, value: &str, iden: u8) {
	for i in 0..iden {
		input.push_str(" ")
	}
	input.push_str(value);
	input.push_str("\n");
}

impl Website {
	pub fn new() -> Self {
		Self {
			title: String::from("New Website"),
			description: String::from("Desc"),
			author: String::from("Marko Petrlic"),
			content: String::from("Empty"),
		}
	}

	pub fn build(&self) -> Html<String> {
		let mut html = "<!doctype html>".to_string();
		html.push_str(&"\n");
		add(&mut html, &self.generate_head(), 0);
		add(&mut html, r#"<body>"#, 0);
		add(&mut html, &ToHTML::button("Hello World", "/action"), 2);
		add(&mut html, &ToHTML::empty_line(), 2);
		add(&mut html, &ToHTML::paragraph("Omg!"), 2);
		add(&mut html, &ToHTML::paragraph(&self.content), 2);
		add(&mut html, &self.create_endpoint(), 2);
		add(&mut html, r#"</body>"#, 0);
		add(&mut html, r#"</html>"#, 0);

		Html(html)
	}

	pub fn magic(&mut self) {
		self.content.push_str("New String. ");
	}

	fn generate_head(&self) -> String {
		let mut head = String::from("<head>");
		head.push_str(&"\n");
		add(&mut head, r#"<meta charset="utf-8">"#, 2);
		add(&mut head, &ToHTML::title(&self.title), 2);
		add(&mut head, &ToHTML::description(&self.description), 2);
		add(&mut head, &ToHTML::author(&self.author), 2);
		add(&mut head, r#"</head>"#, 0);

		head
	}

	fn create_endpoint(&self) -> String {
		format!(
			r#"<script>
		function endpoint(arg) {{
			fetch("http://127.0.0.1:3000" + arg);
		}}
		function refresh() {{
			fetch("http://127.0.0.1:3000/refresh").then(
				(x) => x.json()
			).then((y) => {{
				if (y == true) {{
					window.location.reload(1);
				}}
			}});
		}}

		setInterval(function() {{
			refresh();
		 }}, 1000);
	</script>
		"#
		)
	}
}

#[tokio::main]
async fn main() {
	println!("Hello World!");

	let state = Arc::new(Mutex::from(StateData::new(Website::new())));
	let app = Router::new()
		.route("/", get(root))
		.route("/action", get(action))
		.route("/refresh", get(refresh))
		.with_state(state);

	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

// basic handler that responds with a static string
async fn root(state: State<Arc<Mutex<StateData<Website>>>>) -> Html<String> {
	state.lock().unwrap().data.build()
}

// basic handler that responds with a static string
async fn action(state: State<Arc<Mutex<StateData<Website>>>>) {
	dbg!("Hello World!");
	let mut state = state.lock().unwrap();
	state.refresh = true;
	state.data.title += "Hello";
}

// basic handler that responds with a static string
async fn refresh(state: State<Arc<Mutex<StateData<Website>>>>) -> Json<bool> {
	let mut state = state.lock().unwrap();
	let refresh = state.refresh;
	dbg!(state.refresh);
	state.refresh = false;
	Json::from(refresh)
}

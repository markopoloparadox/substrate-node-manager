use axum::response::Html as AxumHtml;

pub mod elements;
pub use elements::*;

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
}

pub struct RawHtml {
	pub inner: String,
}

impl RawHtml {
	pub fn new() -> Self {
		Self { inner: String::default() }
	}

	pub fn add(&mut self, value: &str, idn: u8) {
		for _ in 0..idn {
			self.inner.push_str(" ")
		}
		self.inner.push_str(value);
		self.inner.push_str("\n");
	}
}

pub struct Html {
	pub header: Header,
	pub body: Body,
	pub scripts: Scripts,
}

impl Html {
	pub fn new(header: Header, body: Body, scripts: Scripts) -> Self {
		Self { header, body, scripts }
	}

	pub fn build(&self) -> AxumHtml<String> {
		let mut html = RawHtml::new();
		let header = self.header.clone();
		let mut body = self.body.clone();
		let scripts = Box::new(self.scripts.clone());
		body.elements.push(scripts);

		html.add(r#"<!doctype html>"#, 0);
		html.add(&header.to_raw_html(), 0);
		html.add(&body.to_raw_html(), 0);
		html.add(r#"</html>"#, 0);

		AxumHtml::from(html.inner)
	}
}

pub trait ToRawHtml: Send + Sync {
	fn to_raw_html(&self) -> String;
	fn clone_dyn(&self) -> Box<dyn ToRawHtml>;
}

impl Clone for Box<dyn ToRawHtml> {
	fn clone(&self) -> Self {
		self.clone_dyn()
	}
}

#[derive(Default, Clone)]
pub struct Header {
	pub title: String,
	pub description: String,
	pub author: String,
}

impl ToRawHtml for Header {
	fn to_raw_html(&self) -> String {
		let mut html = RawHtml::new();
		html.add("<head>", 0);
		html.add(r#"<meta charset="utf-8">"#, 2);
		if !self.title.is_empty() {
			html.add(&ToHTML::title(&self.title), 2);
		}
		if !self.description.is_empty() {
			html.add(&ToHTML::description(&self.description), 2);
		}
		if !self.author.is_empty() {
			html.add(&ToHTML::author(&self.author), 2);
		}
		html.add(r#"</head>"#, 0);

		html.inner
	}

	fn clone_dyn(&self) -> Box<dyn ToRawHtml> {
		Box::new(self.clone())
	}
}

#[derive(Default, Clone)]
pub struct Body {
	pub elements: Vec<Box<dyn ToRawHtml>>,
}

impl ToRawHtml for Body {
	fn to_raw_html(&self) -> String {
		let mut html = RawHtml::new();
		html.add("<body>", 0);
		self.elements.iter().for_each(|e| html.add(&e.to_raw_html(), 2));
		html.add(r#"</body>"#, 0);

		html.inner
	}

	fn clone_dyn(&self) -> Box<dyn ToRawHtml> {
		Box::new(self.clone())
	}
}

#[derive(Default, Clone)]
pub struct Scripts {
	pub elements: Vec<Box<dyn ToRawHtml>>,
}

impl ToRawHtml for Scripts {
	fn to_raw_html(&self) -> String {
		let mut html = RawHtml::new();
		html.add(r#"<script>"#, 0);
		self.elements.iter().for_each(|e| html.add(&e.to_raw_html(), 2));
		html.add(r#"</script>"#, 0);

		html.inner
	}

	fn clone_dyn(&self) -> Box<dyn ToRawHtml> {
		Box::new(self.clone())
	}
}

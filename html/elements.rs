use super::ToRawHtml;

#[derive(Clone)]
pub struct EndpointScript;
impl ToRawHtml for EndpointScript {
	fn to_raw_html(&self) -> String {
		r#"	function endpoint(arg) {
		fetch("http://127.0.0.1:3000" + arg);
	}
		"#
		.to_owned()
	}

	fn clone_dyn(&self) -> Box<dyn ToRawHtml> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct RefreshScript;
impl ToRawHtml for RefreshScript {
	fn to_raw_html(&self) -> String {
		r#"	function refresh() {
		fetch("http://127.0.0.1:3000/refresh").then(
			(x) => x.json()
		).then((y) => {
			if (y == true) {
				window.location.reload(1);
			}
		});
	}

	setInterval(function() {
		refresh();
	}, 1000);
		"#
		.to_owned()
	}

	fn clone_dyn(&self) -> Box<dyn ToRawHtml> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Button {
	pub text: String,
	pub action: String,
}

impl ToRawHtml for Button {
	fn to_raw_html(&self) -> String {
		format!(r#"<button onclick="endpoint('{}')">{}</button>"#, self.action, self.text)
	}

	fn clone_dyn(&self) -> Box<dyn ToRawHtml> {
		Box::new(self.clone())
	}
}

#[derive(Clone)]
pub struct Paragraph {
	pub text: String,
}

impl ToRawHtml for Paragraph {
	fn to_raw_html(&self) -> String {
		self.text.clone()
	}

	fn clone_dyn(&self) -> Box<dyn ToRawHtml> {
		Box::new(self.clone())
	}
}

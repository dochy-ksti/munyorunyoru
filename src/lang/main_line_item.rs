use super::line_type::LineType;

pub(crate) struct MainLine{
	line_type : LineType,
	content : String,
	params : Vec<String>,
}

pub(crate) struct MainNormal{
	pub content : String,
	pub params : Vec<String>,
}

impl MainNormal {
    pub(crate) fn new(content: String, params: Vec<String>) -> Self { Self { content, params } }
}

pub(crate) struct MainSingle{
	pub content : String,
	pub param : String,
}

impl MainSingle {
    pub(crate) fn new(content: String, param: String) -> Self { Self { content, param } }
}

pub(crate) struct MainDouble{
	pub content : String,
	pub param : String,
}

impl MainDouble {
    pub(crate) fn new(content: String, param: String) -> Self { Self { content, param } }
}

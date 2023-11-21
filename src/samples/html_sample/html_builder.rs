pub struct HtmlBuilder{
	pub items : Vec<HtmlItem>
}

pub enum HtmlItem{
	Text(String),
	Tag(Tag)
}

pub struct Tag{
	pub name : String,
	pub params : Vec<Param>,
}

pub struct Param{
	pub name : String,
	pub value : String,
}

impl HtmlBuilder{
	
}
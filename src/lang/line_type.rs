pub(crate) enum LineType{
	Normal,
	Single,
	Double,
	CharSingle,
	CharDouble,
	CharTriple,
}

impl LineType{
	pub(crate) fn starting_text(&self) -> &str{
		match self{
			Self::Normal | Self::Single | Self::Double => "",
			Self::CharSingle => ">",
			Self::CharDouble => ">>",
			Self::CharTriple => ">>>",
		}
	}
}
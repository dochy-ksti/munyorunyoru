use crate::error::MunyoResult;

pub(crate) struct State{
	indent_level : usize,
	default_stack : Vec<DefaultTypes>,
	leveled_default : Vec<(String, String)>,
}

pub(crate) struct DefaultTypes{
	default_type : String,
	empty_line_type : String,
	indent_level : usize,
}

impl State{
	pub(crate) fn new() -> State{
		State { indent_level: 0, default_stack: vec![], leveled_default: vec![] }
	}

	pub(crate) fn indent(&mut self, level : usize) -> Result<(), String>{
		if self.indent_level + 2 <= level{
			Err("Indent is too deep.")?
		}
		self.indent_level = level;
		self.set_stacks_indent_level(level);
		Ok(())
	}

	fn set_stacks_indent_level(&mut self, level : usize){
		if level + 1 < self.default_stack.len(){
			unsafe{ self.default_stack.set_len(level + 1) }
		}
		if level + 1 < self.leveled_default.len(){
			unsafe{ self.leveled_default.set_len(level + 1) }
		}
	}

	pub(crate) fn set_default_items(&mut self, level : usize, 
		default_type_name : String, empty_line_type_name : String){
		self.default_stack.push(DefaultTypes::)

	}
}
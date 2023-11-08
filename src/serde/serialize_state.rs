use crate::lang::make_escaped_string::{self, make_escaped_string};

pub(crate) struct SerializeState {
    pub output: String,
    indent_level: usize,
    state: State,
}

#[derive(PartialEq)]
enum State {
    InStruct,
    InSeq,
    InArgs,
    EndOfArgs,
}

pub(crate) enum Er {
    None,
    Message(String),
}
type Result = std::result::Result<(), ()>;
type ResultS = std::result::Result<(), Er>;

fn message(s: &str) -> Er {
    Er::Message(s.to_string())
}

impl SerializeState {
    pub(crate) fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
            state: State::InStruct,
        }
    }

    pub(crate) fn start_seq(&mut self) -> Result {
        if self.state != State::InStruct {
            return Err(());
        }
        self.indent_level += 1;
        self.state = State::InSeq;
        Ok(())
    }
    pub(crate) fn end_seq(&mut self) {
        self.indent_level -= 1;
        self.output.push('\n');
        self.state = State::InStruct;
    }
    pub(crate) fn start_line(&mut self, name: &str) -> Result {
        if self.state != State::InSeq {
            return Err(());
        }
        for _ in 1..self.indent_level {
            self.output.push('\t');
        }
        self.output.push_str(name);
        self.state = State::InArgs;
        Ok(())
    }
	pub(crate) fn end_line(&mut self){
		self.output.push('\n');
		self.state = State::InSeq;
	}
    pub(crate) fn add_arg(&mut self, arg: String) -> ResultS {
        match self.state {
            State::InArgs => {}
            State::EndOfArgs => {
                return Err(message("no arguments allowed after string argument except one struct"))
            }
            _ => return Err(Er::None),
        }
		self.output.push(' ');
        self.output.push_str(&arg);
        Ok(())
    }
	pub(crate) fn add_str(&mut self, unescaped: &str) -> ResultS {
        match self.state {
            State::InArgs => self.state = State::EndOfArgs,
            State::EndOfArgs => {
                return Err(message("no arguments allowed after string argument except one struct"))
            }
            _ => return Err(Er::None),
        }
		self.output.push(' ');
        self.output.push_str(&make_escaped_string(unescaped));
        Ok(())
    }
}

use crate::lang::make_escaped_string::{self, make_escaped_string};

pub(crate) struct SerializeState {
    pub(crate) output: String,
    indent_level: usize,
    state: State,
}

#[derive(PartialEq)]
enum State {
    WfSeq,
    WfLine,
    WfArg,
    WfParam,
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
            state: State::WfSeq,
        }
    }

    pub(crate) fn start_seq(&mut self) -> Result {
        if self.state != State::WfSeq {
            return Err(());
        }
        self.indent_level += 1;
        self.state = State::WfLine;
        Ok(())
    }
    pub(crate) fn end_seq(&mut self) -> Result {
        match self.state {
            State::WfLine => {
                self.indent_level -= 1;
                self.output.push('\n');
                self.state = State::WfSeq;
                Ok(())
            }
            _ => Err(()),
        }
    }
    pub(crate) fn start_line(&mut self, name: &str) -> Result {
        if self.state != State::WfLine {
            return Err(());
        }
        for _ in 1..self.indent_level {
            self.output.push('\t');
        }
        self.output.push_str(name);
        self.state = State::WfArg;
        Ok(())
    }
    pub(crate) fn end_line(&mut self) -> Result {
        match self.state {
            State::WfArg | State::WfParam => {
                self.output.push('\n');
                self.state = State::WfLine;
                Ok(())
            }
            State::WfSeq | State::WfLine => Err(()),
        }
    }
    pub(crate) fn add_arg(&mut self, arg: &str) -> ResultS {
        match self.state {
            State::WfArg => {}
            _ => return Err(Er::None),
        }
        self.output.push(' ');
        self.output.push_str(arg);
        Ok(())
    }
    pub(crate) fn add_str(&mut self, unescaped: &str) -> ResultS {
        self.add_arg(&make_escaped_string(unescaped))
    }
    pub(crate) fn add_param(&mut self, name: &str, value: &str) -> ResultS {
        match self.state{
            State::WfArg | State::WfParam =>{},
            _ => return Err(message(&format!("param struct is not expected {name} {value}")))
        }
        self.output.push_str(&format!("|{name} {value}"));
        Ok(())
    }
    pub(crate) fn add_param_str(&mut self, name: &str, unescaped:&str) -> ResultS {
        self.add_param(name, &make_escaped_string(unescaped))
    }
}

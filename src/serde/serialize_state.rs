use crate::lang::make_escaped_string::make_escaped_string;

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
    WfParamKey,
    WfParamValue,
}

pub(crate) enum Er {
    None,
    Message(String),
}
type Result = std::result::Result<(), ()>;
type ResultS = std::result::Result<(), Er>;

use State::*;

impl SerializeState {
    pub(crate) fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
            state: WfSeq,
        }
    }

    pub(crate) fn start_seq(&mut self) -> Result {
        match self.state {
            WfSeq => {}
            WfArg | WfParamKey => self.output.push('\n'),
            WfLine | WfParamValue => return Err(()),
        }
        self.indent_level += 1;
        self.state = WfLine;
        Ok(())
    }
    pub(crate) fn end_seq(&mut self) -> Result {
        match self.state {
            WfLine => {
                self.indent_level -= 1;
                self.state = WfLine;
                Ok(())
            }
            _ => Err(()),
        }
    }
    pub(crate) fn start_line(&mut self, name: &str) -> Result {
        if self.state != WfLine {
            return Err(());
        }
        for _ in 1..self.indent_level {
            self.output.push('\t');
        }
        self.output.push_str(name);
        self.state = WfArg;
        Ok(())
    }
    pub(crate) fn end_line(&mut self) -> Result {
        match self.state {
            WfArg | WfParamKey | WfSeq => {
                self.output.push('\n');
                self.state = WfLine;
                Ok(())
            }
            WfLine => Ok(()),
            WfParamValue => Err(()),
        }
    }
    pub(crate) fn add_arg(&mut self, arg: &str) -> ResultS {
        match self.state {
            WfArg => {}
            WfParamValue => self.state = WfParamKey,
            WfParamKey => return Err(Er::Message(format!("param expected {arg}"))),
            _ => return Err(Er::None),
        }
        self.output.push(' ');
        self.output.push_str(arg);
        Ok(())
    }
    pub(crate) fn add_str(&mut self, unescaped: &str) -> ResultS {
        self.add_arg(&make_escaped_string(unescaped))
    }
    pub(crate) fn add_param_key(&mut self, name: &str) -> ResultS {
        match self.state {
            WfArg | WfParamKey => {}
            _ => return Err(Er::Message(format!("param struct is not expected {name}"))),
        }
        self.state = WfParamValue;
        self.output.push('|');
        self.output.push_str(name);
        Ok(())
    }
    pub(crate) fn add_none(&mut self) -> Result {
        match self.state {
            WfParamValue => {
                self.state = WfParamKey;
                while let Some(c) = self.output.pop() {
                    if c == '|' {
                        break;
                    }
                }
                Ok(())
            }
            _ => Err(()),
        }
    }
}

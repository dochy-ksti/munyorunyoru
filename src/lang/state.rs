



pub(crate) struct State {
    indent_level: usize,
    default_stack: Vec<DefaultTypes>,
    leveled_default: Vec<Option<(String, String)>>,
}

pub(crate) struct DefaultTypes {
    default_type: String,
    empty_line_type: String,
    indent_level: usize,
}

impl DefaultTypes {
    pub(crate) fn new(default_type: String, empty_line_type: String, indent_level: usize) -> Self {
        Self {
            default_type,
            empty_line_type,
            indent_level,
        }
    }
}

impl State {
    pub(crate) fn new() -> State {
        State {
            indent_level: 0,
            default_stack: vec![],
            leveled_default: vec![],
        }
    }
    pub(crate) fn indent_level(&self) -> usize {
        self.indent_level
    }

    pub(crate) fn set_indent(&mut self, indent_level: usize) -> Result<(), String> {
        if self.indent_level + 2 <= indent_level {
            Err("Indent is too deep.")?
        }
        self.indent_level = indent_level;
        self.set_stacks_indent_level(indent_level);
        if indent_level + 1 < self.leveled_default.len() {
            unsafe { self.leveled_default.set_len(indent_level + 1) }
        }
        Ok(())
    }

    fn set_stacks_indent_level(&mut self, level: usize) {
        while level < self.peek_indent().unwrap_or(0) {
            self.default_stack.pop();
        }
    }

    fn peek_indent(&self) -> Option<usize> {
        self.default_stack.last().map(|item| item.indent_level)
    }

    pub(crate) fn set_doubled_default_types(
        &mut self,
        indent_level: usize,
        default_type: String,
        empty_line_type: String,
    ) {
        if self.peek_indent() == Some(indent_level) {
            self.default_stack.pop();
        }

        self.default_stack.push(DefaultTypes::new(
            default_type,
            empty_line_type,
            indent_level,
        ))
    }

    pub(crate) fn set_single_default_types(
        &mut self,
        indent_level: usize,
        default_type: String,
        empty_line_type: String,
    ) {
        while self.leveled_default.len() <= indent_level {
            self.leveled_default.push(None);
        }

        self.leveled_default[indent_level] = Some((default_type, empty_line_type))
    }

    pub(crate) fn default_types(&self) -> (&str, &str) {
        if let Some(Some((def, emp))) = self.leveled_default.get(self.indent_level) {
            return (def, emp);
        }
        if let Some(last) = self.default_stack.last() {
            return (&last.default_type, &last.empty_line_type);
        }
        ("", "")
    }
}

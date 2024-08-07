pub(crate) struct Arguments {
    s: Vec<u8>,
}

impl Arguments {
    pub(crate) fn new(s: &str) -> Self {
        Self {
            s: s.bytes().rev().collect(),
        }
    }

    /// may be empty
    ///
    /// Args are splitted by whitespaces.
    /// This method takes an arg and moves the cursor to the next arg.
    pub(crate) fn arg(&mut self) -> String {
        discard_spaces(&mut self.s);

        let s = get_nonspace(&mut self.s);
        discard_space(&mut self.s);
        s
    }

    /// May be empty.
    ///
    /// All remained arguments including whitespaces are returned.
    pub(crate) fn rest_mut(&mut self) -> String {
        self.s.reverse();
        let r = std::mem::take(&mut self.s);

        unchecked(r)
    }

    pub(crate) fn copy_rest(&self) -> String{
		let mut s = self.s.clone();
		s.reverse();

		unchecked(s)
	}

    pub(crate) fn is_empty(&self) -> bool {
        self.s.is_empty()
    }
}

fn unchecked(vec: Vec<u8>) -> String {
    unsafe { String::from_utf8_unchecked(vec) }
}

fn discard_spaces(s: &mut Vec<u8>) {
    loop {
        if s.is_empty() {
            return;
        }
        let c = s.last().unwrap();
        if c == &b' ' {
            s.pop();
        } else {
            return;
        }
    }
}

fn discard_space(s: &mut Vec<u8>) {
    if s.is_empty() {
        return;
    }
    let c = s.last().unwrap();
    if c == &b' ' {
        s.pop();
    }
}

fn get_nonspace(s: &mut Vec<u8>) -> String {
    let mut r: Vec<u8> = vec![];
    loop {
        if s.is_empty() {
            return unchecked(r);
        }

        let c = s.last().unwrap();
        if c == &b' ' {
            return unchecked(r);
        } else {
            r.push(s.pop().unwrap());
        }
    }
}

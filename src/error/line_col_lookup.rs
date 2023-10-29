pub(crate) struct LineColLookup {
    src_len: usize,
    line_heads: Vec<usize>,
}

pub(crate) struct LineColResult {
    pub line: usize,
    pub col: usize,
    pub line_start: usize,
    pub line_end: usize,
}

impl LineColLookup {
    pub fn new(src: &str) -> Self {
        Self {
            src_len: src.len(),
            line_heads: Self::heads(src),
        }
    }

    fn heads(s: &str) -> Vec<usize> {
        let s = s.as_bytes();
        let mut vec = vec![];
        for i in 0..s.len() {
            if s[i] == b'\n' {
                vec.push(i);
            } else if s[i] == b'\r' && s.get(i + 1) != Some(&b'\n') {
                vec.push(i)
            }
        }
        vec
    }

    pub fn line_col(&self, index: usize) -> Result<LineColResult, String> {
        if index > self.src_len {
            Err("Index cannot be greater than the length of the input slice.")?
        }

        let heads = &self.line_heads;
        //row = line_index + 1
        let line_index = heads.binary_search(&index).map_or_else(|e| e, |v| v);
        let line_start = if line_index == 0 {
            0
        } else {
            heads[line_index - 1] + 1
        };
        let line_end = if line_index == heads.len() {
            self.src_len
        } else {
            heads[line_index] + 1
        };
        let col = index + 1 - line_start;

        Ok(LineColResult {
            line: line_index + 1,
            col,
            line_start,
            line_end,
        })
    }

    fn _get(&self, index: usize) -> (usize, usize) {
        let r = self.line_col(index).unwrap();
        (r.line, r.col)
    }
}

#[cfg(test)]
mod tests {
    use crate::error::line_col_lookup::LineColLookup;

    #[test]
    fn empty_str() {
        let text = "";
        let lookup = LineColLookup::new(text);
        assert_eq!(lookup._get(0), (1, 1));
    }

    #[test]
    fn line_col_iter_by_codepoints() {
        let text = "a\nab\nabc";
        let lookup = LineColLookup::new(text);
        assert_eq!(lookup._get(0), (1, 1));
        assert_eq!(lookup._get(1), (1, 2));
        assert_eq!(lookup._get(2), (2, 1));
        assert_eq!(lookup._get(3), (2, 2));
        assert_eq!(lookup._get(4), (2, 3));
        assert_eq!(lookup._get(5), (3, 1));
        assert_eq!(lookup._get(6), (3, 2));
        assert_eq!(lookup._get(7), (3, 3));
        assert_eq!(lookup._get(8), (3, 4));
    }
}

use std::sync::OnceLock;

use regex::Regex;

pub(crate) fn process_file_text(lines: Vec<String>) {
    static RE: OnceLock<Regex> = OnceLock::new();
    //static lang_pat = r"([^|]*)(|
    let regex = RE.get_or_init(|| {
        Regex::new(r"[a-zA-Z0-9_]+( ([^|]+))?(|[a-zA-Z0-9_]+( ([^|]+))?)*$").unwrap()
    });
    for (line_num, line) in lines.into_iter().enumerate() {}
}

fn process_line(line_num : usize, indent : usize, lines : &[String], regex : &Regex){

}

fn check_indent_and_concat(s : &str) -> (usize, bool){
	unimplemented!()
}
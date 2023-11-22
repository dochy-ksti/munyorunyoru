use crate::error::parse_fail::ParseFail;

use super::{
    munyo_parser::{Pair, Pairs, Rule},
    parse_content::parse_content,
    parse_main_line::{parse_param_item, LineResult},
};

pub(crate) struct LcnResult {
    pub(crate) content: String,
    pub(crate) params: Vec<String>,
    pub(crate) new_line: String,
}

impl LcnResult {
    // pub(crate) fn new(content: String, params: Vec<String>, new_line: NewLineCode) -> Self {
    //     Self {
    //         content,
    //         params,
    //         new_line,
    //     }
    // }

    pub(crate) fn new(r: LineResult, new_line: String) -> Self {
        Self {
            content: r.content,
            params: r.params,
            new_line,
        }
    }
}

pub(crate) fn set_results(
    content: &mut String,
    params: &mut Vec<String>,
    r_content: &str,
    r_params: &mut Vec<String>,
) {
    if let Some(last) = params.last_mut() {
        last.push_str(r_content)
    } else {
        content.push_str(r_content);
    }
    params.append(r_params);
}

pub(crate) fn parse_line_continuation(pair: Pair) -> Result<LineResult, ParseFail> {
    match pair.as_rule() {
        Rule::normal_end => Ok(LineResult {
            content: String::new(),
            params: parse_normal_end(pair.into_inner())?.params,
            define_canceled : false,
        }),
        Rule::backslash_comment_end => {
            let mut r = parse_backslash_comment_end(pair.into_inner())?;
            r.content.insert_str(0, &r.new_line);
            Ok(LineResult {
                content: r.content,
                params: r.params,
                define_canceled : false,
            })
        }
        Rule::backslash_end => {
            let mut r = parse_backslash_end(pair.into_inner())?;
            r.content.insert_str(0, &r.new_line);
            Ok(LineResult {
                content: r.content,
                params: r.params,
                define_canceled : false,
            })
        }
        Rule::single_bar => parse_single_bar(pair.into_inner()),
        Rule::triple_bars => parse_triple_bars(pair.into_inner()),
        Rule::double_bars => Ok(LineResult {
            content: String::new(),
            params: parse_double_bars(pair.into_inner())?.params,
            define_canceled : false,
        }),
        _ => unreachable!(),
    }
}

fn parse_normal_end(mut pairs: Pairs) -> Result<Params, ParseFail> {
    let _newline = pairs.next();
    let continuation = pairs.next().unwrap();
    parse_continuation(continuation.into_inner())
}

struct Params {
    pub(crate) params: Vec<String>,
}

fn parse_continuation(mut pairs: Pairs) -> Result<Params, ParseFail> {
    let _tabs = pairs.next();
    parse_continued_line_without_content(pairs)
}

fn parse_backslash_comment_end(mut pairs: Pairs) -> Result<LcnResult, ParseFail> {
    let _comment = pairs.next();
    parse_backslash_end(pairs)
}

fn parse_backslash_end(mut pairs: Pairs) -> Result<LcnResult, ParseFail> {
    let new_line = pairs.next().unwrap().as_str().to_string();
    let continued_line = pairs.next().unwrap();
    let r = parse_continued_line(continued_line.into_inner())?;
    Ok(LcnResult::new(r, new_line))
}

fn parse_single_bar(mut pairs: Pairs) -> Result<LineResult, ParseFail> {
    let _new_line = pairs.next();
    parse_continued_line(pairs.next().unwrap().into_inner())
}

fn parse_double_bars(mut pairs: Pairs) -> Result<Params, ParseFail> {
    let _comment = pairs.next();
    if pairs.peek().is_some() {
        parse_normal_end(pairs)
    } else {
        Ok(Params { params: vec![] })
    }
}

fn parse_triple_bars(mut pairs: Pairs) -> Result<LineResult, ParseFail> {
    let _comment = pairs.next();
    parse_single_bar(pairs)
}

fn parse_continued_line(mut pairs: Pairs) -> Result<LineResult, ParseFail> {
    let _tabs = pairs.next();
    let p = pairs.next().unwrap();
    match p.as_rule() {
        Rule::continued_line_with_content => parse_continued_line_with_content(p.into_inner()),
        Rule::continued_line_without_content => {
            let params = parse_continued_line_without_content(p.into_inner())?;
            Ok(LineResult {
                content: String::new(),
                params: params.params,
                define_canceled : false,
            })
        }
        _ => unreachable!(),
    }
}

fn parse_continued_line_with_content(mut pairs: Pairs) -> Result<LineResult, ParseFail> {
    let mut content = parse_content(pairs.next().unwrap().into_inner(), "")?;
    let mut params = vec![];
    for p in pairs {
        match p.as_rule() {
            Rule::param_item => {
                params.push(parse_param_item(p.into_inner().next().unwrap())?);
            }
            Rule::line_continuation => {
                let mut r = parse_line_continuation(p.into_inner().next().unwrap())?;
                set_results(&mut content, &mut params, &r.content, &mut r.params);
            }
            _ => unreachable!(),
        }
    }
    Ok(LineResult::new(content, params, false))
}

fn parse_continued_line_without_content(pairs: Pairs) -> Result<Params, ParseFail> {
    let mut vec = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::param_item => vec.push(parse_param_item(pair.into_inner().next().unwrap())?),
            Rule::line_continuation => {
                let mut r = parse_line_continuation(pair)?;
                if let Some(last) = vec.last_mut() {
                    last.push_str(&r.content);
                } else {
                    unreachable!();
                }
                vec.append(&mut r.params);
            }
            _ => unreachable!(),
        }
    }
    Ok(Params { params: vec })
}

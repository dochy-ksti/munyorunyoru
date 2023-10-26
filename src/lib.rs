mod converters;
pub mod error;
mod file_io;
mod lang;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{error::MunyoResult, lang::process_file_text::process_file_text};

    #[test]
    fn it_works() -> MunyoResult<()> {
        let unparsed_file = fs::read_to_string("sample.munyo").expect("cannot read file");
        process_file_text(unparsed_file)?;
        Ok(())
        // let file = CSVParser::parse(Rule::file, &unparsed_file)
        //     .expect("unsuccessful parse")
        //     .next()
        //     .unwrap(); //this unwrap never fails

        // let mut field_sum: f64 = 0.0;
        // let mut record_count: u64 = 0;

        // for record in file.into_inner() {
        //     match record.as_rule() {
        //         Rule::record => {
        //             record_count += 1;

        //             for field in record.into_inner() {
        //                 field_sum += field.as_str().parse::<f64>().unwrap();
        //             }
        //         }
        //         Rule::EOI => (),
        //         _ => unreachable!(),
        //     }
        // }

        // println!("Sum of fields: {field_sum}");
        // println!("Number of records: {record_count}");
    }
}

use std::io::{BufReader, Read};

use crate::error::Result;
use crate::line_reader::line_reader;
use crate::GSHeader;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub struct GSProperty {
    pub header: GSHeader,
    pub properties: Box<[Box<[f32]>]>,
}

impl GSProperty {
    pub fn from_gs_header<R: Read>(header: GSHeader, reader: BufReader<R>) -> Result<GSProperty> {
        let properties = GSProperty::load_properties(header.values_number, reader)?;

        Ok(GSProperty { header, properties })
    }

    fn load_properties<R: Read>(
        properties_num: usize,
        reader: BufReader<R>,
    ) -> Result<Box<[Box<[f32]>]>> {
        let mut reader = reader;
        let mut result = vec![Vec::new(); properties_num];

        let mut line_counter: usize = 1;
        loop {
            let line = line_reader(&mut reader)
                .map_err(|e| format!("while reading property section line #{}: {}", line_counter, e))?;
            let Some(line) = line else {
                break;
            };
            let parsed_line: Vec<&str> = line.split_ascii_whitespace().collect();
            if parsed_line.len() != properties_num {
                return Err(format!(
                    "while reading property section line #{}: got {} values, must be {} values",
                    line_counter,
                    parsed_line.len(),
                    properties_num
                )
                .into());
            };
            for pi in 0..properties_num {
                let col_str = parsed_line[pi];
                let v: f32 = match col_str.parse::<f32>() {
                    Ok(p) => p,
                    Err(e) => {
                        return Err(format!(
                            "while reading property section line #{} got <{}>: {e}",
                            line_counter,
                            col_str,
                        )
                        .into());
                    },
                };
                result[pi].push(v);
            }
            line_counter += 1;
        } //loop
        let result: Vec<Box<[f32]>> = result.into_iter().map(|item| item.into_boxed_slice()).collect();
        Ok(result.into_boxed_slice())
    }
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod basic {
    use super::*;
    use crate::Result;

    #[test]
    fn simple() -> Result<()> {
        let s = "title\n\
                 2\n\
                 i\n\
                 j\n\
                 3. 4\n\
                 1 3\n\
                 5\t2\n\
                 ";
        let reader = BufReader::new(s.as_bytes());
        let (header, reader) = GSHeader::from_reader(reader)?;
        let prop = GSProperty::from_gs_header(header, reader)?;
        assert!(prop.properties.len() == 2);
        let len0 = prop.properties[0].len();
        assert!(len0 == 3, "len0 should be 3, got {len0}");
        let len1 = prop.properties[0].len();
        assert!(len1 == len0, "len1 should be equal to len0={len0}, got {len1}");
        Ok(())
    }

    #[test]
    fn err_values_invalide() -> Result<()> {
        let s = "title\n\
                 2\n\
                 i\n\
                 j\n\
                 3. w\n\
                 5\t2\n\
                 ";
        let reader = BufReader::new(s.as_bytes());
        let (header, reader) = GSHeader::from_reader(reader)?;
        let Err(e) = GSProperty::from_gs_header(header, reader) else {
            return Err("must be Err!".into());
        };
        let c = "while reading property section line #1 got <w>: invalid float literal";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
    #[test]
    fn err_values_num() -> Result<()> {
        let s = "title\n\
                 2\n\
                 i\n\
                 j\n\
                 3. 3\n\
                 1\n\
                 5\t2\n\
                 ";
        let reader = BufReader::new(s.as_bytes());
        let (header, reader) = GSHeader::from_reader(reader)?;
        let Err(e) = GSProperty::from_gs_header(header, reader) else {
            return Err("must be Err!".into());
        };
        let c = "while reading property section line #2: got 1 values, must be 2 values";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
}

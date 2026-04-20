use std::io::{BufReader, Read};

use crate::error::Result;
use crate::line_reader::line_reader;
use crate::GSHeader;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub struct GSProperty {
    pub header: GSHeader,
    pub properties: Vec<Vec<String>>,
}

impl GSProperty {
    pub fn from_gs_header<R: Read>(header: GSHeader, reader: BufReader<R>) -> Result<GSProperty> {
        let properties = GSProperty::load_properties(header.values_number, reader)?;

        Ok(GSProperty { header, properties })
    }

    fn load_properties<R: Read>(
        properties_num: usize,
        reader: BufReader<R>,
    ) -> Result<Vec<Vec<String>>> {
        let mut reader = reader;
        let mut result = vec![Vec::new(); properties_num];

        let mut counter: usize = 1;
        loop {
            let line = line_reader(&mut reader)
                .map_err(|e| format!("while reading properties line #{}: {}", counter, e))?;
            let Some(line) = line else {
                break;
            };
            let parsed_line: Vec<&str> = line.split_ascii_whitespace().collect();
            if parsed_line.len() != properties_num {
                return Err(format!(
                    "while reading properties line #{}: got {} values, must be {} values",
                    counter,
                    parsed_line.len(),
                    properties_num
                )
                .into());
            };
            /*
            let Ok(value) = parsed_line[3].parse::<T>() else {
                return Err(format!(
                    "Unable to parse <{}> as value #{}",
                    parsed_line[3],
                    counter + 1
                )
                .into());
            };
            */
            for pi in 0..properties_num {
                result[pi].push(parsed_line[pi].into());
            }
            counter += 1;
        } //loop
        Ok(result)
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
        assert!(prop.properties[0].len() == 3);
        assert!(prop.properties[1].len() == 3);
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
        let c = "while reading properties line #2: got 1 values, must be 2 values";
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
        let c = "while reading properties line #2: got 1 values, must be 2 values";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
}

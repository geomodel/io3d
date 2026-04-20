use std::{
    fs::File,
    io::{
        Read,
        BufReader,
    },
};

use crate::error::Result;
use crate::line_reader::line_reader;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub struct GSHeader {
    #[allow(dead_code)]
    pub title: String,
    pub values_number: usize,
    #[allow(dead_code)]
    pub descriptions: Box<[Box<str>]>,
}

impl GSHeader {
    pub fn from_filename(file_name: &str) -> Result<(GSHeader, BufReader<File>)> {
        let f = File::open(file_name)?;
        let r = BufReader::new(f);
        GSHeader::from_reader(r)
    }

    pub fn from_reader<R: Read>(reader: BufReader<R>) -> Result<(GSHeader, BufReader<R>)> {
        let mut reader = reader;
        let title = line_reader(&mut reader)
            .map_err(|e| format!("while reading Header title: {}", e))?
            .ok_or(format!("while reading Header title: no data"))?;

        let Ok(values_number) = line_reader(&mut reader)
            .map_err(|e| format!("while reading Header properies number: {}", e))?
            .ok_or(format!("while reading Header properies number: no data"))?
            .parse::<usize>() else {
            return Err("invalid format Header values number".into());
        };
        if values_number == 0 {
            return Err("zero Header values number".into());
        }

        let mut descriptions = Vec::new();
        for i in 0..values_number {
            let description = line_reader(&mut reader)
                .map_err(|e| format!("while reading Header property description #{}: {}", i + 1, e))?
                .ok_or(format!("while reading Header property description #{}: no data", i + 1))?;
            descriptions.push(description.into_boxed_str());
        }

        Ok(
            (
                GSHeader {
                    title,
                    values_number,
                    descriptions: descriptions.into_boxed_slice(),
                },
                reader,
            )
        )
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
    fn err_header_no_title() ->Result<()> {
        let title = "some title";
        let values_num:usize = 2;
        let descr_1 = String::from("desc 1").into_boxed_str();
        let descr_2 = String::from("desc 2").into_boxed_str();
        let s = format!("{title}\n{values_num}\n{descr_1}\n{descr_2}");
        let reader = BufReader::new(s.as_bytes());
        let (header, _reader) = GSHeader::from_reader(reader)?;
        assert!(header.title == title);
        assert!(header.values_number == values_num);
        assert!(header.descriptions.len() == values_num);
        assert!(header.descriptions == vec![descr_1,descr_2].into_boxed_slice());
        Ok(())
    }
}

#[cfg(test)]
mod errors {
    use super::*;
    use crate::Result;

    #[test]
    fn err_header_no_title() ->Result<()> {
        let s = "";
        let reader = BufReader::new(s.as_bytes());
        let header = GSHeader::from_reader(reader);
        let Err(e) = header else {
            return Err("must be Err!".into());
        };
        let c = "while reading Header title: no data";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
    #[test]
    fn err_header_no_values_number() ->Result<()> {
        let s = "some title";
        let reader = BufReader::new(s.as_bytes());
        let header = GSHeader::from_reader(reader);
        let Err(e) = header else {
            return Err("must be Err!".into());
        };
        let c = "while reading Header properies number: no data";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
    #[test]
    fn err_header_invalid_values_number() ->Result<()> {
        let s = "some title\ng";
        let reader = BufReader::new(s.as_bytes());
        let header = GSHeader::from_reader(reader);
        let Err(e) = header else {
            return Err("must be Err!".into());
        };
        let c = "invalid format Header values number";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
    #[test]
    fn err_header_zero_values_number() ->Result<()> {
        let s = "some title\n0";
        let reader = BufReader::new(s.as_bytes());
        let header = GSHeader::from_reader(reader);
        let Err(e) = header else {
            return Err("must be Err!".into());
        };
        let c = "zero Header values number";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
    #[test]
    fn err_header_invalid_descriptions_1() ->Result<()> {
        let s = "some title\n1";
        let reader = BufReader::new(s.as_bytes());
        let header = GSHeader::from_reader(reader);
        let Err(e) = header else {
            return Err("must be Err!".into());
        };
        let c = "while reading Header property description #1: no data";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
    #[test]
    fn err_header_invalid_descriptions_2() ->Result<()> {
        let s = "some title\n2\ndescr1";
        let reader = BufReader::new(s.as_bytes());
        let header = GSHeader::from_reader(reader);
        let Err(e) = header else {
            return Err("must be Err!".into());
        };
        let c = "while reading Header property description #2: no data";
        let e = format!("{e}");
        assert!(e == c, "Err must be: {:?}\nbut got: {:?}", c, e);
        Ok(())
    }
}

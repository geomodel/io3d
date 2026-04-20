use std::{
    io::{
        BufReader,
        BufRead,
    },
};

use crate::Result;

//  //  //  //  //  //  //  //
pub(crate) fn line_reader<R>(reader: &mut BufReader<R>) -> Result<Option<String>>
where
    R: std::io::Read,
{
    let mut line = String::new();
    loop {
        reader.read_line(&mut line)?;
        if line.is_empty() {
            return Ok(None);
            //return Err("no data to read".into());
        }
        line.retain(|ch| !"\n\r".contains(ch));
        if !line.is_empty() {
            break;
        }
    }
    Ok(Some(line))
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod line_reader {
    use super::*;
    use std::io::BufReader;

    #[test]
    fn arounded_line() -> Result<()> {
        let s = "\n\nsome data\n\nafter next data\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let ln1 = line_reader(&mut reader)?.expect("none string");
        let ln2 = line_reader(&mut reader)?.expect("none string");
        assert!(ln1 == "some data");
        assert!(ln2 == "after next data");
        Ok(())
    }
    #[test]
    fn single_line() -> Result<()> {
        let s = "some data";
        let mut reader = BufReader::new(s.as_bytes());
        let result = line_reader(&mut reader)?.unwrap();
        assert!(result == s);
        Ok(())
    }

    #[test]
    fn none_data() -> Result<()> {
        let s = "";
        let mut reader = BufReader::new(s.as_bytes());
        let result = line_reader(&mut reader)?;
        assert!(result.is_none(), "must be None!");
        Ok(())
    }
    #[test]
    fn empty_line() -> Result<()> {
        let s = "\n\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let result = line_reader(&mut reader)?;
        assert!(result.is_none(), "must be None!");
        Ok(())
    }
}

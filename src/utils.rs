use anyhow::Result;

use std::io::{BufRead, BufReader};

//  //  //  //  //  //  //  //
pub(crate) fn line_reader<R>(reader: &mut BufReader<R>, line_type_name: &str) -> Result<String>
where
    R: std::io::Read,
{
    let mut line = String::new();
    loop {
        reader.read_line(&mut line)?;
        if line.is_empty() {
            return Err(anyhow::anyhow!("No data for reading <{}>", line_type_name));
        }
        line.retain(|ch| !"\n\r".contains(ch));
        if !line.is_empty() {
            break;
        }
    }
    Ok(line)
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod line_reader {
    use super::*;

    #[test]
    fn arounded_line() -> Result<()> {
        let s = "\n\nsome data\n\nafter next data\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let ln1 = line_reader(&mut reader, "ln1")?;
        let ln2 = line_reader(&mut reader, "ln2")?;
        assert!(ln1 == "some data");
        assert!(ln2 == "after next data");
        Ok(())
    }
    #[test]
    fn single_line() -> Result<()> {
        let s = "some data";
        let mut reader = BufReader::new(s.as_bytes());
        let result = line_reader(&mut reader, "test")?;
        assert!(result == s);
        Ok(())
    }

    #[test]
    fn empty_line_2_error() {
        let s = "";
        let mut reader = BufReader::new(s.as_bytes());
        let result = line_reader(&mut reader, "");
        assert!(result.is_err(), "must be Error!");
    }
    #[test]
    fn empty_line_error() {
        let s = "\n\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let result = line_reader(&mut reader, "");
        assert!(result.is_err(), "must be Error!");
    }
}

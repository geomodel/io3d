use anyhow::Result as Result;

use std::io::{BufReader,BufRead};
use std::fs::File;

//  //  //  //  //  //  //  //
fn get_next_not_empty_string<R>(reader: &mut BufReader<R>) -> Result<String>
where R: std::io::Read
{
    let mut line = String::new();
    loop {
        reader.read_line(&mut line)?;
        if line.is_empty() {
            return Ok( line );
        }
        line.retain( |ch| !"\n\r".contains(ch) );
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
mod tests {
    use super::*;

    #[test]
    fn continue_values() -> Result<()> {
        let s = "\n\n1.3\n\n0.3\n-999\n\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let arr = read_continue_values(&mut reader, 3)?;
        assert!(arr.len() == 3);
        Ok(())
    }

    #[test]
    fn discrete_values() -> Result<()> {
        let s = "\n\n1\n\n0\n-999\n\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let arr = read_discrete_values(&mut reader, 3)?;
        assert!(arr.len() == 3);
        Ok(())
    }

    #[test]
    fn bool_values() -> Result<()> {
        let s = "\n\n1\n\n0\n1\n\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let arr = read_bool_values(&mut reader, 3)?;
        assert!(arr.len() == 3);
        Ok(())
    }

    #[test]
    fn header_values_description() -> Result<()> {
        let s = "\n\ni j k\n\nvalues\n\n\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let descriptions = read_header_values_description(&mut reader, 2)?;
        assert!(descriptions.len() == 2);
        Ok(())
    }

    #[test]
    fn error_empty_header_values_number() {
        let s = "\n\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let result = read_header_values_number(&mut reader);
        assert!(result.is_err(), "must get the error!");
    }
    #[test]
    fn error_invalid_header_values_number() {
        let s = "\n\n1.\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let result = read_header_values_number(&mut reader);
        assert!(result.is_err(), "must get the error!");
    }
    #[test]
    fn error_zero_header_values_number() {
        let s = "\n\n0\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let result = read_header_values_number(&mut reader);
        assert!(result.is_err(), "must get the error!");
    }
    #[test]
    fn header_values_number() -> Result<()> {
        let s = "\n\n4\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let count = read_header_values_number(&mut reader)?;
        assert!(count == 4);
        Ok(())
    }

    #[test]
    fn error_empty_header_title() {
        let s = "\n\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let result = read_header_title(&mut reader);
        assert!(result.is_err(), "must get the error!");
    }
    #[test]
    fn header_title() -> Result<()> {
        let s = "\n\nGeoModel: Property\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let _ = read_header_title(&mut reader)?;
        Ok(())
    }
}

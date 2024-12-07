use anyhow::Result;
use std::io::BufReader;

use crate::utils::line_reader;

//  //  //  //  //  //  //  //
pub(crate) fn read_bool<R>(reader: &mut BufReader<R>, size: usize) -> Result<Vec<bool>>
where
    R: std::io::Read,
{
    let mut result = Vec::<bool>::new();
    result.try_reserve_exact(size)?;

    for i in 0..size {
        let line = line_reader(reader, &format!("Value #{}", i + 1))?;
        match line.as_str() {
            "0" => result.push(false),
            "1" => result.push(true),
            _ => return Err(anyhow::anyhow!("Unable to parse #{} <{}> as BOOL", i, line)),
        }
    }
    Ok(result)
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod read_bool {
    use super::*;

    #[test]
    fn no_values_error() {
        let s = "\n\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_bool(&mut reader, 1);
        assert!(values.is_err(), "must get the error!");
    }
    #[test]
    fn invalid_values_error() {
        let s = "\n2\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_bool(&mut reader, 1);
        assert!(values.is_err(), "must get the error!");
    }

    #[test]
    fn trhee_values() -> Result<()> {
        let s = "\n\n\n0\n\n\n\n1\n0\n2\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_bool(&mut reader, 3)?;
        assert!(values.len() == 3);
        assert!(values[0] == false);
        assert!(values[1] == true);
        assert!(values[2] == false);
        Ok(())
    }
}

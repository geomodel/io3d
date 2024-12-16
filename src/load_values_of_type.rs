use anyhow::Result;
use std::io::BufReader;

use crate::utils::line_reader;

//  //  //  //  //  //  //  //
pub(crate) fn read_values<R, T>(
    reader: &mut BufReader<R>,
    size: usize,
    undef_value: &str,
) -> Result<Box<[Option<T>]>>
where
    R: std::io::Read,
    T: std::str::FromStr,
{
    let mut result = Vec::<Option<T>>::with_capacity(size);

    for i in 0..size {
        let line = line_reader(reader, &format!("Value #{}", i + 1))?;
        if line == undef_value {
            result.push(None);
        } else {
            let Ok(value) = line.parse::<T>() else {
                return Err(anyhow::anyhow!(
                    "Unable to parse #{} <{}> as value",
                    i,
                    line
                ));
            };
            result.push(Some(value));
        }
    }
    Ok(result.into_boxed_slice())
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod read_values_of_type {
    use super::*;
    type Continuous = f64;
    type Discrete = i16;

    #[test]
    fn no_values_error() {
        let s = "\n\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_values::<&[u8], Discrete>(&mut reader, 1, "-999");
        assert!(values.is_err(), "must get the error!");
    }
    #[test]
    fn invalid_values_error() {
        let s = "\n2.\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_values::<&[u8], Discrete>(&mut reader, 1, "-999");
        assert!(values.is_err(), "must get the error!");
    }

    #[test]
    fn integer_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1\n0\n5\nunreachable\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_values::<&[u8], Continuous>(&mut reader, 4, "-999")?;
        assert!(values.len() == 4);
        assert!(values[0] == None);
        assert!(values[1] == Some(1.));
        assert!(values[2] == Some(0.));
        assert!(values[3] == Some(5.));
        Ok(())
    }

    #[test]
    fn countinues_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1.0\n0.3\n5\nunreachable\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_values::<&[u8], Continuous>(&mut reader, 4, "-999")?;
        assert!(values.len() == 4);
        assert!(values[0] == None);
        assert!(values[1] == Some(1.));
        assert!(values[2] == Some(0.3));
        assert!(values[3] == Some(5.));
        Ok(())
    }
}

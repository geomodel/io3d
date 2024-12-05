use anyhow::Result as Result;
use std::io::BufReader;

use crate::utils::line_reader;

//  //  //  //  //  //  //  //
pub(crate) fn read_values<R, T>(reader: &mut BufReader<R>, values_number: usize) -> Result<Vec<T>>
where R: std::io::Read, T: std::str::FromStr
{
    let mut result = Vec::<T>::new();
    result.try_reserve_exact(values_number)?;

    for i in 0..values_number {
        let line = line_reader(reader, &format!("Value #{}", i+1) )?;
        let Ok(value) = line.parse::<T>() else {
            return Err(anyhow::anyhow!("Unable to parse #{} <{}> as BOOL", i, line ));
        };
        result.push(value);
    }
    Ok(result)
}


//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod read_values_of_type {
    use super::*;
    use crate::types::*;

    #[test]
    fn no_values_error() {
        let s = "\n\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let values = read_values::<&[u8], Discrete>(&mut reader, 1);
        assert!(values.is_err(), "must get the error!");
    }
    #[test]
    fn invalid_values_error() {
        let s = "\n2.\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let values = read_values::<&[u8], Discrete>(&mut reader, 1);
        assert!(values.is_err(), "must get the error!");
    }

    #[test]
    fn integer_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1\n0\n5\nunreachable\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let values = read_values::<&[u8], Discrete>(&mut reader, 4)?;
        assert!( values.len() == 4 );
        assert!( values[0] == -999 );
        assert!( values[1] == 1 );
        assert!( values[2] == 0 );
        assert!( values[3] == 5 );
        Ok(())
    }

    #[test]
    fn countinues_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1.0\n0.3\n5\nunreachable\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let values = read_values::<&[u8], Continuous>(&mut reader, 4)?;
        assert!( values.len() == 4 );
        assert!( values[0] == (-999).into() );
        assert!( values[1] == 1.into() );
        assert!( values[2] == 0.3.into() );
        assert!( values[3] == 5.into() );
        Ok(())
    }
}

use anyhow::Result;
use std::io::BufReader;

use crate::utils::line_reader;

//  //  //  //  //  //  //  //
#[allow(dead_code)]
struct IJK {
    i: u16,
    j: u16,
    k: u16,
}

#[allow(dead_code)]
fn read_ijk_values<R, T>(reader: &mut BufReader<R>) -> Result<Vec<(IJK, T)>>
where
    R: std::io::Read,
    T: std::str::FromStr,
{
    let mut result = Vec::new();

    let mut counter: u64 = 1;
    loop {
        let Ok(line) = line_reader(reader, &format!("Value #{}", counter + 1)) else {
            break;
        };
        let parsed_line: Vec<&str> = line.split_ascii_whitespace().collect();
        if parsed_line.len() < 4 {
            return Err(anyhow::anyhow!(
                "Unable to parse <{}> as i-j-k-value #{}",
                line,
                counter + 1
            ));
        };
        let Ok(i) = parsed_line[0].parse::<u16>() else {
            return Err(anyhow::anyhow!(
                "Unable to parse <{}> as I-coordinate #{}",
                parsed_line[0],
                counter + 1
            ));
        };
        let Ok(j) = parsed_line[1].parse::<u16>() else {
            return Err(anyhow::anyhow!(
                "Unable to parse <{}> as J-coordinate #{}",
                parsed_line[1],
                counter + 1
            ));
        };
        let Ok(k) = parsed_line[2].parse::<u16>() else {
            return Err(anyhow::anyhow!(
                "Unable to parse <{}> as K-coordinate #{}",
                parsed_line[2],
                counter + 1
            ));
        };
        let Ok(value) = parsed_line[3].parse::<T>() else {
            return Err(anyhow::anyhow!(
                "Unable to parse <{}> as value #{}",
                parsed_line[3],
                counter + 1
            ));
        };
        result.push((IJK { i, j, k }, value));
        counter += 1;
    }
    Ok(result)
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod read_ijk_values_of_type {
    use super::*;

    #[test]
    fn invalid_value_error() {
        let s = "\n1 2 3 r\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        assert!(
            read_ijk_values::<&[u8], i16>(&mut reader).is_err(),
            "must be Error"
        );
    }
    #[test]
    fn no_value_error() {
        let s = "\n1 2 3\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        assert!(
            read_ijk_values::<&[u8], i16>(&mut reader).is_err(),
            "must be Error"
        );
    }
    #[test]
    fn invalid_ijk_error() {
        let mut s = "\n1. 2 3 5\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        assert!(
            read_ijk_values::<&[u8], i16>(&mut reader).is_err(),
            "must be Error"
        );

        s = "\n1 2. 3 5\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        assert!(
            read_ijk_values::<&[u8], i16>(&mut reader).is_err(),
            "must be Error"
        );

        s = "\n1 2 3. 5\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        assert!(
            read_ijk_values::<&[u8], i16>(&mut reader).is_err(),
            "must be Error"
        );
    }
    #[test]
    fn negative_ijk_error() {
        let mut s = "\n-1 2 3 5\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        assert!(
            read_ijk_values::<&[u8], i16>(&mut reader).is_err(),
            "must be Error"
        );

        s = "\n1 -2 3 5\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        assert!(
            read_ijk_values::<&[u8], i16>(&mut reader).is_err(),
            "must be Error"
        );

        s = "\n1 2 -3 5\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        assert!(
            read_ijk_values::<&[u8], i16>(&mut reader).is_err(),
            "must be Error"
        );
    }

    #[test]
    fn integer_values() -> Result<()> {
        let s = "\n\n\n 1 \t2 \t 3   -999\n\n\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_ijk_values::<&[u8], i16>(&mut reader)?;
        assert!(values.len() == 1);
        assert!(values[0].1 == -999);
        assert!(values[0].0.i == 1);
        assert!(values[0].0.j == 2);
        assert!(values[0].0.k == 3);
        Ok(())
    }

    #[test]
    fn float_values() -> Result<()> {
        let s = "\n\n\n 1 \t2 \t 3   -999.9\n\n2 3 4 5\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let values = read_ijk_values::<&[u8], f64>(&mut reader)?;
        assert!(values.len() == 2);
        assert!(values[0].1 == -999.9_f64);
        assert!(values[0].0.i == 1);
        assert!(values[0].0.j == 2);
        assert!(values[0].0.k == 3);
        assert!(values[1].1 == 5_f64);
        assert!(values[1].0.i == 2);
        assert!(values[1].0.j == 3);
        assert!(values[1].0.k == 4);
        Ok(())
    }
}

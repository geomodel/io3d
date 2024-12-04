use anyhow::Result as Result;

use std::io::{BufReader,BufRead};
use std::fs::File;

/*
fn auto_load_via_filename( file_name: &str ) -> Result<()> {
    let file = File::open(file_name)?;
    auto_load_via_file_descriptor( file )
}
fn auto_load_via_file_descriptor( file: File ) -> Result<()> {
    let mut reader = BufReader::new(file);
    auto_load_via_reader( &mut reader )
}
fn auto_load_via_reader<R: std::io::Read>( reader: &mut BufReader<R> ) -> Result<()> {
    let header_title = get_next_not_empty_string(reader)?;
    let s2 = get_next_not_empty_string(reader)?;
    println!("<{}>\n<{}>", header_title, s2);
    //
    //todo!("autoload");
    Ok(())
}
*/

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


fn read_header_title<R>(reader: &mut BufReader<R>) -> Result<String>
where R: std::io::Read
{
    let header = get_next_not_empty_string(reader)?;
    if header.is_empty() {
        return Err(anyhow::anyhow!("{}","Empty header title"));
    }
    Ok(String::new())
}


fn read_header_values_number<R>(reader: &mut BufReader<R>) -> Result<u8>
where R: std::io::Read
{
    let values_number_text = get_next_not_empty_string(reader)?;
    if values_number_text.is_empty() {
        return Err(anyhow::anyhow!("{}","Empty header values number"));
    }
    let values_number = values_number_text.parse::<u8>()?;
    if values_number == 0 {
        return Err(anyhow::anyhow!("{}","Zero header values number"));
    }
    Ok(values_number)
}

fn read_header_values_description<R>(reader: &mut BufReader<R>, count: u8) -> Result<Vec<String>>
where R: std::io::Read
{
    let mut result = Vec::new();
    for i in 0..count {
        let header_description = get_next_not_empty_string(reader)?;
        if header_description.is_empty() {
            return Err(anyhow::anyhow!("Unable to read header description line #{}", i+1));
        }
        result.push(header_description);
    }
    Ok(result)
}

fn read_bool_values<R>(reader: &mut BufReader<R>, count: u64) -> Result<Vec<bool>>
where R: std::io::Read
{
    let mut result = Vec::new();
    for i in 0..count {
        let value_text = get_next_not_empty_string(reader)?;
        if value_text.is_empty() {
            return Err(anyhow::anyhow!("Unable to read value #{}", i+1));
        }
        match value_text.as_str() {
            "0" => result.push(false),
            "1" => result.push(true),
            _ => return Err(anyhow::anyhow!("Invalid value #{} <{}>", i+1, value_text)),
        }
    }
    Ok(result)
}

fn read_discrete_values<R>(reader: &mut BufReader<R>, count: u64) -> Result<Vec<i16>>
where R: std::io::Read
{
    let mut result = Vec::new();
    for i in 0..count {
        let value_text = get_next_not_empty_string(reader)?;
        if value_text.is_empty() {
            return Err(anyhow::anyhow!("Unable to read value #{}", i+1));
        }
        let value = value_text.parse::<i16>()?;
        result.push(value);
    }
    Ok(result)
}

fn read_continue_values<R>(reader: &mut BufReader<R>, count: u64) -> Result<Vec<f32>>
where R: std::io::Read
{
    let mut result = Vec::new();
    for i in 0..count {
        let value_text = get_next_not_empty_string(reader)?;
        if value_text.is_empty() {
            return Err(anyhow::anyhow!("Unable to read value #{}", i+1));
        }
        let value = value_text.parse::<f32>()?;
        result.push(value);
    }
    Ok(result)
}


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

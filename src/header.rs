#[allow(dead_code)]



use anyhow::Result as Result;
use std::io::BufReader;

use crate::utils::line_reader;

//  //  //  //  //  //  //  //
#[allow(dead_code)]
struct PropertyHeader {
    title: String,
    values_number: u8,
    descriptions: Vec<String>,
}

#[allow(dead_code)]
fn read_header<R>(reader: &mut BufReader<R>) -> Result<PropertyHeader>
where R: std::io::Read
{
    let title = line_reader(reader, "Header Title")?;

    let Ok(values_number) = line_reader(reader, "Header Values Number")?
                                    .parse::<u8>() else {
                                        return Err(anyhow::anyhow!("Invalid format Header Values Number"));
                                    };
    if values_number == 0 {
        return Err(anyhow::anyhow!("Zero Header Values Number"));
    }

    let mut descriptions = Vec::new();
    for i in 0..values_number {
        let description = line_reader(reader, &format!("Values description #{}", i+1) )?;
        descriptions.push(description);
    }

    Ok( PropertyHeader{ title, values_number, descriptions, } )
}


//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod read_header {
    use super::*;

    #[test]
    fn no_title_error() {
        let s = "\n\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let header = read_header(&mut reader);
        assert!(header.is_err(), "must get the error!");
    }
    #[test]
    fn header_title() -> Result<()> {
        let s = "\n\nGeoModel: Property\n2\nfirst\n\n\n3\n\n333\n\n";
        let mut reader = BufReader::new( s.as_bytes() );
        let _header = read_header(&mut reader)?;
        Ok(())
    }
}

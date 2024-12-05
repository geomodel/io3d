use anyhow::Result;
use std::io::BufWriter;
use std::io::Write;

//  //  //  //  //  //  //  //
pub fn write_property<R, T>(writer: &mut BufWriter<R>, data: &[T]) -> Result<()>
where
    R: std::io::Write,
    T: std::fmt::Display,
{
    writeln!(writer, "GeoModel: Property")?;
    writeln!(writer, "1")?;
    writeln!(writer, "Value")?;
    for value in data {
        writeln!(writer, "{}", value)?;
    }
    Ok(())
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod read_values_of_type {
    use super::*;
    use crate::types::*;

    #[test]
    fn discrete_values() -> Result<()> {
        let comp = "GeoModel: Property\n1\nValue\n1\n2\n3\n";
        let mut buf = Vec::new();
        let mut writer = BufWriter::new(&mut buf);
        let property: Vec<Discrete> = vec![1, 2, 3];
        write_property(&mut writer, &property)?;
        drop(writer);
        let s = String::from_utf8(buf)?;
        assert!(s == comp);
        Ok(())
    }

    #[test]
    fn continue_values() -> Result<()> {
        let comp = "GeoModel: Property\n1\nValue\n1\n2.2\n0.3\n";
        let mut buf = Vec::new();
        let mut writer = BufWriter::new(&mut buf);
        let arr: [Continuous; 3] = [1.0, 2.2, 0.3];
        write_property(&mut writer, &arr)?;
        drop(writer);
        let s = String::from_utf8(buf)?;
        assert!(s == comp);
        Ok(())
    }
}

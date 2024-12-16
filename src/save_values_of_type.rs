use anyhow::Result;
use std::io::BufWriter;
use std::io::Write;

//  //  //  //  //  //  //  //
pub fn write_property<R, T>(
    writer: &mut BufWriter<R>,
    data: &[Option<T>],
    undef_value: &str,
) -> Result<()>
where
    R: std::io::Write,
    T: std::fmt::Display,
{
    writeln!(writer, "GeoModel: Property")?;
    writeln!(writer, "1")?;
    writeln!(writer, "Value")?;
    for value in data {
        match value {
            None => {
                writeln!(writer, "{}", undef_value)?;
            }
            Some(v) => {
                writeln!(writer, "{}", v)?;
            }
        }
    }
    Ok(())
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
    fn discrete_values() -> Result<()> {
        let comp = "GeoModel: Property\n1\nValue\n1\n2\n3\n";
        let mut buf = Vec::new();
        let mut writer = BufWriter::new(&mut buf);
        let property: Vec<Option<Discrete>> = vec![Some(1), Some(2), Some(3)];
        write_property(&mut writer, &property, "-999")?;
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
        let arr: [Option<Continuous>; 3] = [Some(1.0), Some(2.2), Some(0.3)];
        write_property(&mut writer, &arr, "-999")?;
        drop(writer);
        let s = String::from_utf8(buf)?;
        assert!(s == comp);
        Ok(())
    }
}

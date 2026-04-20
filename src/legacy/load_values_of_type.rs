use std::io::BufReader;

use super::utils;
use crate::Result;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub(crate) trait OutputContainer<T> {
    fn new_with_capacity(size: usize) -> Self;
    fn push(&mut self, value: T);
    fn push_undef(&mut self);
}

//  //  //  //  //  //  //  //
pub(crate) struct AsValuesAndDefFlag<T> {
    pub(crate) main_array: Vec<T>,
    pub(crate) flag_array: Vec<bool>,
}
impl<T: std::convert::From<i16>> OutputContainer<T> for AsValuesAndDefFlag<T> {
    fn new_with_capacity(size: usize) -> Self {
        Self {
            main_array: Vec::<T>::with_capacity(size),
            flag_array: Vec::<bool>::with_capacity(size),
        }
    }
    fn push(&mut self, value: T) {
        self.main_array.push(value);
        self.flag_array.push(true);
    }
    fn push_undef(&mut self) {
        self.main_array.push(0.into());
        self.flag_array.push(false);
    }
}

pub(crate) struct AsOptionValues<T> {
    pub(crate) array: Vec<Option<T>>,
}
impl<T> OutputContainer<T> for AsOptionValues<T> {
    fn new_with_capacity(size: usize) -> Self {
        Self {
            array: Vec::<Option<T>>::with_capacity(size),
        }
    }
    fn push(&mut self, value: T) {
        self.array.push(Some(value));
    }
    fn push_undef(&mut self) {
        self.array.push(None);
    }
}

pub(crate) struct AsRawValues<T> {
    pub(crate) array: Vec<T>,
    pub(crate) undef: T,
}
impl<T: std::convert::From<i16>+Clone> OutputContainer<T> for AsRawValues<T> {
    fn new_with_capacity(size: usize) -> Self {
        Self {
            array: Vec::<T>::with_capacity(size),
            undef: T::from(-999),
        }
    }
    fn push(&mut self, value: T) {
        self.array.push(value);
    }
    fn push_undef(&mut self) {
        self.array.push(self.undef.clone());
    }
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub(crate) fn read_values<R, T, O>(
    reader: &mut BufReader<R>,
    size: usize,
    undef_value: &str,
) -> Result<O>
where
    R: std::io::Read,
    T: std::str::FromStr,
    O: OutputContainer<T>,
{
    let mut result = O::new_with_capacity(size);

    for i in 0..size {
        let line = utils::line_reader(reader, &format!("Value #{}", i + 1))?;
        if line == undef_value {
            result.push_undef();
        } else {
            let Ok(value) = line.parse::<T>() else {
                return Err(format!("Unable to parse #{} <{}> as value", i, line).into());
            };
            result.push(value);
        }
    }
    Ok(result)
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod with_defflag_values {
    use super::*;
    type Continuous = f64;
    type Discrete = i16;
    type TestedType<T> = AsValuesAndDefFlag<T>;

    #[test]
    fn no_values_error() {
        let s = "\n\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container =
            read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 1, "-999");
        assert!(container.is_err(), "must get the error!");
    }
    #[test]
    fn invalid_values_error() {
        let s = "\n2.\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container = read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 1, "-999");
        assert!(container.is_err(), "must get the error!");
    }

    #[test]
    fn integer_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1\n0\n5\nunreachable\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container = read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 4, "-999")?;
        let main_values = container.main_array;
        let flag_values = container.flag_array;
        assert!(main_values.len() == 4);
        assert!(flag_values.len() == 4);
        assert!(main_values[0] == 0);
        assert!(flag_values[0] == false);
        assert!(main_values[1] == 1);
        assert!(flag_values[1] == true);
        assert!(main_values[2] == 0);
        assert!(flag_values[2] == true);
        assert!(main_values[3] == 5);
        assert!(flag_values[3] == true);
        Ok(())
    }

    #[test]
    fn countinues_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1.0\n0.3\n5\nunreachable\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container = read_values::<&[u8], Continuous, TestedType<Continuous>>(&mut reader, 4, "-999")?;
        let main_values = container.main_array;
        let flag_values = container.flag_array;
        assert!(main_values.len() == 4);
        assert!(flag_values.len() == 4);
        assert!(main_values[0] == 0.);
        assert!(flag_values[0] == false);
        assert!(main_values[1] == 1.);
        assert!(flag_values[1] == true);
        assert!(main_values[2] == 0.3);
        assert!(flag_values[2] == true);
        assert!(main_values[3] == 5.);
        assert!(flag_values[3] == true);
        Ok(())
    }
}

#[cfg(test)]
mod option_values {
    use super::*;
    type Continuous = f64;
    type Discrete = i16;
    type TestedType<T> = AsOptionValues<T>;

    #[test]
    fn no_values_error() {
        let s = "\n\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container =
            read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 1, "-999");
        assert!(container.is_err(), "must get the error!");
    }
    #[test]
    fn invalid_values_error() {
        let s = "\n2.\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container = read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 1, "-999");
        assert!(container.is_err(), "must get the error!");
    }

    #[test]
    fn integer_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1\n0\n5\nunreachable\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container = read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 4, "-999")?;
        let values = container.array;
        assert!(values.len() == 4);
        assert!(values[0] == None);
        assert!(values[1] == Some(1));
        assert!(values[2] == Some(0));
        assert!(values[3] == Some(5));
        Ok(())
    }

    #[test]
    fn countinues_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1.0\n0.3\n5\nunreachable\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container = read_values::<&[u8], Continuous, TestedType<Continuous>>(&mut reader, 4, "-999")?;
        let values = container.array;
        assert!(values.len() == 4);
        assert!(values[0] == None);
        assert!(values[1] == Some(1.));
        assert!(values[2] == Some(0.3));
        assert!(values[3] == Some(5.));
        Ok(())
    }
}

#[cfg(test)]
mod raw_values {
    use super::*;
    type Continuous = f64;
    type Discrete = i16;
    type TestedType<T> = AsRawValues<T>;

    #[test]
    fn no_values_error() {
        let s = "\n\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container =
            read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 1, "-999");
        assert!(container.is_err(), "must get the error!");
    }
    #[test]
    fn invalid_values_error() {
        let s = "\n2.\n\n";
        let mut reader = BufReader::new(s.as_bytes());
        let container = read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 1, "-999");
        assert!(container.is_err(), "must get the error!");
    }

    #[test]
    fn integer_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1\n0\n5\nunreachable\n";
        let mut reader = BufReader::new(s.as_bytes());
        let mut container = read_values::<&[u8], Discrete, TestedType<Discrete>>(&mut reader, 4, "-999")?;
        container.undef = -999;
        let values = container.array;
        assert!(values.len() == 4);
        assert!(values[0] == container.undef);
        assert!(values[1] == 1);
        assert!(values[2] == 0);
        assert!(values[3] == 5);
        Ok(())
    }

    #[test]
    fn countinues_values() -> Result<()> {
        let s = "\n\n\n-999\n\n\n\n1.0\n0.3\n5\nunreachable\n";
        let mut reader = BufReader::new(s.as_bytes());
        let mut container = read_values::<&[u8], Continuous, TestedType<Continuous>>(&mut reader, 4, "-999")?;
        container.undef = -999.0;
        let values = container.array;
        assert!(values.len() == 4);
        assert!(values[0] == container.undef);
        assert!(values[1] == 1.);
        assert!(values[2] == 0.3);
        assert!(values[3] == 5.);
        Ok(())
    }
}

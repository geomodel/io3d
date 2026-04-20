use std::{
    fs::File,
    io::{
        Read,
        BufReader,
    },
};

use crate::error::Result;

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub struct GSData<R> {
    pub reader: BufReader<R>,
}
impl<R: Read> GSData<R> {
    pub fn from_reader(reader: BufReader<R>) -> Result<GSData<R>> {
        Ok(
            GSData {
                reader,
            }
        )
    }
    pub fn from_filename(file_name: &str) -> Result<GSData<File>> {
        let f = File::open(file_name)?;
        let r = BufReader::new(f);
        GSData::from_reader(r)
    }
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod basic {
    use super::*;
    use crate::Result;

    #[test]
    fn creating_from_buffer() ->Result<()> {
        let s = "";
        let reader = BufReader::new(s.as_bytes());
        let pr = GSData::from_reader(reader);
        assert!(pr.is_ok(), "must be Ok!");
        Ok(())
    }

    #[test]
    fn creating_from_file() ->Result<()> {
        let pr = GSData::<File>::from_filename("/n");
        assert!(pr.is_err(), "must be error!");
        Ok(())
    }
}

mod types;
mod utils;

mod load_header;

mod load_values_bool;
mod load_values_of_type;
mod load_ijk_values_of_type;

mod save_values_of_type;

//  //  //  //  //  //  //  //
use anyhow::Result;
use std::io::BufReader;
use std::{fs::File, io::BufWriter};

use types::*;

pub fn save_discrete_property(file_name: &str, property: &[Discrete]) -> Result<()> {
    let fl = File::create(file_name)?;
    let mut writer = BufWriter::new(fl);
    save_values_of_type::write_property(&mut writer, property)?;
    Ok(())
}
pub fn save_continuous_property(file_name: &str, property: &[Continuous]) -> Result<()> {
    let fl = File::create(file_name)?;
    let mut writer = BufWriter::new(fl);
    save_values_of_type::write_property(&mut writer, property)?;
    Ok(())
}

pub fn load_actnum(file_name: &str, size: usize) -> Result<Vec<bool>> {
    let fl = File::open(file_name)?;
    let mut reader = BufReader::new(fl);
    let header = load_header::read_header(&mut reader)?;
    if header.values_number != 1 {
        return Err(anyhow::anyhow!(
            "Actnum property file must contains the only value"
        ));
    }
    load_values_bool::read_bool(&mut reader, size)
}
pub fn load_discrete_property(file_name: &str, size: usize) -> Result<Vec<Discrete>> {
    let fl = File::open(file_name)?;
    let mut reader = BufReader::new(fl);
    let header = load_header::read_header(&mut reader)?;
    if header.values_number != 1 {
        return Err(anyhow::anyhow!(
            "Discrete property file must contains the only value"
        ));
    }
    load_values_of_type::read_values(&mut reader, size)
}
pub fn load_continuous_property(file_name: &str, size: usize) -> Result<Vec<Continuous>> {
    let fl = File::open(file_name)?;
    let mut reader = BufReader::new(fl);
    let header = load_header::read_header(&mut reader)?;
    if header.values_number != 1 {
        return Err(anyhow::anyhow!(
            "Continuous property file must contains the only value"
        ));
    }
    load_values_of_type::read_values(&mut reader, size)
}

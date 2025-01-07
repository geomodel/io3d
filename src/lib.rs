mod utils;

mod load_header;

mod load_ijk_values_of_type;
mod load_values_bool;
mod load_values_of_type;

mod save_values_of_type;

//  //  //  //  //  //  //  //
use anyhow::Result;
use std::io::BufReader;
use std::{fs::File, io::BufWriter};

use data_types::*;


pub fn save_property<T>(file_name: &str, property: &[Option<T>], undef_value: &str) -> Result<()>
where
    T: std::fmt::Display,
{
    let fl = File::create(file_name)?;
    let mut writer = BufWriter::new(fl);
    save_values_of_type::write_property(&mut writer, property, undef_value)?;
    Ok(())
}

pub fn load_property<T>(file_name: &str, size: usize, undef_value: &str) -> Result<Box<[Option<T>]>>
where
    T: std::str::FromStr,
{
    let fl = File::open(file_name)?;
    let mut reader = BufReader::new(fl);
    let header = load_header::read_header(&mut reader)?;
    if header.values_number != 1 {
        return Err(anyhow::anyhow!(
            "Discrete property file must contains the only value"
        ));
    }
    load_values_of_type::read_values(&mut reader, size, undef_value)
}

pub fn load_actnum(file_name: &str, size: usize) -> Result<Box<[bool]>> {
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

pub fn load_bw<T>(file_name: &str) -> Result<Box<[(IJK, T)]>>
where
    T: std::str::FromStr,
{
    let fl = File::open(file_name)?;
    let mut reader = BufReader::new(fl);
    let header = load_header::read_header(&mut reader)?;
    if header.values_number != 4 {
        return Err(anyhow::anyhow!(
            "Upscaled file must contains I, J, K, Value"
        ));
    }
    load_ijk_values_of_type::read_ijk_values(&mut reader)
}

use types3d;
use types3d::*;

use crate::error::Result;
use super::{
    utils,
    load_values_bool,
    load_values_of_type,
    load_ijk_values_of_type,
    save_values_of_type,
};



//  //  //  //  //  //  //  //
pub fn save_property<T>(file_name: &str, property: &[Option<T>], undef_value: &str) -> Result<()>
where
    T: std::fmt::Display,
{
    let mut writer = utils::prepare_saving(file_name)?;
    save_values_of_type::write_property(&mut writer, property, undef_value)?;
    Ok(())
}

pub fn load_property<T>(file_name: &str, size: usize, undef_value: &str) -> Result<Box<[Option<T>]>>
where
    T: std::str::FromStr,
{
    let (mut reader, header) = utils::prepare_loading(file_name)?;
    if header.values_number != 1 {
        return Err("Property file must contains the only value".into());
    }
    let container: load_values_of_type::AsOptionValues<T> =
        load_values_of_type::read_values(&mut reader, size, undef_value)?;
    Ok(container.array.into_boxed_slice())
}

pub fn load_actnum(file_name: &str, size: usize) -> Result<Box<[bool]>> {
    let (mut reader, header) = utils::prepare_loading(file_name)?;
    if header.values_number != 1 {
        return Err("Actnum property file must contains the only value".into());
    }
    Ok(load_values_bool::read_bool(&mut reader, size)?)
}

pub fn load_bw<T>(file_name: &str) -> Result<Box<[(IJK, T)]>>
where
    T: std::str::FromStr,
{
    let (mut reader, header) = utils::prepare_loading(file_name)?;
    if header.values_number != 4 {
        return Err("Upscaled file must contains I, J, K, Value".into());
    }
    Ok(load_ijk_values_of_type::read_ijk_values(&mut reader)?)
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
const UNDEF_VALUE: i16 = -999;
const UNDEF_VALUE_STR: &str = "-999";

pub fn save_raw_property<T>(file_name: &str, property: &[T]) -> Result<()>
where
    T: std::fmt::Display,
{
    let mut writer = utils::prepare_saving(file_name)?;
    save_values_of_type::write_raw_property(&mut writer, property)?;
    Ok(())
}

pub fn load_raw_property<T: std::convert::From<i16>+Clone>(file_name: &str, size: usize) -> Result<Box<[T]>>
where
    T: std::str::FromStr,
{
    let (mut reader, header) = utils::prepare_loading(file_name)?;
    if header.values_number != 1 {
        return Err("Discrete property file must contains the only value".into());
    }
    let mut container: load_values_of_type::AsRawValues<T> =
        load_values_of_type::read_values(&mut reader, size, UNDEF_VALUE_STR)?;
    container.undef = UNDEF_VALUE.try_into()?;
    Ok(container.array.into_boxed_slice())
}

//  //  //  //  //  //  //  //
pub fn load_property_with_def_flag<T: std::convert::From<i16>+Clone>(file_name: &str, size: usize) -> Result<(Box<[T]>, Box<[bool]>)>
where
    T: std::str::FromStr,
{
    let (mut reader, header) = utils::prepare_loading(file_name)?;
    if header.values_number != 1 {
        return Err("Discrete property file must contains the only value".into());
    }
    let container: load_values_of_type::AsValuesAndDefFlag<T> =
        load_values_of_type::read_values(&mut reader, size, UNDEF_VALUE_STR)?;
    let main_array = container.main_array.into_boxed_slice();
    let flag_array = container.flag_array.into_boxed_slice();
    Ok((main_array, flag_array))
}

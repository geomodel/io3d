mod error;
pub use error::{Error, Result};
pub use types3d;

//  //  //  //  //  //  //  //
mod legacy;
pub use legacy::api as legacy_api;
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
mod line_reader;

//mod gs_data;
//use gs_data::*;

mod gs_header;
pub use gs_header::GSHeader;

mod gs_property;
pub use gs_property::GSProperty;

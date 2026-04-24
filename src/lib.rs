mod error;
pub use error::{Error, Result};
pub use types3d;

//  //  //  //  //  //  //  //
mod legacy;
pub use legacy::api as legacy_api;
//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
mod line_reader;

//  //  //  //  //  //
//      LOADING     //
mod gs_r_header;
pub use gs_r_header::GSHeader;

mod gs_r_property;
pub use gs_r_property::GSProperty;

//  //  //  //  //  //
//      SAVING      //
//mod gs_w_header;

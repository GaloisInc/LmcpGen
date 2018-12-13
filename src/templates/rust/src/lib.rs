// ===============================================================================
// Authors: AFRL/RQQA
// Organization: Air Force Research Laboratory, Aerospace Systems Directorate, Power and Control Division
// 
// Copyright (c) 2017 Government of the United State of America, as represented by
// the Secretary of the Air Force.  No copyright is claimed in the United States under
// Title 17, U.S. Code.  All Other Rights Reserved.
// ===============================================================================

// This file was auto-created by LmcpGen. Modifications will be overwritten.
#![allow(non_camel_case_types)]

#[macro_use]
#[cfg(test)]
extern crate quickcheck;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
mod avtas;
-<declare_top_level_modules>-

//use avtas::lmcp::{Lmcp, StructInfo};
//pub use avtas::lmcp::{Error, ErrorType, LmcpSubscription, SrcLoc};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    -<declare_top_enum>-
}
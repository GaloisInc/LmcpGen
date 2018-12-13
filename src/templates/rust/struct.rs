// ===============================================================================
// Authors: AFRL/RQQA
// Organization: Air Force Research Laboratory, Aerospace Systems Directorate, Power and Control Division
// 
// Copyright (c) 2017 Government of the United State of America, as represented by
// the Secretary of the Air Force.  No copyright is claimed in the United States under
// Title 17, U.S. Code.  All Other Rights Reserved.
// ===============================================================================

// This file was auto-created by LmcpGen. Modifications will be overwritten.

//use avtas::lmcp::{Error, ErrorType, Lmcp, LmcpSubscription, SrcLoc, Struct, StructInfo};
use std::fmt::Debug;

#[derive(Clone, -<struct_copy>-Debug, Default, Serialize, Deserialize)]
#[repr(C)]
pub struct -<datatype_name>- {-<declare_fields>-
}

impl PartialEq for -<datatype_name>- {
    fn eq(&self, _other: &-<datatype_name>-) -> bool {
        true
        -<struct_partialeq_cases>-
    }
}

pub trait -<datatype_name>-T: Debug + Send -<declare_parent_trait>- {
    -<declare_trait_methods>-
}

impl PartialEq for Box<-<datatype_name>-T> {
    fn eq(&self, other: &Box<-<datatype_name>-T>) -> bool {
        if let (Some(x), Some(y)) =
            (-<datatype_name>-T::as_-<series_snake_name>-_-<datatype_snake_name>-(self.as_ref()),
             -<datatype_name>-T::as_-<series_snake_name>-_-<datatype_snake_name>-(other.as_ref())) {
                x == y
        -<trait_partialeq_cases>-
        } else {
            false
        }
    }
}

-<declare_trait_impls>-


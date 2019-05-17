// This file is generated by lavish: DO NOT EDIT
// https://github.com/fasterthanlime/lavish

// Disable some lints, since this file is generated.
#![allow(clippy::all)]
#![allow(unknown_lints)]

use lavish_rpc::erased_serde;
use lavish_rpc::serde_derive::*;

pub type Message = lavish_rpc::Message<Params, NotificationParams, Results>;

#[derive(Serialize, Debug)]
#[serde(untagged)]
#[allow(non_camel_case_types, unused)]
pub enum Params {
    double_Print(double::print::Params),
    double_Double(double::double::Params),
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
#[allow(non_camel_case_types, unused)]
pub enum Results {
    double_Print(double::print::Results),
    double_Double(double::double::Results),
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
#[allow(non_camel_case_types, unused)]
pub enum NotificationParams {
    double_Log(double::log::Params),
}

impl lavish_rpc::Atom for Params {
    fn method(&self) -> &'static str {
        match self {
            Params::double_Print(_) => "double.Print",
            Params::double_Double(_) => "double.Double",
        }
    }

    fn deserialize(
        method: &str,
        de: &mut erased_serde::Deserializer,
    ) -> erased_serde::Result<Self> {
        use erased_serde::deserialize as deser;
        use serde::de::Error;

        match method {
            "double.Print" => Ok(Params::double_Print(deser::<double::print::Params>(de)?)),
            "double.Double" => Ok(Params::double_Double(deser::<double::double::Params>(de)?)),
            _ => Err(erased_serde::Error::custom(format!(
                "unknown method: {}",
                method,
            ))),
        }
    }
}

impl lavish_rpc::Atom for Results {
    fn method(&self) -> &'static str {
        match self {
            Results::double_Print(_) => "double.Print",
            Results::double_Double(_) => "double.Double",
        }
    }

    fn deserialize(
        method: &str,
        de: &mut erased_serde::Deserializer,
    ) -> erased_serde::Result<Self> {
        use erased_serde::deserialize as deser;
        use serde::de::Error;

        match method {
            "double.Print" => Ok(Results::double_Print(deser::<double::print::Results>(de)?)),
            "double.Double" => Ok(Results::double_Double(deser::<double::double::Results>(
                de,
            )?)),
            _ => Err(erased_serde::Error::custom(format!(
                "unknown method: {}",
                method,
            ))),
        }
    }
}

impl lavish_rpc::Atom for NotificationParams {
    fn method(&self) -> &'static str {
        match self {
            NotificationParams::double_Log(_) => "double.Log",
        }
    }

    fn deserialize(
        method: &str,
        de: &mut erased_serde::Deserializer,
    ) -> erased_serde::Result<Self> {
        use erased_serde::deserialize as deser;
        use serde::de::Error;

        match method {
            "double.Log" => Ok(NotificationParams::double_Log(
                deser::<double::log::Params>(de)?,
            )),
            _ => Err(erased_serde::Error::custom(format!(
                "unknown method: {}",
                method,
            ))),
        }
    }
}

pub mod double {
    pub mod print {
        use lavish_rpc::serde_derive::*;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Params {
            pub s: String,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Results {}
    }

    pub mod double {
        use lavish_rpc::serde_derive::*;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Params {
            pub x: i64,
        }

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Results {
            pub x: i64,
        }
    }

    pub mod log {
        use lavish_rpc::serde_derive::*;

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Params {
            pub msg: String,
        }
    }

}

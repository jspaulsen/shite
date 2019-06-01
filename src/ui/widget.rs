use super::{
    Interfaceable,
    Ui,
};
use std::fmt::Debug;

pub trait Widget: Ui + Interfaceable + Debug {}

use std::sync::mpsc::Sender;

use lib_backend_common::TimeStatus;

#[derive(Debug)]
pub enum TimeQuery {
    GetCurrent { tx: Sender<TimeStatus> }
}
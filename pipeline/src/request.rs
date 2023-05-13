use crate::time::Time;

#[derive(Clone, Copy, Default, Debug)]
pub struct Request {
    pub arrival_time: Time,
    pub leaving_time: Time,
}

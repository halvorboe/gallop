use mockall::predicate::*;
use mockall::*;

#[automock]
pub trait PackerCaller {
    fn new() -> Self;
}

#[derive(Clone)]
pub struct ConnectedPackerCaller {}

impl PackerCaller for ConnectedPackerCaller {
    fn new() -> Self {
        Self {}
    }
}

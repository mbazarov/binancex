use serde_repr::{Deserialize_repr, Serialize_repr};

#[rustfmt::skip]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq,
    Deserialize_repr, Serialize_repr,
)]
#[repr(u16)]
pub enum DepthLimit {
    Limit5    = 5,
    Limit10   = 10,
    Limit20   = 20,
    Limit50   = 50,
    Limit100  = 100,
    Limit500  = 500,
    Limit1000 = 1000,
}

impl Default for DepthLimit {
    fn default() -> Self {
        DepthLimit::Limit500
    }
}

impl DepthLimit {
    pub fn request_weight(&self) -> RequestWeight {
        RequestWeight::from(*self)
    }
}

#[rustfmt::skip]
impl From<DepthLimit> for u16 {
    fn from(limit: DepthLimit) -> Self {
        limit as u16
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestWeight {
    Weight2,
    Weight5,
    Weight10,
    Weight20,
}

#[rustfmt::skip]
impl From<RequestWeight> for u8 {
    fn from(weight: RequestWeight) -> Self {
        match weight {
            RequestWeight::Weight2  => 2,
            RequestWeight::Weight5  => 5,
            RequestWeight::Weight10 => 10,
            RequestWeight::Weight20 => 20,
        }
    }
}

#[rustfmt::skip]
impl From<DepthLimit> for RequestWeight {
    fn from(limit: DepthLimit) -> Self {
        match limit {
            DepthLimit::Limit5    => RequestWeight::Weight2,
            DepthLimit::Limit10   => RequestWeight::Weight2,
            DepthLimit::Limit20   => RequestWeight::Weight2,
            DepthLimit::Limit50   => RequestWeight::Weight2,
            DepthLimit::Limit100  => RequestWeight::Weight5,
            DepthLimit::Limit500  => RequestWeight::Weight10,
            DepthLimit::Limit1000 => RequestWeight::Weight20,
        }
    }
}

#[rustfmt::skip]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq,
    Deserialize_repr, Serialize_repr,
)]
#[repr(u16)]
pub enum DepthUpdateSpeedMs {
    Speed100 = 100,
    Speed250 = 250,
    Speed500 = 500,
}

impl From<DepthUpdateSpeedMs> for u16 {
    fn from(update_speed: DepthUpdateSpeedMs) -> Self {
        update_speed as u16
    }
}

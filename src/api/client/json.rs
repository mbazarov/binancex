use serde::{
    de::Deserialize,
    // ser::Serialize,
};

// #[cfg(all(feature = "serde_json", not(feature = "simd_json")))]
// #[inline(always)]
// pub fn to_vec<S: Serialize + ?Sized>(json: &S) -> Result<Vec<u8>, serde_json::Error> {
//     serde_json::to_vec(json)
// }
//
// #[cfg(all(feature = "simd_json", not(feature = "serde_json")))]
// #[inline(always)]
// pub fn to_vec<S: Serialize + ?Sized>(json: &S) -> Result<Vec<u8>, simd_json::Error> {
//     simd_json::to_vec(json)
// }

#[cfg(all(feature = "serde_json", not(feature = "simd_json")))]
#[inline(always)]
pub async fn from_slice<D>(s: &[u8]) -> Result<D, serde_json::Error>
where
    D: for<'de> Deserialize<'de>,
{
    serde_json::from_slice(&s)
}

#[cfg(all(feature = "simd_json", not(feature = "serde_json")))]
#[inline]
pub async fn from_slice<D>(s: &[u8]) -> Result<D, simd_json::Error>
where
    D: for<'de> Deserialize<'de>,
{
    let len = s.len();
    unsafe {
        let s_ptr = s.as_ptr() as *mut u8;
        let mut json_slice = std::slice::from_raw_parts_mut(s_ptr, len);

        simd_json::from_slice(&mut json_slice)
    }
}

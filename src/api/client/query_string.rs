use serde::ser::Serialize;

#[inline(always)]
pub fn to_query_string<S: Serialize>(params: &S) -> Result<String, serde_qs::Error> {
    serde_qs::to_string(params)
}

#[inline(always)]
pub fn to_query_string_writer<S: Serialize>(
    params: &S,
    buf: &mut Vec<u8>,
) -> Result<(), serde_qs::Error> {
    serde_qs::to_writer(params, buf)
}

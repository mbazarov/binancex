pub fn gen_signature(data: &[u8], secret_key: &ring::hmac::Key) -> String {
    hex::encode(ring::hmac::sign(secret_key, data).as_ref())
}

pub fn gen_signature_to_slice(
    input: &[u8],
    output: &mut [u8],
    secret_key: &ring::hmac::Key,
) -> Result<(), hex::FromHexError> {
    hex::encode_to_slice(ring::hmac::sign(secret_key, input).as_ref(), output)
}

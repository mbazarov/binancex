
pub fn vec_strings_to_string(list: &[String]) -> String {
    let mut buf = String::with_capacity(list.len() * 12);
    buf.push_str("[");
    for symbol in list.iter() {
        buf.push('"');
        buf.push_str(symbol);
        buf.push('"');
        buf.push_str(",");
    }
    buf.pop();
    buf.push_str("]");

    buf
}

// Fetch the unicode string slice based on the start..=end index
// Refer: https://docs.rs/utf8_slice/latest/src/utf8_slice/lib.rs.html#52-70
pub fn fetch_string_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    s.char_indices()
        .nth(start)
        .and_then(|(start_pos, _)| {
            if end >= s.len() {
                return None;
            }

            return s[start_pos..]
                  .char_indices()
                  .nth(end - start)
                  .map(|(end_pos, _)| &s[start_pos..=start_pos + end_pos]);
        })
}

pub(super) trait AppendAndHash {
    fn append_and_hash(self, append_with: u32) -> String;
}

impl AppendAndHash for &str {
    fn append_and_hash(self, append_with: u32) -> String {
        format!(
            "{:x}",
            md5::compute(format!("{}{}", self, append_with).as_bytes())
        )
    }
}

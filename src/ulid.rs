/// Create a ULID with the time set to the current time
pub fn ulid() -> ulid::Ulid {
    ulid::Ulid::new()
}

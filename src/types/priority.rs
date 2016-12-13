/// Represents file's priority
#[derive(Clone, Debug)]
pub enum Priority {
    Low,
    Normal,
    High
}

impl_enum_serde! {
    Priority {
        -1 => Priority::Low,
        0 => Priority::Normal,
        1 => Priority::High
    }
}

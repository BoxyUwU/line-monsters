pub struct Context {
    pub texture_ids: u64,
}

impl Context {
    pub fn new() -> Self {
        Self { texture_ids: 0 }
    }
}

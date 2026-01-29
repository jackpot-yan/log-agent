pub struct Net {
    pub url: String,
}

impl Net {
    pub fn new(url: String) -> Self {
        Net { url: url }
    }
}

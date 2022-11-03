pub struct RAM {
    data: [u8; 4096],
}

impl RAM {
    pub fn set(&self, address: usize, data: &[u8]) {}
    pub fn get(&self, address: usize, bytes: u32) -> &[u8] {
        &[]
    }
}

impl Default for RAM {
    fn default() -> Self {
        RAM { data: [0; 4096] }
    }
}

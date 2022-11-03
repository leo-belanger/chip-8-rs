const RAM_SIZE_IN_BYTES: usize = 4096;

pub struct RAM {
    data: [u8; RAM_SIZE_IN_BYTES],
}

impl RAM {
    pub fn write(&mut self, address: usize, data: &[u8]) -> Result<usize, String> {
        let number_of_bytes = data.len();

        if address + number_of_bytes >= RAM_SIZE_IN_BYTES {
            return Err(format!(
                "Trying to write {} bytes at address {} would exceed bounds({} bytes) of the RAM.",
                number_of_bytes, address, RAM_SIZE_IN_BYTES
            ));
        }

        for (byte_index, byte) in data.iter().enumerate() {
            self.data[address + byte_index] = byte.clone();
        }

        Ok(number_of_bytes)
    }
    pub fn read(&self, address: usize, bytes_to_read: usize) -> Result<&[u8], String> {
        if address + bytes_to_read >= RAM_SIZE_IN_BYTES {
            return Err(format!(
                "Trying to read {} bytes at address {} would exceed bounds({} bytes) of the RAM.",
                bytes_to_read, address, RAM_SIZE_IN_BYTES
            ));
        }

        Ok(&self.data[address..address + bytes_to_read + 1])
    }
}

impl Default for RAM {
    fn default() -> Self {
        RAM { data: [0; 4096] }
    }
}

use rand::RngCore;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct UUID([u8; 16]);

impl UUID {
    fn generate_random_bytes() -> [u8; 10] {
        let mut buf = [0u8; 10];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut buf);
        return buf;
    }

    pub fn format_to_string(&self) -> String {
        return self
            .0
            .into_iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();
    }

    pub fn generate_v7() -> Self {
        let mut buf = [0u8; 16];

        // Generate current unix millisecond timestamp
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        let tsbuf: [u8; 8] = (ts << 16).to_be_bytes();

        // Set the first 6 bytes to a 48-bit big-endian timestamp
        buf[0..8].copy_from_slice(&tsbuf);

        // Copy 10 bytes of random data into the buffer
        let rand_data = Self::generate_random_bytes();
        buf[6..].copy_from_slice(&rand_data);

        // Clear the first 4 bits of 7th byte
        buf[6] &= 0xF;
        // Set the first 4 bits to UUID version marker
        buf[6] |= 7 << 4;

        // Zero the first 2 bits of 9th byte
        buf[8] &= 0x3F;
        // Set the first 2 bits to RFC variant 2
        buf[8] |= 2 << 6;

        Self(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_v7() {
        let result = UUID::generate_v7();
        let byte = result.0[6];
        assert_eq!(byte & 0xF0, 7 << 4);
    }

    #[test]
    fn test_format() {
        let result = UUID::generate_v7().format_to_string();
        assert_eq!(result.len(), 32);
    }
}

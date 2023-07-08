use std::time::{Duration, SystemTime, UNIX_EPOCH};

trait UUIDTimestamp {
    fn to_uuidv7_format(&self) -> [u8; 8];
}

impl UUIDTimestamp for SystemTime {
    fn to_uuidv7_format(&self) -> [u8; 8] {
        let ts = self
            .duration_since(UNIX_EPOCH)
            .expect("This function is only defined for timestamps before unix epoch")
            .as_secs() as u64;

        let buffer: [u8; 8] = (ts << 16).to_be_bytes();
        return buffer;
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converting_timestamps() {
        let ts = UNIX_EPOCH;
        let actual = ts.to_uuidv7_format();
        assert_eq!(actual, [0, 0, 0, 0, 0, 0, 0, 0]);

        let ts = UNIX_EPOCH + Duration::from_millis(1688812097280);
        let actual = ts.to_uuidv7_format();
        assert_eq!(actual, [1, 137, 53, 11, 143, 0, 0, 0]);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

const WIDTH: u16 = 256;
const HEIGHT: u16 = 240;
const SIZE: usize = (WIDTH * HEIGHT) as usize;

pub struct Frame {
    pub bytes: [u8; SIZE],
}

impl Frame {
    pub const WIDTH: u16 = WIDTH;

    pub fn new() -> Self {
        Frame { bytes: [0; SIZE] }
    }
}

#[cfg(test)]

mod test_frame {
    use super::*;

    #[test]
    fn test_frame_bytes_are_initialized_to_zero() {
        let frame = Frame::new();
        assert_eq!([0; SIZE], frame.bytes);
    }

    #[test]
    fn test_frame_bytes_size() {
        let frame = Frame::new();
        assert_eq!(256 * 240, frame.bytes.len());
    }
}

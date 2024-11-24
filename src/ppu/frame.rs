const WIDTH: usize = 256;
const HEIGHT: usize = 240;
// width is multiplied by 3, because each pixel needs 3 u8 values to represent its RGB colors
const SIZE: usize = (WIDTH * 3 * HEIGHT) as usize; 

pub struct Frame {
    pub bytes: [u8; SIZE],
}

impl Frame {
    pub fn new() -> Self {
        Frame { bytes: [0; SIZE]}
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

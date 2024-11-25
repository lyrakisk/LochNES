#[derive(PartialEq, Debug)]
pub enum WriteToggle {
    FirstWrite,
    SecondWrite,
}

impl WriteToggle {
    pub fn toggle(&mut self) {
        match self {
            Self::FirstWrite => *self = Self::SecondWrite,
            Self::SecondWrite => *self = Self::FirstWrite,
        }
    }
}

#[cfg(test)]
mod test_write_toggle {
    use super::*;

    #[test]
    fn test_toggle() {
        let mut w = WriteToggle::FirstWrite;

        w.toggle();
        assert_eq!(WriteToggle::SecondWrite, w);

        w.toggle();
        assert_eq!(WriteToggle::FirstWrite, w)
    }
}

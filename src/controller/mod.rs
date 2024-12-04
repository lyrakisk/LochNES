const RIGHT: u8 = 0b10000000;
const LEFT: u8 = 0b01000000;
const DOWN: u8 = 0b00100000;
const UP: u8 = 0b00010000;
const START: u8 = 0b00001000;
const SELECT: u8 = 0b00000100;
const BUTTON_B: u8 = 0b00000010;
const BUTTON_A: u8 = 0b00000001;

#[derive(PartialEq, Debug)]
enum StrobeMode {
    ON,
    OFF,
}

#[derive(PartialEq, Debug)]
enum ButtonState {
    RELEASED = 0,
    PRESSED = 1,
}

pub struct Controller {
    strobe_mode: StrobeMode,
    index: u8,
    status: u8,
}

impl Controller {
    pub const RIGHT: u8 = RIGHT;
    pub const LEFT: u8 = LEFT;
    pub const DOWN: u8 = DOWN;
    pub const UP: u8 = UP;
    pub const START: u8 = START;
    pub const SELECT: u8 = SELECT;
    pub const BUTTON_B: u8 = BUTTON_B;
    pub const BUTTON_A: u8 = BUTTON_A;
    pub fn new() -> Self {
        return Controller {
            strobe_mode: StrobeMode::OFF,
            index: 0,
            status: 0b0000_0000,
        };
    }

    pub fn read_u8(&mut self) -> u8 {
        return self.read() as u8;
    }

    pub fn write(&mut self, data: u8) {
        assert!(data == 0 || data == 1);

        if data == 0 {
            self.strobe_mode = StrobeMode::OFF;
        } else {
            self.strobe_mode = StrobeMode::ON;
            self.index = 0;
        }
    }
    fn read(&mut self) -> ButtonState {
        if self.index > 7 {
            return ButtonState::PRESSED
        }
        let result = match self.strobe_mode {
            StrobeMode::ON => self.status & BUTTON_A,
            StrobeMode::OFF => {
                let mut mask = 1;
                mask = mask << self.index;
                self.index += 1;
                self.status & mask
            },
        };

        if result == 0 {
            return ButtonState::RELEASED;
        } else {
            return ButtonState::PRESSED;
        }
    }

    pub fn press_button(&mut self, button: u8) {
        self.status = self.status | button;
    }

    pub fn release_button(&mut self, button: u8) {
        self.status = self.status & (!button);
    }

}

#[cfg(test)]
mod test_controller {
    use super::*;
    use test_case::test_case;
    #[test]
    fn test_controller_init_state() {
        let controller = Controller::new();
        assert_eq!(0b0000_0000, controller.status);
        assert_eq!(0, controller.index);
        assert_eq!(StrobeMode::OFF, controller.strobe_mode);
    }

    #[test_case(0b0000_0001, ButtonState::PRESSED)]
    #[test_case(0b0000_0000, ButtonState::RELEASED)]
    fn test_read_when_strobe_mode_is_on(controller_status: u8, expected_button_state: ButtonState) {
        let mut controller = Controller::new();
        controller.strobe_mode = StrobeMode::ON;

        controller.status = controller_status;
        assert_eq!(expected_button_state, controller.read());
    }

    #[test]
    fn test_read_when_strobe_mode_is_off() {}

    #[test]
    fn test_read_u8_when_strobe_mode_is_off() {}

    #[test]
    fn test_read_u8_when_strobe_mode_is_on() {}

    #[test]
    fn test_write() {}
    
}

use sdl2::keyboard::Keycode;

pub struct Keypad {
    keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { keys: [false; 16] }
    }

    pub fn is_key_pressed(&self, key: u8) -> Result<bool, String> {
        let key = key as usize;

        if key >= self.keys.len() {
            return Err(format!(
                "Could not find key '{}' while trying to see if it's pressed.",
                key
            ));
        }

        Ok(self.keys[key])
    }

    pub fn press_key(&mut self, keycode: Keycode) {
        let mapped_key = Keypad::map_keycode(keycode);

        if let Some(key) = mapped_key {
            self.keys[key] = true;
        }
    }

    pub fn release_key(&mut self, keycode: Keycode) {
        let mapped_key = Keypad::map_keycode(keycode);

        if let Some(key) = mapped_key {
            self.keys[key] = false;
        }
    }

    fn map_keycode(keycode: Keycode) -> Option<usize> {
        // TODO: abstract this a bit so we can support other keypad configs
        match keycode {
            Keycode::Num0 => Some(0x0),
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0x4),
            Keycode::Num5 => Some(0x5),
            Keycode::Num6 => Some(0x6),
            Keycode::Num7 => Some(0x7),
            Keycode::Num8 => Some(0x8),
            Keycode::Num9 => Some(0x9),
            Keycode::A => Some(0xA),
            Keycode::B => Some(0xB),
            Keycode::C => Some(0xC),
            Keycode::D => Some(0xD),
            Keycode::E => Some(0xE),
            Keycode::F => Some(0xF),
            _ => None,
        }
    }
}

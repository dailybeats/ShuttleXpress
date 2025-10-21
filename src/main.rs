use enigo::{Enigo, Key, Keyboard, Settings};
use hidapi::{HidApi, HidDevice};

struct ShuttleXpress {
    device: HidDevice,
    previous_wheel: u8,

    // Operations
    pub button1: Option<Key>,
    pub button2: Option<Key>,
    pub button3: Option<Key>,
    pub button4: Option<Key>,
    pub button5: Option<Key>,
    pub wheel: Option<(Key, Key)>,
    pub spring_wheel: Option<Key>,
}

impl ShuttleXpress {
    fn new() -> Self {
        let api = HidApi::new().unwrap();

        let (vid, pid) = (0x0B33, 0x0020);
        let device = api.open(vid, pid).unwrap();
        println!(
            "Connected: {}",
            device.get_device_info().unwrap().product_string().unwrap()
        );

        Self {
            device,
            previous_wheel: 0x0,
            button1: None,
            button2: None,
            button3: None,
            button4: None,
            button5: None,
            wheel: Some((Key::VolumeDown, Key::VolumeUp)),
            spring_wheel: None,
        }
    }

    // fn from_buffer(&mut self, &buffer: &[u8]) -> ShuttleXpressControl {
    //     return Self::None;
    // }

    pub fn get_key(&mut self) -> Option<Key> {
        let mut buffer = [0x0; 8];
        if let Ok(n) = self.device.read(&mut buffer) {
            println!("{:?}", &buffer[..n]);
            if buffer[0] != 0 {
            } else if buffer[1] != 0 && self.previous_wheel != buffer[1] {
                let previous_value = self.previous_wheel;
                self.previous_wheel = buffer[1];
                if let Some(keys) = self.wheel {
                    if buffer[1] > previous_value {
                        return Some(keys.0);
                    } else {
                        return Some(keys.1);
                    }
                }
            } else if buffer[3] == 16 {
                // return Self::Button1;
            } else if buffer[3] == 32 {
                // return Self::Button2;
            } else if buffer[3] == 64 {
                // return Self::Button3;
            } else if buffer[3] == 128 {
                // return Self::Button4;
            } else if buffer[4] == 1 {
                // return Self::Button5;
            }
        }
        return None;
    }
}

fn main() {
    let mut shuttle_xpress = ShuttleXpress::new();
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    loop {
        if let Some(key) = shuttle_xpress.get_key() {
            match enigo.key(key, enigo::Direction::Click) {
                Ok(_) => println!("Volume"),
                Err(e) => println!("{}", e.to_string()),
            }
        }
    }
}

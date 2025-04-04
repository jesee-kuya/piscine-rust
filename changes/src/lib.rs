#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
    pub alias: String,
    pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Self {
        Self {
            alias: String::from(alias),
            brightness: 0,
        }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for light in lights {
        if light.alias == alias {
            light.brightness = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changes() {
        let light1 = Light{
            alias: "zone01".to_string(),
            brightness: 0,
        };

        let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);
        change_brightness(&mut lights, "living_room", 200);

        assert_eq!(light1, Light::new("zone01"));
        assert_eq!(lights[0], Light{
            brightness: 200,
            alias: "living_room".to_string(),
        });
    }
}

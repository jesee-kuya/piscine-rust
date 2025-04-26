#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        let actual_position = Object {
            x: init_position.x,
            y: init_position.y,
        };
        let actual_velocity = Object {
            x: init_velocity.x,
            y: init_velocity.y,
        };
        ThrowObject {
            init_position,
            init_velocity,
            actual_position,
            actual_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        let new_time = self.time + 1.0;

        let new_x = self.init_position.x + self.init_velocity.x * new_time;
        let new_y = self.init_position.y + self.init_velocity.y * new_time - 0.5 * 9.8 * new_time.powi(2);

        let new_vx = self.init_velocity.x;
        let new_vy = self.init_velocity.y - 9.8 * new_time;

        if new_y >= 0.0 {
            let next_state = ThrowObject {
                init_position: self.init_position.clone(),
                init_velocity: self.init_velocity.clone(),
                actual_position: Object { x: new_x, y: new_y },
                actual_velocity: Object { x: new_vx, y: new_vy },
                time: new_time,
            };

            self.actual_position = next_state.actual_position.clone();
            self.actual_velocity = next_state.actual_velocity.clone();
            self.time = next_state.time;

            Some(next_state)
        } else {
            None
        }
    }
}
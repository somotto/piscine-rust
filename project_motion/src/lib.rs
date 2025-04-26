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
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        let g = 9.8;

        self.time += 1.0;

        let new_x = self.init_position.x + self.init_velocity.x * self.time;
        let new_y = self.init_position.y + self.init_velocity.y * self.time - 0.5 * g * self.time * self.time;

        let new_vx = self.init_velocity.x;
        let new_vy = self.init_velocity.y - g * self.time;

        if new_y <= 0.0 {
            return None;
        }

        self.actual_position = Object { x: new_x, y: new_y };
        self.actual_velocity = Object { x: new_vx, y: new_vy };

        Some(self.clone())
    }
}

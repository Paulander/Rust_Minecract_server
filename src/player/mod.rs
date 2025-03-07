use uuid::Uuid;

#[derive(Debug)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
}

impl Player {
    pub fn new(username: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            x: 0.0,
            y: 70.0,
            z: 0.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
} 
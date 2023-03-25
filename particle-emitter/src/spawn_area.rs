use rand::random;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub enum SpawnArea {
    Point(f32, f32),
    Rectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    Circle {
        x: f32,
        y: f32,
        radius: f32,
    },
}

impl SpawnArea {
    pub fn gen_coord(&self) -> (f32, f32) {
        match *self {
            Self::Point(x, y) => (x, y),
            SpawnArea::Rectangle {
                x,
                y,
                width,
                height,
            } => {
                let x = width * random::<f32>() + x;
                let y = height * random::<f32>() + y;
                (x, y)
            }
            Self::Circle { x, y, radius } => {
                let r = random::<f32>().sqrt() * radius;
                let theta = random::<f32>() * PI * 2.0;

                let x = x + r * theta.cos();
                let y = y + r * theta.sin();

                (x, y)
            }
        }
    }
}

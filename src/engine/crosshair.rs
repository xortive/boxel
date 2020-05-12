use crate::primitives::CrosshairVertex;
use glium::{Display, VertexBuffer};

pub struct Crosshair {
    pub vbo: VertexBuffer<CrosshairVertex>
}

impl Crosshair {
    pub fn new(display: &Display) -> Crosshair {
        let instances: Vec<CrosshairVertex> = vec![
            CrosshairVertex {
                screen_position: [-0.02, 0.]
            },
            CrosshairVertex {
                screen_position: [0.02, 0.]
            },
            CrosshairVertex {
                screen_position: [0., -0.02]
            },
            CrosshairVertex {
                screen_position: [0., 0.02]
            }
        ];

        let vbo = VertexBuffer::new(display, &instances).expect("to create vb");
        Crosshair {
            vbo
        }
    }
}
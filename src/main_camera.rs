use rendering::camera;
use winit::keyboard::KeyCode;

pub struct MainCamera {
    buffer: Option<camera::CameraBuffer>,
    position: cgmath::Vector2<f32>,

    delta_times: Vec<f64>,
    movement: cgmath::Vector2<f32>,
}

impl MainCamera {
    pub fn new(position: cgmath::Vector2<f32>) -> Self {
        Self {
            buffer: None,
            position,
            delta_times: vec![],
            movement: cgmath::Vector2::new(0.0, 0.0),
        }
    }
}

impl camera::Camera for MainCamera {
    fn buffer(&self) -> &Option<camera::CameraBuffer> {&self.buffer}
    fn position(&self) -> cgmath::Vector2<f32> {self.position}
    fn interested_keys(&self) -> Vec<KeyCode> {
        vec![KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowUp, KeyCode::ArrowDown]
    }

    fn init_buffer(&mut self, buffer: camera::CameraBuffer) {
        self.buffer = Some(buffer);
    }

    fn key_event(&mut self, key: KeyCode, pressed: bool) {
        if key == KeyCode::ArrowLeft{
            if pressed {
                self.movement.x += 0.5;
            } else {
                self.movement.x -= 0.5;
            }
        }
        if key == KeyCode::ArrowRight{
            if pressed {
                self.movement.x += -0.5;
            } else {
                self.movement.x -= -0.5;
            }
        }
        if key == KeyCode::ArrowUp{
            if pressed {
                self.movement.y += -0.5;
            } else {
                self.movement.y -= -0.5;
            }
        }
        if key == KeyCode::ArrowDown{
            if pressed {
                self.movement.y += 0.5;
            } else {
                self.movement.y -= 0.5;
            }
        }
    }

    fn start(&mut self) {
        
    }

    fn update(&mut self, delta_time: f64) {    
        // if self.delta_times.len() > 3 {
            //     println!("fps: {:.1}", self.delta_times.len() as f64 / self.delta_times.iter().sum::<f64>());
            //     self.delta_times = vec![];
            // }
        // if 1.0 / delta_time < 50.0 {
        //     println!("{}", 1.0 / delta_time);
        // }
        
        // self.position += self.movement * delta_time as f32;

        // if self.position.x < -1.0 {
        //     self.movement.x = 0.5;
        // } else if self.position.x > 1.0 {
        //     self.movement.x = -0.5;
        // }

        self.position += self.movement * delta_time as f32;
    }
}
pub struct Transform {
    dimensions: cgmath::Vector2<f32>,
    position: cgmath::Vector2<f32>,
    changed: bool,
}

impl Transform {
    pub fn dimensions(&self) -> cgmath::Vector2<f32> {self.dimensions}
    pub fn position(&self) -> cgmath::Vector2<f32> {self.position}
    pub fn changed(&self) -> bool {self.changed}
    
    pub fn new(dimensions: cgmath::Vector2<f32>, position: cgmath::Vector2<f32>) -> Self {
        Self { 
            dimensions,
            position,
            changed: false,
        }
    }

    pub fn set_position(&mut self, position: cgmath::Vector2<f32>) {
        self.position = position;
        self.changed = true;
    }

    pub fn change_position(&mut self, delta: cgmath::Vector2<f32>) {
        self.position += delta;
        self.changed = true;
    }

    pub fn set_dimensions(&mut self, dimensions: cgmath::Vector2<f32>) {
        self.dimensions = dimensions;
        self.changed = true;
    }

    pub fn change_handled(&mut self) {
        self.changed = false;
    }
}
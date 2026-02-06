pub struct Transform {
    dimensions: cgmath::Vector2<f32>,
    true_dimensions: cgmath::Vector2<f32>,
    position: cgmath::Vector2<f32>,
    true_position: cgmath::Vector2<f32>,
    changed: bool,
    visible: bool,
    visible_changed: bool,
}

impl Transform {
    pub fn dimensions(&self) -> cgmath::Vector2<f32> {self.true_dimensions}
    pub fn position(&self) -> cgmath::Vector2<f32> {self.true_position}
    pub(crate) fn changed(&self) -> bool {self.changed}
    pub fn visible(&self) -> bool {self.visible}
    pub(crate) fn visible_changed(&self) -> bool {self.visible_changed}
    
    pub(crate) fn correct_shape_and_pos(&mut self, width: u32, height: u32) {
        self.true_dimensions.y = (width as f32 / height as f32) * self.dimensions.y;
        self.true_position.y = (width as f32 / height as f32) * self.position.y;

        self.changed = true;
    }

    pub fn new(dimensions: cgmath::Vector2<f32>, position: cgmath::Vector2<f32>) -> Self {
        Self { 
            dimensions,
            true_dimensions: dimensions,
            position,
            true_position: position,
            changed: false,
            visible: true,
            visible_changed: false,
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

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
        self.visible_changed = true;
    }

    pub(crate) fn visible_change_handled(&mut self) {
        self.visible_changed = false;
    }

    pub(crate) fn change_handled(&mut self) {
        self.changed = false;
    }
}
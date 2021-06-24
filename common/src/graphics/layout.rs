pub enum LayoutType {
    Single,
    Row,
    Column
}

pub struct Layout {
    pub id: Option<u64>,
    pub layout_type: LayoutType,
    pub force: f32,
    pub inner_margin: f32,
    pub outer_margin: f32,
    pub scale: f32,
    pub aspect_ratio: Option<f32>,
    pub children: Vec<Layout>
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            id: None,
            layout_type: LayoutType::Single,
            force: 1.,
            inner_margin: 0.,
            outer_margin: 0.,
            scale: 1.,
            aspect_ratio: None,
            children: vec![]
        }
    }
}
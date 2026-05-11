#[allow(dead_code)]
#[derive(Debug)]
pub struct App {
    pub name: String,
    pub port: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Deployment {
    pub app_name: String,
    pub image_tag: String,
    pub port: u16,
    pub container_id: String,
    pub deployed_at: String,
    pub status: String,
}
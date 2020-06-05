use winapi::shared::windef::RECT;

#[derive(Clone)]
pub struct Window {
    pub id: i32,
    pub name: String,
    pub original_style: i32,
    pub original_rect: RECT
}
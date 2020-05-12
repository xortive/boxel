use derive_more::Constructor;
use glium::implement_vertex;
#[derive(Copy, Clone, Constructor)]
pub struct InstanceAttr {
    pub world_position: (f32, f32, f32),
    pub color: (f32, f32, f32, f32),
}

implement_vertex!(InstanceAttr, world_position, color);

#[derive(Copy, Clone, Constructor)]
pub struct CrosshairVertex {
    pub screen_position: [f32; 2],
}

implement_vertex!(CrosshairVertex, screen_position);

#[derive(Copy, Clone, Constructor)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

#[derive(Copy, Clone, Constructor)]
pub struct ColorVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

implement_vertex!(ColorVertex, position, normal, color);

#[derive(Copy, Clone, Constructor)]
pub struct TextureVertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texture: [f32; 2],
}

implement_vertex!(TextureVertex, position, normal, texture);

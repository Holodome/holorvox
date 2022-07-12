use crate::renderer::misc::{Quad, Setup, Vertex};
use crate::Vec4;

pub enum Command {
    Clear(Vec4),
    Quad(CommandQuad),
    Setup(Setup),
}

pub struct CommandQuad {
    quad_count: usize,
    vertex_buffer_offset: usize,
    index_buffer_offset: usize,
}

pub struct Commands {
    commands: Vec<Command>,
    vertices: Vec<Vertex>,
    indices: Vec<usize>,
}

pub struct QuadRenderData {
    vertex_buffer_offset: usize,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn clear(&mut self, c: Vec4) {
        self.commands.push(Command::Clear(c))
    }

    pub fn quad(&mut self, quad: Quad) {
        let command = self.get_current_quads();

        self.vertices.push(Vertex::new(
            quad.v00,
            quad.uv00,
            quad.c00,
        ));
        self.vertices.push(Vertex::new(
            quad.v01,
            quad.uv01,
            quad.c01,
        ));
        self.vertices.push(Vertex::new(
            quad.v10,
            quad.uv10,
            quad.c10,
        ));
        self.vertices.push(Vertex::new(
            quad.v11,
            quad.uv11,
            quad.c11,
        ));

        let base_index = self.vertices.len() - command.vertex_buffer_offset;
        self.indices.push(base_index + 0);
        self.indices.push(base_index + 2);
        self.indices.push(base_index + 3);
        self.indices.push(base_index + 0);
        self.indices.push(base_index + 1);
        self.indices.push(base_index + 3);
    }

    pub fn setup(&mut self, setup: Setup) {
        self.commands.push(Command::Setup(setup));
    }

    fn get_current_quads(&mut self) -> QuadRenderData {
        if let Some(last) = self.commands.last_mut() {
            match last {
                Command::Quad(q) => {
                    q.quad_count += 1;
                    return QuadRenderData {
                        vertex_buffer_offset: q.vertex_buffer_offset
                    };
                },
                _ => {}
            }
        };

        let command = Command::Quad(CommandQuad {
            quad_count: 1,
            vertex_buffer_offset: self.vertices.len(),
            index_buffer_offset: self.indices.len(),
        });
        self.commands.push(command);

        match self.commands.last_mut().unwrap() {
            Command::Quad(q) => QuadRenderData {
                vertex_buffer_offset: q.vertex_buffer_offset
            },
            _ => panic!("Unexpected behaviour"),
        }
    }
}
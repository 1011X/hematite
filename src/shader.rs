extern crate device;

use vecmath::Matrix4;
use gfx;
use gfx::{Device, DeviceHelper};

static VERTEX: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    uniform mat4 projection, view;

    attribute vec2 tex_coord;
    attribute vec3 color, position;

    varying vec2 v_tex_coord;
    varying vec3 v_color;

    void main() {
        v_tex_coord = tex_coord;
        v_color = color;
        gl_Position = projection * view * vec4(position, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    uniform mat4 projection, view;

    in vec2 tex_coord;
    in vec3 color, position;

    out vec2 v_tex_coord;
    out vec3 v_color;

    void main() {
        v_tex_coord = tex_coord;
        v_color = color;
        gl_Position = projection * view * vec4(position, 1.0);
    }
"
};

static FRAGMENT: gfx::ShaderSource = shaders!{
GLSL_120: b"
    #version 120

    uniform sampler2D s_texture;

    varying vec2 v_tex_coord;
    varying vec3 v_color;

    void main() {
        vec4 tex_color = texture2D(s_texture, v_tex_coord);
        if(tex_color.a == 0.0) // Discard transparent pixels.
            discard;
        gl_FragColor = tex_color * vec4(v_color, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    out vec4 out_color;

    uniform sampler2D s_texture;

    in vec2 v_tex_coord;
    in vec3 v_color;

    void main() {
        vec4 tex_color = texture(s_texture, v_tex_coord);
        if(tex_color.a == 0.0) // Discard transparent pixels.
            discard;
        out_color = tex_color * vec4(v_color, 1.0);
    }
"
};

#[shader_param(Program)]
pub struct ShaderParam {
    pub projection: [[f32, ..4], ..4],
    pub view: [[f32, ..4], ..4],
    pub s_texture: gfx::shade::TextureParam,
}

#[vertex_format]
pub struct Vertex {
    #[name="position"]
    pub xyz: [f32, ..3],
    #[name="tex_coord"]
    pub uv: [f32, ..2],
    #[name="color"]
    pub rgb: [f32, ..3],
}

impl Clone for Vertex {
    fn clone(&self) -> Vertex {
        *self
    }
}

pub struct Buffer {
    buf: gfx::BufferHandle<Vertex>,
    len: u32
}

pub struct Renderer<D: gfx::Device> {
    device: D,
    renderer: gfx::Renderer,
    params: ShaderParam,
    frame: gfx::Frame,
    cd: gfx::ClearData,
    prog: Program,
    drawstate: gfx::DrawState
}

impl<D: gfx::Device> Renderer<D> {
    pub fn new(mut device: D, frame: gfx::Frame, tex: gfx::TextureHandle) -> Renderer<D> {
        let renderer = device.create_renderer();
        let sam = device.create_sampler(gfx::tex::SamplerInfo::new(gfx::tex::Scale, gfx::tex::Tile));

        let params = ShaderParam {
            projection: [[0.0, ..4], ..4],
            view: [[0.0, ..4], ..4],
            s_texture: (tex, Some(sam))
        };
        let prog = device.link_program(VERTEX.clone(), FRAGMENT.clone()).unwrap();
        let mut drawstate = gfx::DrawState::new().depth(gfx::state::LessEqual, true);
        drawstate.primitive.front_face = gfx::state::Clockwise;

        Renderer {
            device: device,
            renderer: renderer,
            params: params,
            frame: frame,
            cd: gfx::ClearData {
                color: Some([0.81, 0.8, 1.0, 1.0]),
                depth: Some(1.0),
                stencil: None,
            },
            prog: prog,
            drawstate: drawstate,
        }
    }

    pub fn set_projection(&mut self, proj_mat: Matrix4) {
        self.params.projection = proj_mat;
    }

    pub fn set_view(&mut self, view_mat: Matrix4) {
        self.params.view = view_mat;
    }

    pub fn reset(&mut self) {
        self.renderer.reset();
        self.renderer.clear(self.cd, &self.frame);
    }

    pub fn create_buffer(&mut self, data: &[Vertex]) -> Buffer {
        let buf = self.device.create_buffer(data.len(), gfx::UsageStatic);
        self.device.update_buffer(buf, &data, 0);
        Buffer { buf: buf, len: data.len() as u32 }
    }

    pub fn delete_buffer(&mut self, buf: Buffer) {
        self.device.delete_buffer(buf.buf);
    }

    pub fn render(&mut self, buffer: Buffer) {
        let mesh = gfx::Mesh::from(buffer.buf, buffer.len);
        self.renderer.draw(
                &mesh, 
                mesh.get_slice(device::TriangleList), 
                &self.frame, 
                (&self.prog, &self.params), 
                &self.drawstate
            ).unwrap();
    }

    pub fn end_frame(&mut self) {
        self.device.submit(self.renderer.as_buffer())
    }
}

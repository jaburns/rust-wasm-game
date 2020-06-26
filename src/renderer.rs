use js_sys::Math::random;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{
    window, HtmlCanvasElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader,
};

struct TriangleRenderer {
    gl: Rc<WebGlRenderingContext>,
    program: WebGlProgram,
    buffer: WebGlBuffer,
    num_verts: i32,
}

impl TriangleRenderer {
    pub fn new(gl: Rc<WebGlRenderingContext>) -> TriangleRenderer {
        let vert_shader = compile_shader(
            &gl,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec4 position;
            void main() {
                gl_Position = position;
            }
        "#,
        )
        .unwrap();

        let frag_shader = compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
            precision highp float;
            uniform vec3 u_color;
            void main() {
                gl_FragColor = vec4(u_color, 1.0);
            }
        "#,
        )
        .unwrap();

        let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

        let buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();

        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);

            gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        TriangleRenderer {
            gl: gl,
            program: program,
            buffer: buffer,
            num_verts: (vertices.len() / 3) as i32,
        }
    }

    pub fn draw_frame(&mut self) {
        let gl = &self.gl;

        gl.use_program(Some(&self.program));
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffer));

        gl.uniform3f(
            gl.get_uniform_location(&self.program, "u_color").as_ref(),
            random() as f32,
            random() as f32,
            random() as f32,
        );

        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, self.num_verts);
    }
}

pub struct Renderer {
    gl: Rc<WebGlRenderingContext>,
    triangle_renderer: TriangleRenderer,
}

impl Renderer {
    pub fn new() -> Renderer {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();

        let context = canvas
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        let rc_gl = Rc::new(context);

        let triangle_renderer = TriangleRenderer::new(rc_gl.clone());

        Renderer {
            gl: rc_gl,
            triangle_renderer: triangle_renderer,
        }
    }

    pub fn draw_frame(&mut self, dt: f32) {
        self.gl.clear_color(0.0, dt / 30f32, 0.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.triangle_renderer.draw_frame();
    }
}

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

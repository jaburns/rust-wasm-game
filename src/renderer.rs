use wasm_bindgen::JsCast;
use web_sys::{
    window, HtmlCanvasElement, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlRenderingContext,
    WebGlShader, WebGlTexture,
};

struct BufferRenderer {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    vertex_buffer: WebGlBuffer,
}

impl BufferRenderer {
    pub fn new(gl: WebGlRenderingContext) -> BufferRenderer {
        let vert_shader = compile_shader(
            &gl,
            WebGlRenderingContext::VERTEX_SHADER,
            include_str!("../shaders/bufferCopy.vert"),
        )
        .unwrap();

        let frag_shader = compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            include_str!("../shaders/bufferCopy.frag"),
        )
        .unwrap();

        let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

        let vertices: [f32; 12] = [
            -1f32, 1f32, -1f32, -1f32, 1f32, -1f32, 1f32, -1f32, 1f32, 1f32, -1f32, 1f32,
        ];

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

        BufferRenderer {
            gl: gl,
            program: program,
            vertex_buffer: buffer,
        }
    }

    pub fn draw_frame(&mut self, tex: &WebGlTexture) {
        let gl = &self.gl;

        gl.use_program(Some(&self.program));

        gl.active_texture(WebGlRenderingContext::TEXTURE0);
        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(tex));
        gl.uniform1i(gl.get_uniform_location(&self.program, "u_tex").as_ref(), 0);

        gl.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&self.vertex_buffer),
        );
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);

        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, 6);
    }
}

struct TriangleRenderer {
    gl: WebGlRenderingContext,
    program: WebGlProgram,
    buffer: WebGlBuffer,
    num_verts: i32,
}

impl TriangleRenderer {
    pub fn new(gl: WebGlRenderingContext) -> TriangleRenderer {
        let vert_shader = compile_shader(
            &gl,
            WebGlRenderingContext::VERTEX_SHADER,
            include_str!("../shaders/triangle.vert"),
        )
        .unwrap();

        let frag_shader = compile_shader(
            &gl,
            WebGlRenderingContext::FRAGMENT_SHADER,
            include_str!("../shaders/triangle.frag"),
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

    pub fn draw_frame(&mut self, t: f32) {
        let gl = &self.gl;

        gl.use_program(Some(&self.program));
        gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.buffer));

        gl.uniform1f(gl.get_uniform_location(&self.program, "u_time").as_ref(), t);

        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);

        gl.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, self.num_verts);
    }
}

pub struct Renderer {
    gl: WebGlRenderingContext,
    framebuffer: WebGlFramebuffer,
    framebuffer_tex: WebGlTexture,
    buffer_renderer: BufferRenderer,
    triangle_renderer: TriangleRenderer,
}

impl Renderer {
    pub fn new() -> Renderer {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();

        let gl = canvas
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        let framebuffer = gl.create_framebuffer().unwrap();
        gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, Some(&framebuffer));
        let framebuffer_tex = gl.create_texture().unwrap();

        gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&framebuffer_tex));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGlRenderingContext::TEXTURE_2D,
            0,
            WebGlRenderingContext::RGBA as i32,
            64,
            64,
            0,
            WebGlRenderingContext::RGBA,
            WebGlRenderingContext::UNSIGNED_BYTE,
            None,
        )
        .unwrap();

        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        gl.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );

        gl.framebuffer_texture_2d(
            WebGlRenderingContext::FRAMEBUFFER,
            WebGlRenderingContext::COLOR_ATTACHMENT0,
            WebGlRenderingContext::TEXTURE_2D,
            Some(&framebuffer_tex),
            0,
        );

        gl.clear_color(0.0, 0.0, 0.0, 1.0);

        let triangle_renderer = TriangleRenderer::new(gl.clone());
        let buffer_renderer = BufferRenderer::new(gl.clone());

        Renderer {
            gl: gl,
            framebuffer: framebuffer,
            framebuffer_tex: framebuffer_tex,
            buffer_renderer: buffer_renderer,
            triangle_renderer: triangle_renderer,
        }
    }

    pub fn draw_frame(&mut self, t: f32) {
        let gl = &self.gl;

        gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, Some(&self.framebuffer));
        gl.viewport(0, 0, 64, 64);
        gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.triangle_renderer.draw_frame(t);

        gl.bind_framebuffer(WebGlRenderingContext::FRAMEBUFFER, None);
        gl.viewport(0, 0, 512, 512);

        self.buffer_renderer.draw_frame(&self.framebuffer_tex);
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

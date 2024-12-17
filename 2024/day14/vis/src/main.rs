mod shader;
mod sim;

use anyhow::{anyhow, bail, Context as _};
use core::{slice, str};
use glad_gl::gl::{self, GLchar, GLenum, GLfloat, GLsizei, GLsizeiptr, GLuint};
use glfw::{Context, OpenGlProfileHint, WindowHint, WindowMode};
use nalgebra::{Matrix4, Orthographic3, Vector2, Vector3};
use shader::Shader;
use std::{
    env,
    ffi::{self, c_void},
    fs,
    ptr::null,
};

const TILE_WIDTH: i64 = 10;

#[rustfmt::skip]
const SQUARE: [GLfloat; 18] = [
    TILE_WIDTH as f32, 0.0,               0.0,
    TILE_WIDTH as f32, TILE_WIDTH as f32, 0.0,
    0.0,               TILE_WIDTH as f32, 0.0,
    0.0,               0.0,               0.0,
    0.0,               TILE_WIDTH as f32, 0.0,
    TILE_WIDTH as f32, 0.0,               0.0,
];

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        bail!(
            "usage: {} <input file> <room width> <room height> <states per second> <starting time>",
            args[0]
        );
    }

    let room_width: u64 = args[2]
        .parse()
        .context("room must have positive integer width")?;
    let room_height: u64 = args[3]
        .parse()
        .context("room must have positive integer height")?;
    let room_size = Vector2::new(room_width as i64, room_height as i64);
    let screen_size = Vector2::new(
        (room_width * TILE_WIDTH as u64) as u32,
        (room_height * TILE_WIDTH as u64) as u32,
    );

    let states_per_s: f64 = args[4]
        .parse()
        .context("states per second must be a number")?;
    if states_per_s <= 0f64 {
        bail!("states per second must be positive");
    }

    let starting_time: f64 = args[5].parse().context("starting time must be a number")?;
    if starting_time < 0f64 {
        bail!("starting time must be nonnegative");
    }

    let input =
        fs::read_to_string(&args[1]).with_context(|| format!("failed to read file {}", args[1]))?;
    let input =
        sim::parse_input(&input).with_context(|| format!("failed to parse file {}", args[1]))?;

    let states = sim::get_all_states(&input, room_size);

    let mut glfw = glfw::init_no_callbacks().context("failed to initialize GLFW")?;

    glfw.window_hint(WindowHint::ContextVersion(4, 6));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    let (mut win, _) = glfw
        .create_window(
            screen_size.x,
            screen_size.y,
            "Advent of Code 2024 Day 14 Visualization",
            WindowMode::Windowed,
        )
        .ok_or(anyhow!("failed to create window"))?;

    win.make_current();
    gl::load(|p| glfw.get_proc_address_raw(p));

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(message_callback, null());
        gl::DebugMessageControl(
            gl::DONT_CARE,
            gl::DONT_CARE,
            gl::DEBUG_SEVERITY_NOTIFICATION,
            0,
            null(),
            gl::FALSE,
        );
    }

    let shader = Shader::new(
        include_str!("./shaders/simple.vert"),
        include_str!("./shaders/simple.frag"),
        &[("model", 0), ("projection", 1), ("color", 2)],
    )
    .context("failed to create shader program")?;

    let mut vao = 0;
    let mut vbo = 0;
    unsafe {
        gl::CreateVertexArrays(1, &mut vao);

        gl::CreateBuffers(1, &mut vbo);
        gl::NamedBufferStorage(
            vbo,
            (24 * size_of::<GLfloat>()) as GLsizeiptr,
            SQUARE.as_ptr() as *const ffi::c_void,
            0,
        );

        gl::VertexArrayVertexBuffer(vao, 0, vbo, 0, (3 * size_of::<GLfloat>()) as i32);
        gl::EnableVertexArrayAttrib(vao, 0);
        gl::VertexArrayAttribFormat(vao, 0, 3, gl::FLOAT, gl::FALSE, 0);
        gl::VertexArrayAttribBinding(vao, 0, 0);
    }

    let projection = *Orthographic3::new(
        0.0,
        screen_size.x as f32,
        screen_size.y as f32,
        0.0,
        -1.0,
        1.0,
    )
    .as_matrix();

    glfw.set_time(starting_time);
    while !win.should_close() {
        win.swap_buffers();

        glfw.poll_events();

        let ctime = glfw.get_time();
        let cframe = ((states_per_s * ctime).floor() as usize) % states.len();
        let frametime = ctime - (cframe as f64) / states_per_s;
        let frameprop = states_per_s * frametime;

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader.use_program();
        shader.set_uniform_mat4f("projection", projection);
        for robot in &states[cframe] {
            let p: Vector3<f32> = Vector3::new(
                (robot.p.x * TILE_WIDTH) as f32,
                (robot.p.y * TILE_WIDTH) as f32,
                0f32,
            );
            let v: Vector3<f32> = Vector3::new(
                (robot.v.x * TILE_WIDTH) as f32,
                (robot.v.y * TILE_WIDTH) as f32,
                0f32,
            );
            let mut np = p + frameprop as f32 * v;
            if np.x >= screen_size.x as f32 {
                np.x -= screen_size.x as f32;
            }
            if np.x <= -TILE_WIDTH as f32 {
                np.x += screen_size.x as f32;
            }
            if np.y >= screen_size.y as f32 {
                np.y -= screen_size.y as f32;
            }
            if np.y <= -TILE_WIDTH as f32 {
                np.y += screen_size.y as f32;
            }
            let model = Matrix4::new_translation(&np);
            shader.set_uniform_mat4f("model", model);
            let color = Vector3::new(
                robot.color.x as f32,
                robot.color.y as f32,
                robot.color.z as f32,
            ) / 255f32;
            shader.set_uniform_3f("color", color);

            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
        }
    }

    Ok(())
}

extern "system" fn message_callback(
    source: GLenum,
    t: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    _: *mut c_void,
) {
    let message =
        str::from_utf8(unsafe { slice::from_raw_parts(message as *const u8, length as usize) })
            .unwrap_or("");

    let source = match source {
        gl::DEBUG_SOURCE_API => "API",
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => "WINDOW SYSTEM",
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER COMPILER",
        gl::DEBUG_SOURCE_THIRD_PARTY => "THIRD PARTY",
        gl::DEBUG_SOURCE_APPLICATION => "APPLICATION",
        _ => "OTHER",
    };

    let t = match t {
        gl::DEBUG_TYPE_ERROR => "ERROR",
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED BEHAVIOR",
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "UNDEFINED BEHAVIOR",
        gl::DEBUG_TYPE_PORTABILITY => "PORTABILITY",
        gl::DEBUG_TYPE_PERFORMANCE => "PERFORMANCE",
        gl::DEBUG_TYPE_MARKER => "MARKER",
        _ => "OTHER",
    };

    let severity = match severity {
        gl::DEBUG_SEVERITY_NOTIFICATION => "NOTIFICATION",
        gl::DEBUG_SEVERITY_LOW => "LOW",
        gl::DEBUG_SEVERITY_MEDIUM => "MEDIUM",
        gl::DEBUG_SEVERITY_HIGH => "HIGH",
        _ => "OTHER",
    };

    eprintln!("{}, {}, {}, {}: {}", source, t, severity, id, message);
}

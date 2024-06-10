use notan::prelude::*;

// language=glsl
const VERT: ShaderSource = notan::vertex_shader! {
    r#"
    #version 450
    layout(location = 0) in vec2 pos;

    void main() {
        gl_Position = vec4(pos, 1.0, 1.0);
    }
    "#
};

// language=glsl
const FRAG: ShaderSource = notan::fragment_shader! {
    r#"
    #version 450
    precision mediump float;

    layout(location = 0) out vec4 color;

    const float shadowMapResolution = 1.0 / 1024.0;

    vec4 fn(vec4 color) {
        float fact = 2.0;
        vec2 pos = vec2(23.5, 1298.2);
        vec2 anchor = vec2(23.5, 1298.2);
        float expFact = exp2(fact);
        vec2 newCoord = (pos - anchor) * expFact;
        float padding = 0.02 * expFact;
        if(any(lessThan(newCoord, vec2(0.0 - padding)) || greaterThan(newCoord, vec2(1.0 + padding)))) {
            return vec4(0.0, 0.0, 0.0, 1.0);
        }
        return vec4(0.0, 0.0, 0.0, 1.0);
    }

    void main() {
        vec4 col = vec4(0.0, 0.0, 0.0, 1.0);
        
        for (int i=0; i<300; i++)
            col = fn(col);
        color = col;
    }
    "#
};

#[derive(AppState)]
struct State {
    dt: i32,
    // clear_options: ClearOptions,
    pipeline: Pipeline,
    vbo: Buffer,
    vbo2: Buffer,
}

#[notan_main]
fn main() -> Result<(), String> {
    let config = WindowConfig::new()
        .set_title("Shader Benchmark")
        .set_size(1280, 720)
        .set_position(0, 0);

    notan::init_with(setup).add_config(config).draw(draw).build()
}

fn setup(gfx: &mut Graphics) -> State {
    // let clear_options = ClearOptions::color(Color::new(0.0, 0.0, 0.0, 1.0));

    let vertex_info = VertexInfo::new()
        .attr(0, VertexFormat::Float32x2)
        .attr(1, VertexFormat::Float32x3);

    let pipeline = gfx
        .create_pipeline()
        .from(&VERT, &FRAG)
        .with_vertex_info(&vertex_info)
        .build()
        .unwrap();

    #[rustfmt::skip]
    let vertices = [
        -1.0, 1.0,   1.0, 0.2, 0.3,  // top-left
        1.0, -1.0,   0.1, 1.0, 0.3,  // bottom-right
        -1.0, -1.0,   0.1, 0.2, 1.0,  // bottom-left
    ];

    let vbo = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices)
        .build()
        .unwrap();

    #[rustfmt::skip]
    let vertices2 = [
            -1.0, 1.0,   1.0, 0.2, 0.3,  // top-left
            1.0, -1.0,   0.1, 1.0, 0.3,  // bottom-right
            1.0, 1.0,   0.1, 0.2, 1.0,  // top-right
        ];

    let vbo2 = gfx
        .create_vertex_buffer()
        .with_info(&vertex_info)
        .with_data(&vertices2)
        .build()
        .unwrap();

    State {
        dt: 0,
        // clear_options,
        pipeline,
        vbo,
        vbo2,
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    state.dt += 1;
    let mut renderer = gfx.create_renderer();

    // renderer.begin(Some(state.clear_options));
    renderer.set_pipeline(&state.pipeline);

    renderer.bind_buffer(&state.vbo);
    renderer.draw(0, 3);
    renderer.bind_buffer(&state.vbo2);
    renderer.draw(0, 3);

    renderer.end();

    // println!("{:.1}", app.timer.fps());
    // println!("{}", state.dt);
    // gfx.render(&renderer);
    if state.dt == 10000 {
        app.exit();
    }
}

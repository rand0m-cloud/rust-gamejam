//https://github.com/bevyengine/bevy/blob/c2da7800e3671ad92e775529070a814d0bc2f5f8/crates/bevy_sprite/src/mesh2d/mesh2d.wgsl
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct MyMat {
    percentage: f32;
    color_1: vec4<f32>;
    color_2: vec4<f32>;
};

[[group(1), binding(0)]]
var<uniform> uniform_data: MyMat;

//Stolen from kayak_ui
// Where P is the position in pixel space, B is the size of the box adn R is the radius of the current corner.
fn sdRoundBox(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 
{
    var q = abs(p)-b+r;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - r;
}

fn round_rect(xy: vec2<f32>) -> f32 {
        var size = vec2<f32>(1.0, 1.0);
        var pos = xy * 2.0;
        // Lock border to max size. This is similar to how HTML/CSS handles border radius.
        var bs = min(0.1 * 2.0, min(size.x, size.y));
        var rect_dist = sdRoundBox(
            pos - size,
            size,
            bs,
        );
        return 1.0 - smoothStep(0.0, fwidth(rect_dist), rect_dist);
}

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> [[location(0)]] vec4<f32> {
    var background = vec4<f32>(0.8, 0.8, 0.8, 1.0);
    var output_color = vec4<f32>(0.0,0.0,0.0,1.0);
    if (uniform_data.percentage >= 0.0)  {
        output_color = uniform_data.color_1;
    } else {
        output_color = uniform_data.color_2;
    }
    if (input.uv.x > (uniform_data.percentage + 1.0) / 2.0)  {
       output_color.a = 0.0; 
    }
    //output_color = output_color * uniform_data.color;
    var color = (output_color.a * output_color.xyz) * background.xyz;
    return vec4<f32>(color, round_rect(input.uv));
}
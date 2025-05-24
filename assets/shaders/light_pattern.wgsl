struct ShaderParams {
    num_slits: u32,
    spacing: f32,
    wavelength: f32,
    velocity: f32,
    damping: f32,
}

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var<uniform> p: ShaderParams;

const TAU: f32 = radians(360.0);
const RED: vec3f = vec3f(236.0, 71.0, 8.0)/255.0;
const BLUE: vec3f = vec3f(0.0, 146.0, 202.0)/255.0;

@fragment
fn fragment(@location(0) world_position: vec4f) -> @location(0) vec4f {
    let pos = world_position.xz;

    let frequency = p.velocity / p.wavelength;

    let total_spacing = p.spacing * f32(p.num_slits - 1);
    let start_pos = -total_spacing / 2.0;

    var cos_sum = 0.0;
    var sin_sum = 0.0;

    for (var i: u32 = 0; i < p.num_slits; i++) {
        let slit_pos = vec2f(start_pos + f32(i) * p.spacing, 0.0);
        let slit_distance = distance(pos, slit_pos);
        let slit_amplitude = exp(-p.damping * slit_distance / p.velocity);

        let phi = TAU * slit_distance / p.wavelength;

        cos_sum += slit_amplitude * cos(phi);
        sin_sum += slit_amplitude * sin(phi);
    }

    let amplitude = sqrt(cos_sum*cos_sum + sin_sum*sin_sum) / f32(p.num_slits);

    return vec4f(1.0, 1.0, 1.0, amplitude*amplitude);
}
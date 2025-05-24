const TAU: f32 = radians(360.0);
const YELLOW: vec3f = vec3f(1.0, 1.0, 0.0);

struct ShaderParams {
    num_slits: u32,
    spacing: f32,
    wavelength: f32,
    velocity: f32,
    damping: f32,
}

@group(2) @binding(0) var<uniform> time: f32;
@group(2) @binding(1) var<uniform> p: ShaderParams;

@fragment
fn fragment(@location(0) world_position: vec4f) -> @location(0) vec4f {
    let pos = world_position.xz;

    let start_pos = -p.spacing * f32(p.num_slits - 1) / 2.0;                // get the position of the leftmost slit in space

    var cos_sum = 0.0;
    var sin_sum = 0.0;

    for (var i: u32 = 0; i < p.num_slits; i++) {
        let slit_pos = vec2f(start_pos + f32(i) * p.spacing, 0.0);          // get the 2d position of the slit in space
        let slit_distance = distance(pos, slit_pos);                        // find the distance between the slit and the fragment
        let slit_amplitude = exp(-p.damping * slit_distance / p.velocity);  // get the amplitude of the wave at this distance 

        let phi = TAU * slit_distance / p.wavelength;                       // find the relative phase offset based on distance from slit

        cos_sum += slit_amplitude * cos(phi);
        sin_sum += slit_amplitude * sin(phi);
    }

    let amplitude = sqrt(cos_sum * cos_sum + sin_sum * sin_sum) / f32(p.num_slits);

    return vec4f(YELLOW, amplitude * amplitude);
}
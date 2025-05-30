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

    // the algorithm i'm using for finding the amplitude of the combined waves is genuinely a complete mystery to me.
    // for reference, its sqrt[(sum_i=1^N(A_i * cos(phi_1)))^2 + (sum_i=1^N(A_i * sin(phi_1)))^2]
    // where N is the number of waves, A_i and phi_i the amplitude and phase offset of the ith wave
    // i believe that this is a formula used for finding the combined amplitude of an arbitrary number of phasors,
    // but the only time i could find this function explitcily defined was asking google gemini about it...
    // but hey, it works?
    // if anyone is somehow stumbing across this code and recognises this formula, please let me know, i'd love to learn more

    var cos_sum = 0.0;
    var sin_sum = 0.0;

    for (var i: u32 = 0; i < p.num_slits; i++) {                            // loop through all slits
        let slit_pos = vec2f(start_pos + f32(i) * p.spacing, 0.0);          // get the 2d position of the slit in space
        let slit_distance = distance(pos, slit_pos);                        // find the distance between the slit and the fragment
        let slit_amplitude = exp(-p.damping * slit_distance / p.velocity);  // get the amplitude of the wave at this distance

        let phi = TAU * slit_distance / p.wavelength;                       // find the relative phase offset based on distance from slit

        cos_sum += slit_amplitude * cos(phi);
        sin_sum += slit_amplitude * sin(phi);
    }

    let amplitude_squared = (cos_sum * cos_sum + sin_sum * sin_sum) / f32(p.num_slits * p.num_slits);

    // this is using the formula L * tan(arcsin(m * lambda / d)) for the locations of where the slits
    // should be. only works well if the distance of the plane is quite far away due to assumptions
    // made for this formula. one day i'll derive a formula that works even at small distances, but
    // that's a later problem.

    for (var i = -4; i <= 4; i++) {
        let ideal_location = -world_position.z * tan(asin(f32(i) * p.wavelength / p.spacing));
        let epsilon = -world_position.z * 0.0075;
        if abs(world_position.x - ideal_location) < epsilon {
            return vec4f(1.0);
        }
    }

    return vec4f(YELLOW, amplitude_squared);
}

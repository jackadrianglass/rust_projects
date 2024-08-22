struct UniformInput {
	blob: f32,
	spike: f32,
	power: f32,
};
@group(0) @binding(0)
var<uniform> uniform: UniformInput;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
	@location(0) fragCoord: vec2<f32>,
	@location(1) color: vec3<f32>,
};

fn distance_to_mandelbulb(point0: vec3<f32>, power: f32) -> f32 {
	// todo: put these into some uniforms
	let ublob: f32 = 0.05;
	let uspike: f32 = 20.0;

	let pi: f32 = 3.14159265359; 
	let bailout: f32 = 2.0;

	// the mandelbulb is at scene scene center. translate to origin.
	// let point = point0 - uniforms.sceneCenter;
	let point = point0;

	// params
	let blob = 1.0 - ublob;
	let spike = uspike * pi / 2.0;

	// iterate to find distance
	var z = point;
	var dr = 1.0;
	var dist: f32;    
	for (var step = 0; step < 16; step++) {
		dist = length(z);        
		if (dist > bailout) { break; }
		// to polar coordinates
		let theta = acos(z.z / dist) * power * blob;
		let phi = atan2(z.y, z.x) * power;
		// scale and rotate
		let distPowMinusOne = pow(dist, power - 1.0);
		let zr = distPowMinusOne * dist;
		dr = distPowMinusOne * power * dr + 1.0;
		// back to cartesian coordinates
		let sinTheta = sin(theta);
		z = zr * vec3<f32>(sinTheta * cos(phi), sin(phi + spike) * sinTheta, cos(theta));
		z += point;
	}
	return 0.5 * log(dist) * dist / dr;
}

fn distance_from_sphere(point: vec3<f32>, sphere_center: vec3<f32>, radius: f32) -> f32 {
	return length(point - sphere_center) - radius;
}

fn ray_march(ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> vec3<f32> {
	var distance_traveled: f32 = 0.0;
	let max_number_of_steps: i32 = 32;
	let min_hit_distance: f32 = 0.001;
	let max_trace_distance: f32 = 1000.0;

	for(var i = 0; i < max_number_of_steps; i += 1) {
		let current_position: vec3<f32> = ray_origin + distance_traveled * ray_direction;
		let distance_to_closest = map_the_world(current_position);

		if (distance_to_closest < min_hit_distance) {
			let normal = calculate_normal(current_position);

			let light_position = vec3<f32>(2.0, -5.0, 3.0);

			// Calculate the unit direction vector that points from
			// the point of intersection to the light source
			let direction_to_light = normalize(current_position - light_position);
			let diffuse_intensity = max(0.0, dot(normal, direction_to_light));

			return vec3<f32>(1.0, 0.0, 0.0) * diffuse_intensity;
		}

		if (distance_traveled > max_trace_distance) {
			break;
		}
		distance_traveled += distance_to_closest;
	}

	return vec3<f32>(0.0);
}

fn map_the_world(position: vec3<f32>) -> f32 {
	// let displacement = sin(5.0 * position.x) * sin(5.0 * position.y) * sin(5.0 * position.z) * 0.25;
    // let sphere_0 = distance_from_sphere(position, vec3<f32>(0.0), 1.0);
    // return sphere_0 + displacement;
	return distance_to_mandelbulb(position, 9.0);
}

fn calculate_normal(position: vec3<f32>) -> vec3<f32>
{
    let small_step = vec3<f32>(0.001, 0.0, 0.0);

    let gradient_x = map_the_world(position + small_step.xyy) - map_the_world(position - small_step.xyy);
    let gradient_y = map_the_world(position + small_step.yxy) - map_the_world(position - small_step.yxy);
    let gradient_z = map_the_world(position + small_step.yyx) - map_the_world(position - small_step.yyx);

    let normal = vec3(gradient_x, gradient_y, gradient_z);

    return normalize(normal);
}


@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    // Assign clip space position
    output.clip_position = vec4<f32>(in.position, 1.0);

    // Convert from clip space (-1 to 1) to normalized device coordinates (0 to 1)
    output.fragCoord = (in.position * 0.5 + 0.5).xy;
	output.color = in.color;

    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let uv = in.fragCoord * 2.0 - 1.0;
	let camera_position = vec3<f32>(0.0, 0.0, -2.5);
	let ray_origin = camera_position;
	let ray_direction = vec3<f32>(uv, 1.0);

	let shaded_color = ray_march(ray_origin, ray_direction);
    return vec4<f32>(shaded_color, 1.0);
}

#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) out vec3 color;

layout(set = 1, binding = 0) uniform Uniforms {
    mat4 u_view;
    mat4 u_proj;
};

void main() {
    v_tex_coords = a_tex_coords;

    vec4 projected = (u_proj * u_view) * vec4(a_position, 1.0);
    
    float ratio = 0.1 + (a_position.y * 2);
    float vertical_percent = (projected.y + 1.0) / 2.0;
    // Flip the percent so that 0 is the top and 1 is the bottom of screen
    vertical_percent = 1 - vertical_percent;

    float expand_amount = vertical_percent * ratio;

    projected.x += projected.x * expand_amount;

    //if (projected.x < 0) {
    //    projected.x -= expand_amount;
    //} else {
    //    projected.x += expand_amount;
    //}

    //gl_Position = projected;
    gl_Position = (u_proj * u_view) * vec4(a_position, 1.0);
    color = vec3(0.0, projected.z, 0.0);
}

 

 
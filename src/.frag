#version 330 core

in vec2 vTexCoord;

out vec4 Color;

uniform float u_time;

uniform sampler2D u_texture;

void main() {
    float sun_lifetime = mod(u_time, 6.0);
    Color = vec4(
        sun_lifetime / 6.0 * 0.8 + 0.2,
        0.2 + 0.4 * min(sun_lifetime, 3.0) / 3.0 - (0.6 * (max(sun_lifetime, 3.0) / 3.0 - 1.0)), 
        1.0 - sun_lifetime / 6.0, 
        1.0
    );
    Color = texture(u_texture, vTexCoord) * Color;
}
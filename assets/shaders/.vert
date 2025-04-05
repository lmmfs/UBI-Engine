#version 330 core

layout (location = 0) in vec3 aPosition;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 u_model_matrix;
uniform mat4 u_view_matrix;

out vec2 vTexCoord;

void main() {
    vec4 uv = u_view_matrix * u_model_matrix * vec4(aPosition, 1.0);

    gl_Position = uv;
    vTexCoord = aTexCoord;

}
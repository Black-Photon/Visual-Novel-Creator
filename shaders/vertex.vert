#version 450

layout (location = 0) in vec2 position;

struct Data {
    vec2 position;
};

out Data data;

uniform vec2 rect_position;
uniform vec2 rect_size;

void main() {
    vec4 pos = vec4(position * rect_size + rect_position, 0.0, 1.0);
    gl_Position = pos;
    data.position = position;
}
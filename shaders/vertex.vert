#version 450

layout (location = 0) in vec2 position;

struct Data {
    vec2 position;
};

out Data data;

void main() {
    vec4 pos = vec4(position, 0.0, 1.0);
    gl_Position = pos;
    data.position = position;
}
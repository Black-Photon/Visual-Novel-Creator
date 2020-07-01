#version 450

struct Data {
    vec2 position;
};

in Data data;

out vec4 colour;

void main() {
    colour = vec4(1.0, 0.0, 0.0, 1.0);
}
#version 450

struct Data {
    vec2 position;
};

in Data data;

out vec4 colour;

uniform sampler2D tex;

void main() {
    vec2 normalised = -(data.position + vec2(1.0)) / 2;
    colour = texture(tex, normalised);
}
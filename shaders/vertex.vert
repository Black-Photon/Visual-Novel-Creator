#version 450

layout (location = 0) in vec2 position;

struct Data {
    vec2 position;
};

out Data data;

uniform vec2 aspect_ratio;
uniform vec2 bl_anchor;
uniform vec2 tr_anchor;
uniform vec2 bl_pos;
uniform vec2 tr_pos;
uniform float depth;

void main() {
    vec2 abs_pos = (position + vec2(1.0))/2;
    vec2 anchor_pos = abs_pos * bl_anchor + (vec2(1.0) - abs_pos) * tr_anchor;
    vec2 aspect_pos = aspect_ratio * anchor_pos;
    vec2 model_pos = aspect_pos + abs_pos * bl_pos + (vec2(1.0) - abs_pos) * tr_pos;
    vec4 pos = vec4((model_pos / aspect_ratio * 2) - vec2(1.0), -depth, 1.0);
    gl_Position = pos;
    data.position = position;
}
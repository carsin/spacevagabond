#version 450

layout(location = 0) in vec2 i_Position;
layout(location = 1) in vec4 i_Color;
layout(location = 4) in vec3 i_Transform0;
layout(location = 5) in vec3 i_Transform1;
layout(location = 6) in vec3 i_Transform2;
mat3 i_Transform = mat3(i_Transform0, i_Transform1, i_Transform2);

layout(set = 0, binding = 0) uniform View {
	mat3 u_Camera;
};

layout(location = 0) out vec4 o_Color;

void main() {
    gl_Position = vec4(i_Transform * u_Camera * vec3(i_Position, 1), 1);
    o_Color = i_Color;
}
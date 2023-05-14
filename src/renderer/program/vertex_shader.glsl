#version 330 core

in float iX;
in float iY;
in vec3 iColor;

out vec3 Color;

void main() {
  gl_Position = vec4(iX - 0.5, iY - 0.5, 0.0, 1.0);
  Color = iColor;
}

#version 330 core

in float x;
in float y;

void main() {
  gl_Position = vec4(x - 0.5, y - 0.5, 0.0, 1.0);
}

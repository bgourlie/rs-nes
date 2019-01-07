#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 target0;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;

layout(set = 1, binding = 0) uniform UBOCol {
    vec3[64] PALETTE;
} color_dat;

void main() {
  vec4 pixel_data = texture(sampler2D(u_texture, u_sampler), v_uv);
  ivec4 pixel_bytes = ivec4(
      int(pixel_data.r * 255.0),
      int(pixel_data.g * 255.0),
      int(pixel_data.b * 255.0),
      int(pixel_data.a * 255.0)
  );

  int background_pixel_value = pixel_bytes.r & 0x3;
  int background_palette_index = pixel_bytes.r >> 2;
  int sprite_pixel_value = pixel_bytes.g & 0x3;
  int sprite_palette_index = pixel_bytes.g >> 2;
  int sprite_priority = pixel_bytes.b & 0x1;

  // TODO: Make branchless
  if ((background_pixel_value == 0 && sprite_pixel_value == 0) || sprite_pixel_value == 0) {
      target0 = vec4(color_dat.PALETTE[background_palette_index], 1.0);
  } else if (background_pixel_value == 0) {
      target0 = vec4(color_dat.PALETTE[sprite_palette_index], 1.0);
  } else {
      if (sprite_priority == 1) {
          target0 = vec4(color_dat.PALETTE[sprite_palette_index], 1.0);
      } else {
          target0 = vec4(color_dat.PALETTE[background_palette_index], 1.0);
      }
  }
}

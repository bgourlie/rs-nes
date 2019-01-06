#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec2 v_uv;
layout(location = 0) out vec4 target0;

layout(set = 0, binding = 0) uniform texture2D u_texture;
layout(set = 0, binding = 1) uniform sampler u_sampler;

layout(set = 1, binding = 0) uniform UBOCol {
    vec4 color;
} color_dat;

void main() {
  vec3 PALETTE[64];
  PALETTE[0] = vec3(0.48627451, 0.48627451, 0.48627451);
  PALETTE[1] = vec3(0.00000000, 0.00000000, 0.98823529);
  PALETTE[2] = vec3(0.00000000, 0.00000000, 0.73725492);
  PALETTE[3] = vec3(0.26666668, 0.15686275, 0.73725492);
  PALETTE[4] = vec3(0.58039218, 0.00000000, 0.51764709);
  PALETTE[5] = vec3(0.65882355, 0.00000000, 0.12549020);
  PALETTE[6] = vec3(0.65882355, 0.06274510, 0.00000000);
  PALETTE[7] = vec3(0.53333336, 0.07843138, 0.00000000);
  PALETTE[8] = vec3(0.31372550, 0.18823530, 0.00000000);
  PALETTE[9] = vec3(0.00000000, 0.47058824, 0.00000000);
  PALETTE[10] = vec3(0.00000000, 0.40784314, 0.00000000);
  PALETTE[11] = vec3(0.00000000, 0.34509805, 0.00000000);
  PALETTE[12] = vec3(0.00000000, 0.25098041, 0.34509805);
  PALETTE[13] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[14] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[15] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[16] = vec3(0.73725492, 0.73725492, 0.73725492);
  PALETTE[17] = vec3(0.00000000, 0.47058824, 0.97254902);
  PALETTE[18] = vec3(0.00000000, 0.34509805, 0.97254902);
  PALETTE[19] = vec3(0.40784314, 0.26666668, 0.98823529);
  PALETTE[20] = vec3(0.84705883, 0.00000000, 0.80000001);
  PALETTE[21] = vec3(0.89411765, 0.00000000, 0.34509805);
  PALETTE[22] = vec3(0.97254902, 0.21960784, 0.00000000);
  PALETTE[23] = vec3(0.89411765, 0.36078432, 0.06274510);
  PALETTE[24] = vec3(0.67450982, 0.48627451, 0.00000000);
  PALETTE[25] = vec3(0.00000000, 0.72156864, 0.00000000);
  PALETTE[26] = vec3(0.00000000, 0.65882355, 0.00000000);
  PALETTE[27] = vec3(0.00000000, 0.65882355, 0.26666668);
  PALETTE[28] = vec3(0.00000000, 0.53333336, 0.53333336);
  PALETTE[29] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[30] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[31] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[32] = vec3(0.97254902, 0.97254902, 0.97254902);
  PALETTE[33] = vec3(0.23529412, 0.73725492, 0.98823529);
  PALETTE[34] = vec3(0.40784314, 0.53333336, 0.98823529);
  PALETTE[35] = vec3(0.59607846, 0.47058824, 0.97254902);
  PALETTE[36] = vec3(0.97254902, 0.47058824, 0.97254902);
  PALETTE[37] = vec3(0.97254902, 0.34509805, 0.59607846);
  PALETTE[38] = vec3(0.97254902, 0.47058824, 0.34509805);
  PALETTE[39] = vec3(0.98823529, 0.62745100, 0.26666668);
  PALETTE[40] = vec3(0.97254902, 0.72156864, 0.00000000);
  PALETTE[41] = vec3(0.72156864, 0.97254902, 0.09411765);
  PALETTE[42] = vec3(0.34509805, 0.84705883, 0.32941177);
  PALETTE[43] = vec3(0.34509805, 0.97254902, 0.59607846);
  PALETTE[44] = vec3(0.00000000, 0.90980393, 0.84705883);
  PALETTE[45] = vec3(0.47058824, 0.47058824, 0.47058824);
  PALETTE[46] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[47] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[48] = vec3(0.98823529, 0.98823529, 0.98823529);
  PALETTE[49] = vec3(0.64313728, 0.89411765, 0.98823529);
  PALETTE[50] = vec3(0.72156864, 0.72156864, 0.97254902);
  PALETTE[51] = vec3(0.84705883, 0.72156864, 0.97254902);
  PALETTE[52] = vec3(0.97254902, 0.72156864, 0.97254902);
  PALETTE[53] = vec3(0.97254902, 0.64313728, 0.75294119);
  PALETTE[54] = vec3(0.94117647, 0.81568629, 0.69019610);
  PALETTE[55] = vec3(0.98823529, 0.87843138, 0.65882355);
  PALETTE[56] = vec3(0.97254902, 0.84705883, 0.47058824);
  PALETTE[57] = vec3(0.84705883, 0.97254902, 0.47058824);
  PALETTE[58] = vec3(0.72156864, 0.97254902, 0.72156864);
  PALETTE[59] = vec3(0.72156864, 0.97254902, 0.84705883);
  PALETTE[60] = vec3(0.00000000, 0.98823529, 0.98823529);
  PALETTE[61] = vec3(0.97254902, 0.84705883, 0.97254902);
  PALETTE[62] = vec3(0.00000000, 0.00000000, 0.00000000);
  PALETTE[63] = vec3(0.00000000, 0.00000000, 0.00000000);
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
      target0 = vec4(PALETTE[background_palette_index], 1.0);
  } else if (background_pixel_value == 0) {
      target0 = vec4(PALETTE[sprite_palette_index], 1.0);
  } else {
      if (sprite_priority == 1) {
          target0 = vec4(PALETTE[sprite_palette_index], 1.0);
      } else {
          target0 = vec4(PALETTE[background_palette_index], 1.0);
      }
  }
}

#version 450

layout(local_size_x = 16, local_size_y = 16, local_size_z = 1) in;

const uint SRC_W = 512;
const uint SRC_H = 512;
const uint OUT_W = 2400;
const uint OUT_H = 1600;
const uint IMAGE_COUNT = 10;

layout(set = 0, binding = 0) readonly buffer SourceImages {
    uint pixels[];
} sourceImages;

layout(set = 0, binding = 1) buffer OutputImage {
    uint pixels[];
} outputImage;

float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

float valueNoise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);
    float a = hash12(i);
    float b = hash12(i + vec2(1.0, 0.0));
    float c = hash12(i + vec2(0.0, 1.0));
    float d = hash12(i + vec2(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

float fbm(vec2 p) {
    float sum = 0.0;
    float amp = 0.5;
    for (int i = 0; i < 5; i++) {
        sum += valueNoise(p) * amp;
        p = mat2(1.7, -1.1, 1.1, 1.7) * p + 17.3;
        amp *= 0.5;
    }
    return sum;
}

vec3 unpackRgb(uint v) {
    return vec3(float(v & 255u), float((v >> 8) & 255u), float((v >> 16) & 255u)) / 255.0;
}

uint packRgb(vec3 c) {
    uvec3 q = uvec3(clamp(c, 0.0, 1.0) * 255.0 + 0.5);
    return q.r | (q.g << 8) | (q.b << 16) | 0xff000000u;
}

vec3 sampleImage(uint imageIndex, vec2 uv) {
    uv = fract(uv);
    uint x = uint(clamp(uv.x, 0.0, 0.99999) * float(SRC_W));
    uint y = uint(clamp(uv.y, 0.0, 0.99999) * float(SRC_H));
    return unpackRgb(sourceImages.pixels[imageIndex * SRC_W * SRC_H + y * SRC_W + x]);
}

vec3 sampleChromatic(uint imageIndex, vec2 uv, vec2 dir, float amount) {
    float r = sampleImage(imageIndex, uv + dir * amount).r;
    float g = sampleImage(imageIndex, uv).g;
    float b = sampleImage(imageIndex, uv - dir * amount * 1.25).b;
    return vec3(r, g, b);
}

vec3 grade(vec3 color) {
    color = pow(max(color, 0.0), vec3(0.82));
    color = (color - 0.5) * 1.38 + 0.5;
    color += vec3(0.03, -0.01, 0.05);
    color.r += smoothstep(0.45, 1.0, color.r) * 0.08;
    color.b += smoothstep(0.35, 1.0, color.b) * 0.07;
    return clamp(color, 0.0, 1.0);
}

void main() {
    uvec2 gid = gl_GlobalInvocationID.xy;
    if (gid.x >= OUT_W || gid.y >= OUT_H) {
        return;
    }

    vec2 uv = (vec2(gid) + 0.5) / vec2(OUT_W, OUT_H);
    vec2 centered = uv - 0.5;
    float aspect = float(OUT_W) / float(OUT_H);
    vec2 lens = vec2(centered.x * aspect, centered.y);
    float radius = length(lens);

    float scanBand = floor(uv.y * 32.0);
    float colBand = floor(uv.x * 24.0);
    float blockJitter = hash12(vec2(scanBand, colBand)) - 0.5;
    float tear = blockJitter * 0.045 * smoothstep(0.15, 0.75, radius);
    float n1 = fbm(uv * vec2(5.0, 3.0) + vec2(2.0, 8.0));
    float n2 = fbm(uv.yx * vec2(3.0, 6.0) + vec2(11.0, 3.0));
    vec2 displacement = vec2(n1 - 0.5, n2 - 0.5) * 0.055 + vec2(tear, 0.0);

    vec2 mosaic = uv * vec2(5.0, 3.0);
    uvec2 tile = uvec2(floor(mosaic));
    vec2 tileUv = fract(mosaic);
    float stagger = (float(tile.y % 2u) - 0.5) * 0.08 + blockJitter * 0.035;
    vec2 localUv = fract(tileUv + vec2(stagger, 0.0));
    uint tileId = (tile.x * 3u + tile.y * 5u) % IMAGE_COUNT;
    uint stripId = (uint(scanBand) + uint(floor(uv.x * 6.0)) * 2u) % IMAGE_COUNT;
    uint imageA = tileId;
    uint imageB = (stripId + uint(floor(n1 * 5.0))) % IMAGE_COUNT;
    uint imageC = (imageA + 4u + uint(floor(uv.y * 4.0))) % IMAGE_COUNT;

    vec2 uvA = localUv + displacement;
    vec2 uvB = vec2(fract(uv.x * 1.12 + n2 * 0.08 + blockJitter * 0.08), fract(uv.y * 0.96 + n1 * 0.1));
    vec2 uvC = fract(uv * mat2(1.04, 0.08, -0.05, 1.02) + displacement * 0.35);

    vec2 chromaDir = normalize(vec2(0.9 + blockJitter, -0.25 + n1) + vec2(0.0001));
    vec3 a = sampleChromatic(imageA, uvA, chromaDir, 0.005 + abs(blockJitter) * 0.008);
    vec3 b = sampleChromatic(imageB, uvB, vec2(1.0, 0.0), 0.007);
    vec3 c = sampleImage(imageC, uvC);

    float lumaA = dot(a, vec3(0.299, 0.587, 0.114));
    float lumaB = dot(b, vec3(0.299, 0.587, 0.114));
    float key = smoothstep(0.18, 0.92, fbm(uv * 10.0 + lumaA * 1.5));
    vec3 color = mix(a, b, key);
    color = mix(color, max(color, c), smoothstep(0.5, 0.9, lumaB) * 0.7);

    float slit = smoothstep(0.018, 0.0, abs(fract(uv.y * 24.0 + n1 * 0.25) - 0.5));
    color = mix(color, vec3(color.r * 1.14, color.g * 0.72, color.b * 1.28), slit * 0.32);

    for (int i = 1; i <= 3; i++) {
        float fi = float(i);
        vec2 trailUv = uv - vec2(0.025, -0.008) * fi;
        uint trailImage = (imageB + uint(i * 2)) % IMAGE_COUNT;
        vec3 trail = sampleImage(trailImage, trailUv + displacement * 0.25 * fi);
        color += trail * (0.035 / fi);
    }

    float grid = max(smoothstep(0.985, 1.0, fract(mosaic.x)), smoothstep(0.985, 1.0, fract(mosaic.y)));
    color = mix(color, vec3(1.0, 0.08, 0.65), grid * 0.2);

    float vignette = smoothstep(0.92, 0.18, radius);
    float scanline = 0.94 + 0.06 * sin(float(gid.y) * 1.9);
    color = grade(color) * mix(0.82, 1.08, vignette) * scanline;

    outputImage.pixels[gid.y * OUT_W + gid.x] = packRgb(color);
}

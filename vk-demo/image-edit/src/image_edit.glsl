#version 450

layout(local_size_x = 16, local_size_y = 16, local_size_z = 1) in;

const uint SRC_W = 1024;
const uint SRC_H = 1024;
const uint OUT_W = 2400;
const uint OUT_H = 1600;

layout(push_constant) uniform Params {
    uint imageCount;
} params;

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

bool matchesAvoid(uint imageIndex, uint avoid0, uint avoid1, uint avoid2, uint avoid3, uint avoid4, uint avoidCount) {
    return (avoidCount > 0u && imageIndex == avoid0) ||
           (avoidCount > 1u && imageIndex == avoid1) ||
           (avoidCount > 2u && imageIndex == avoid2) ||
           (avoidCount > 3u && imageIndex == avoid3) ||
           (avoidCount > 4u && imageIndex == avoid4);
}

uint pickDistinctImage(uint seed, uint avoid0, uint avoid1, uint avoid2, uint avoid3, uint avoid4, uint avoidCount) {
    uint imageCount = max(params.imageCount, 1u);
    uint candidate = seed % imageCount;
    for (uint offset = 0u; offset < imageCount; offset++) {
        uint imageIndex = (candidate + offset) % imageCount;
        if (!matchesAvoid(imageIndex, avoid0, avoid1, avoid2, avoid3, avoid4, avoidCount)) {
            return imageIndex;
        }
    }
    return candidate;
}

uint imageForCell(vec2 cell, uint salt) {
    uint x = uint(max(cell.x, 0.0));
    uint y = uint(max(cell.y, 0.0));
    return (x * 3u + y * 5u + salt) % params.imageCount;
}

vec2 preserveTileAspect(vec2 tileUv, float tileAspect) {
    if (tileAspect > 1.0) {
        tileUv.y = (tileUv.y - 0.5) / tileAspect + 0.5;
    } else {
        tileUv.x = (tileUv.x - 0.5) * tileAspect + 0.5;
    }
    return tileUv;
}

vec3 grade(vec3 color) {
    color = pow(max(color, 0.0), vec3(0.9));
    color = (color - 0.5) * 1.24 + 0.5;
    color += vec3(0.015, -0.012, 0.03);
    color.r += smoothstep(0.45, 1.0, color.r) * 0.035;
    color.b += smoothstep(0.35, 1.0, color.b) * 0.035;
    vec3 excess = max(color - vec3(0.74), vec3(0.0));
    color = min(color, vec3(0.74) + excess / (vec3(1.0) + excess * 5.0));
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

    vec2 section = floor(uv * vec2(4.0, 3.0));
    float sectionSeed = hash12(section);
    vec2 sectionScale = vec2(5.0, 3.0);
    if (sectionSeed < 0.2) {
        sectionScale = vec2(3.0, 2.0);
    } else if (sectionSeed < 0.4) {
        sectionScale = vec2(7.0, 2.5);
    } else if (sectionSeed < 0.63) {
        sectionScale = vec2(4.0, 5.0);
    } else if (sectionSeed < 0.82) {
        sectionScale = vec2(8.0, 4.0);
    }

    vec2 mosaic = uv * sectionScale + vec2(sectionSeed * 1.7, hash12(section + 19.0) * 1.3);
    uvec2 tile = uvec2(floor(mosaic));
    vec2 tileUv = fract(mosaic);
    float tileSeed = hash12(vec2(tile) + section * 13.0);
    float stagger = (float(tile.y % 2u) - 0.5) * (0.04 + sectionSeed * 0.13) + blockJitter * 0.035;
    vec2 localUv = fract(tileUv + vec2(stagger, 0.0));
    float tileAspect = (float(OUT_W) / float(OUT_H)) * (sectionScale.y / sectionScale.x);
    localUv = preserveTileAspect(localUv, tileAspect);
    uint tileId = (tile.x * 3u + tile.y * 5u) % params.imageCount;
    uint stripId = (uint(scanBand) + uint(floor(uv.x * 6.0)) * 2u) % params.imageCount;
    uint imageA = tileId;
    uint imageB = pickDistinctImage(stripId + uint(floor(n1 * 5.0)), imageA, 0u, 0u, 0u, 0u, 1u);
    uint imageC = pickDistinctImage(imageA + 4u + uint(floor(uv.y * 4.0)), imageA, imageB, 0u, 0u, 0u, 2u);

    vec2 uvA = localUv + displacement;
    vec2 uvB = vec2(fract(uv.x * (0.85 + sectionSeed * 0.7) + n2 * 0.08 + blockJitter * 0.08),
                    fract(uv.y * (1.2 - sectionSeed * 0.45) + n1 * 0.1));
    vec2 uvC = fract(uv * mat2(1.04, 0.08 + sectionSeed * 0.05, -0.05 - tileSeed * 0.07, 1.02) + displacement * 0.35);

    vec2 chromaDir = normalize(vec2(0.9 + blockJitter, -0.25 + n1) + vec2(0.0001));
    vec3 a = sampleChromatic(imageA, uvA, chromaDir, 0.005 + abs(blockJitter) * 0.008);
    vec3 b = sampleChromatic(imageB, uvB, vec2(1.0, 0.0), 0.007);
    vec3 c = sampleImage(imageC, uvC);

    float lumaA = dot(a, vec3(0.299, 0.587, 0.114));
    float lumaB = dot(b, vec3(0.299, 0.587, 0.114));
    float key = smoothstep(0.18, 0.92, fbm(uv * 10.0 + lumaA * 1.5));
    vec3 color = mix(a, b, key);
    color = mix(color, max(color, c), smoothstep(0.5, 0.9, lumaB) * 0.7);

    vec2 cell = floor(mosaic);
    float edgeNoise = fbm(uv * 34.0 + cell * 4.7) - 0.5;
    float fineNoise = fbm(uv * 95.0 + cell * 1.9) - 0.5;
    float raggedEdge = edgeNoise * 0.18 + fineNoise * 0.06;
    float leftPaint = smoothstep(0.42, 0.0, tileUv.x + raggedEdge) * mix(0.55, 1.0, hash12(cell + vec2(-3.0, 7.0)));
    float rightPaint = smoothstep(0.58, 1.0, tileUv.x + raggedEdge) * mix(0.55, 1.0, hash12(cell + vec2(5.0, 11.0)));
    float topPaint = smoothstep(0.36, 0.0, tileUv.y - raggedEdge) * mix(0.5, 0.95, hash12(cell + vec2(13.0, -2.0)));
    float bottomPaint = smoothstep(0.64, 1.0, tileUv.y - raggedEdge) * mix(0.5, 0.95, hash12(cell + vec2(-9.0, 17.0)));

    vec2 paintUv = preserveTileAspect(fract(tileUv + vec2(stagger, 0.0)), tileAspect);
    uint leftImage = pickDistinctImage(imageForCell(cell + vec2(-1.0, 0.0), 1u), imageA, imageB, imageC, 0u, 0u, 3u);
    uint rightImage = pickDistinctImage(imageForCell(cell + vec2(1.0, 0.0), 3u), imageA, imageB, imageC, leftImage, 0u, 4u);
    uint topImage = pickDistinctImage(imageForCell(cell + vec2(0.0, -1.0), 5u), imageA, imageB, imageC, leftImage, rightImage, 5u);
    uint bottomImage = pickDistinctImage(imageForCell(cell + vec2(0.0, 1.0), 7u), imageB, imageC, leftImage, rightImage, topImage, 5u);
    vec3 leftColor = sampleChromatic(leftImage, paintUv + displacement * 0.25, chromaDir, 0.004);
    vec3 rightColor = sampleChromatic(rightImage, paintUv + displacement * 0.25, chromaDir, 0.004);
    vec3 topColor = sampleChromatic(topImage, paintUv + displacement * 0.25, chromaDir, 0.004);
    vec3 bottomColor = sampleChromatic(bottomImage, paintUv + displacement * 0.25, chromaDir, 0.004);
    color = mix(color, leftColor, leftPaint * 0.38);
    color = mix(color, rightColor, rightPaint * 0.38);
    color = mix(color, topColor, topPaint * 0.32);
    color = mix(color, bottomColor, bottomPaint * 0.32);

    float slit = smoothstep(0.018, 0.0, abs(fract(uv.y * 24.0 + n1 * 0.25) - 0.5));
    color = mix(color, vec3(color.r * 1.14, color.g * 0.72, color.b * 1.28), slit * 0.32);

    uint trailImage1 = pickDistinctImage(imageB + 2u, imageA, imageB, imageC, 0u, 0u, 3u);
    uint trailImage2 = pickDistinctImage(imageB + 4u, imageA, imageB, imageC, trailImage1, 0u, 4u);
    uint trailImage3 = pickDistinctImage(imageB + 6u, imageA, imageB, imageC, trailImage1, trailImage2, 5u);
    vec3 trail1 = sampleImage(trailImage1, uv - vec2(0.025, -0.008) + displacement * 0.25);
    vec3 trail2 = sampleImage(trailImage2, uv - vec2(0.05, -0.016) + displacement * 0.5);
    vec3 trail3 = sampleImage(trailImage3, uv - vec2(0.075, -0.024) + displacement * 0.75);
    color += trail1 * 0.035 + trail2 * 0.0175 + trail3 * 0.011666667;

    // float grid = max(smoothstep(0.985, 1.0, fract(mosaic.x)), smoothstep(0.985, 1.0, fract(mosaic.y)));
    // color = mix(color, vec3(1.0, 0.08, 0.65), grid * 0.2);

    float vignette = smoothstep(0.92, 0.18, radius);
    float scanline = 0.94 + 0.06 * sin(float(gid.y) * 1.9);
    color = grade(color) * mix(0.80, 1.0, vignette) * scanline;

    outputImage.pixels[gid.y * OUT_W + gid.x] = packRgb(color);
}

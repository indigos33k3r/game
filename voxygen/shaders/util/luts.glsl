const uint grad2_a_lut[16] = uint[16](
	26u, // Dark grass
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u
);

const uint grad2_b_lut[16] = uint[16](
	1u,   // Stone
	133u, // Dry grass
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u,
	0u
);

const uint grad3_o_lut[2] = uint[2](
	154u, // Dark stone
	98u   // Earth
);

const uint grad3_a_lut[2] = uint[2](
	26u, // Dark grass
	0u
);

const uint grad3_b_lut[2] = uint[2](
	127u, // Sand
	7u   // Snow
);

const vec4 col_lut[256] = vec4[256](
    vec4(0.21875, 0.21875, 0.21875, 1.0),
	vec4(0.484375, 0.484375, 0.484375, 1.0),
	vec4(0.87890625, 0.5703125, 0.0, 1.0),
	vec4(0.28125, 0.61328125, 0.76953125, 1.0),
	vec4(0.13671875, 0.078125, 0.96875, 1.0),
	vec4(0.953125, 0.078125, 0.078125, 1.0),
	vec4(0.75390625, 0.75390625, 0.30078125, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.67578125, 0.28515625, 0.70703125, 1.0),
	vec4(0.67578125, 0.28515625, 0.70703125, 1.0),
	vec4(0.67578125, 0.28515625, 0.70703125, 1.0),
	vec4(0.67578125, 0.28515625, 0.70703125, 1.0),
	vec4(0.67578125, 0.28515625, 0.70703125, 1.0),
	vec4(0.67578125, 0.28515625, 0.70703125, 1.0),
	vec4(0.67578125, 0.28515625, 0.70703125, 1.0),
	vec4(0.67578125, 0.28515625, 0.70703125, 1.0),
	vec4(0.0625, 0.25, 0.0859375, 1.0),
	vec4(0.09765625, 0.375, 0.125, 1.0),
	vec4(0.125, 0.4921875, 0.16796875, 1.0),
	vec4(0.16015625, 0.62109375, 0.22265625, 1.0),
	vec4(0.0, 0.26953125, 0.01953125, 1.0),
	vec4(0.0, 0.41796875, 0.0390625, 1.0),
	vec4(0.0, 0.62890625, 0.0703125, 1.0),
	vec4(0.0, 0.76953125, 0.0859375, 1.0),
	vec4(0.0625, 0.30078125, 0.0, 1.0),
	vec4(0.08984375, 0.42578125, 0.0, 1.0),
	vec4(0.109375, 0.5625, 0.0, 1.0),
	vec4(0.171875, 0.6875, 0.0, 1.0),
	vec4(0.03125, 0.41796875, 0.0, 1.0),
	vec4(0.05078125, 0.51171875, 0.0, 1.0),
	vec4(0.07421875, 0.5625, 0.0, 1.0),
	vec4(0.0859375, 0.66796875, 0.0, 1.0),
	vec4(0.203125, 0.30078125, 0.04296875, 1.0),
	vec4(0.2890625, 0.42578125, 0.0625, 1.0),
	vec4(0.3515625, 0.50390625, 0.07421875, 1.0),
	vec4(0.5390625, 0.7109375, 0.11328125, 1.0),
	vec4(0.265625, 0.484375, 0.0, 1.0),
	vec4(0.31640625, 0.578125, 0.0, 1.0),
	vec4(0.3828125, 0.6796875, 0.0, 1.0),
	vec4(0.47265625, 0.8125, 0.0, 1.0),
	vec4(0.08984375, 0.31640625, 0.1015625, 1.0),
	vec4(0.1171875, 0.41015625, 0.1328125, 1.0),
	vec4(0.140625, 0.484375, 0.1640625, 1.0),
	vec4(0.2109375, 0.703125, 0.25, 1.0),
	vec4(0.10546875, 0.4453125, 0.078125, 1.0),
	vec4(0.1328125, 0.55078125, 0.1015625, 1.0),
	vec4(0.1640625, 0.64453125, 0.12109375, 1.0),
	vec4(0.19140625, 0.77734375, 0.15234375, 1.0),
	vec4(0.08984375, 0.359375, 0.0, 1.0),
	vec4(0.140625, 0.50390625, 0.0, 1.0),
	vec4(0.171875, 0.5859375, 0.0, 1.0),
	vec4(0.234375, 0.74609375, 0.0, 1.0),
	vec4(0.0, 0.328125, 0.01953125, 1.0),
	vec4(0.0, 0.4765625, 0.03125, 1.0),
	vec4(0.0, 0.578125, 0.0546875, 1.0),
	vec4(0.0, 0.6953125, 0.078125, 1.0),
	vec4(0.53515625, 0.15234375, 0.06640625, 1.0),
	vec4(0.65234375, 0.1875, 0.08203125, 1.0),
	vec4(0.828125, 0.23828125, 0.10546875, 1.0),
	vec4(1.0, 0.2890625, 0.12890625, 1.0),
	vec4(0.578125, 0.0, 0.0, 1.0),
	vec4(0.7109375, 0.0, 0.0, 1.0),
	vec4(0.8359375, 0.0, 0.0, 1.0),
	vec4(0.89453125, 0.0, 0.0, 1.0),
	vec4(0.578125, 0.46875, 0.0, 1.0),
	vec4(0.6796875, 0.57421875, 0.0, 1.0),
	vec4(0.75390625, 0.65234375, 0.0, 1.0),
	vec4(0.78515625, 0.69140625, 0.0, 1.0),
	vec4(0.578125, 0.25, 0.0, 1.0),
	vec4(0.62890625, 0.26953125, 0.0, 1.0),
	vec4(0.71875, 0.3203125, 0.0, 1.0),
	vec4(0.86328125, 0.40234375, 0.0, 1.0),
	vec4(0.3515625, 0.25, 0.0, 1.0),
	vec4(0.46875, 0.3359375, 0.0, 1.0),
	vec4(0.5625, 0.390625, 0.0, 1.0),
	vec4(0.62890625, 0.4609375, 0.0, 1.0),
	vec4(0.26171875, 0.16015625, 0.0, 1.0),
	vec4(0.3671875, 0.21875, 0.0, 1.0),
	vec4(0.4609375, 0.265625, 0.0, 1.0),
	vec4(0.59375, 0.3359375, 0.0, 1.0),
	vec4(0.31640625, 0.203125, 0.1015625, 1.0),
	vec4(0.41015625, 0.26953125, 0.1328125, 1.0),
	vec4(0.4765625, 0.31640625, 0.15625, 1.0),
	vec4(0.5859375, 0.39453125, 0.1953125, 1.0),
	vec4(0.484375, 0.33203125, 0.21484375, 1.0),
	vec4(0.5859375, 0.40625, 0.26171875, 1.0),
	vec4(0.75390625, 0.53125, 0.33984375, 1.0),
	vec4(0.90234375, 0.64453125, 0.41015625, 1.0),
	vec4(0.16796875, 0.13671875, 0.0859375, 1.0),
	vec4(0.234375, 0.1953125, 0.12109375, 1.0),
	vec4(0.29296875, 0.24609375, 0.15625, 1.0),
	vec4(0.39453125, 0.3359375, 0.21484375, 1.0),
	vec4(0.41796875, 0.2890625, 0.0, 1.0),
	vec4(0.51171875, 0.36328125, 0.0, 1.0),
	vec4(0.62109375, 0.453125, 0.0, 1.0),
	vec4(0.74609375, 0.55859375, 0.0, 1.0),
	vec4(0.2109375, 0.1171875, 0.1015625, 1.0),
	vec4(0.28515625, 0.1640625, 0.140625, 1.0),
	vec4(0.359375, 0.2109375, 0.1796875, 1.0),
	vec4(0.4453125, 0.265625, 0.22265625, 1.0),
	vec4(0.3359375, 0.1640625, 0.0859375, 1.0),
	vec4(0.41015625, 0.203125, 0.10546875, 1.0),
	vec4(0.55078125, 0.27734375, 0.14453125, 1.0),
	vec4(0.703125, 0.3671875, 0.18359375, 1.0),
	vec4(0.26953125, 0.234375, 0.19140625, 1.0),
	vec4(0.30859375, 0.2734375, 0.22265625, 1.0),
	vec4(0.3515625, 0.3125, 0.25390625, 1.0),
	vec4(0.41796875, 0.375, 0.3046875, 1.0),
	vec4(0.4765625, 0.43359375, 0.3515625, 1.0),
	vec4(0.5625, 0.51953125, 0.41796875, 1.0),
	vec4(0.703125, 0.65234375, 0.5234375, 1.0),
	vec4(0.8203125, 0.76171875, 0.61328125, 1.0),
	vec4(0.703125, 0.609375, 0.359375, 1.0),
	vec4(0.75390625, 0.65625, 0.4140625, 1.0),
	vec4(0.04296875, 0.04296875, 0.04296875, 1.0),
	vec4(0.09375, 0.07421875, 0.06640625, 1.0),
	vec4(0.125, 0.125, 0.125, 1.0),
	vec4(0.19140625, 0.16015625, 0.1328125, 1.0),
	vec4(0.2109375, 0.1796875, 0.1484375, 1.0),
	vec4(0.2265625, 0.1953125, 0.16015625, 1.0),
	vec4(0.5625, 0.515625, 0.109375, 1.0),
	vec4(0.6796875, 0.625, 0.1328125, 1.0),
	vec4(0.84375, 0.78515625, 0.1640625, 1.0),
	vec4(0.86328125, 0.8046875, 0.16796875, 1.0),
	vec4(0.609375, 0.55859375, 0.0, 1.0),
	vec4(0.703125, 0.64453125, 0.0, 1.0),
	vec4(0.796875, 0.73046875, 0.0, 1.0),
	vec4(0.88671875, 0.82421875, 0.0, 1.0),
	vec4(0.4765625, 0.41015625, 0.15625, 1.0),
	vec4(0.59375, 0.51171875, 0.19921875, 1.0),
	vec4(0.76953125, 0.67578125, 0.26171875, 1.0),
	vec4(0.9375, 0.85546875, 0.32421875, 1.0),
	vec4(0.4453125, 0.54296875, 0.0625, 1.0),
	vec4(0.55078125, 0.66015625, 0.078125, 1.0),
	vec4(0.69140625, 0.8125, 0.09765625, 1.0),
	vec4(0.86328125, 1.0, 0.12109375, 1.0),
	vec4(0.3515625, 0.25, 0.0, 1.0),
	vec4(0.46875, 0.3359375, 0.0, 1.0),
	vec4(0.5625, 0.390625, 0.0, 1.0),
	vec4(0.62890625, 0.4609375, 0.0, 1.0),
	vec4(0.26171875, 0.16015625, 0.0, 1.0),
	vec4(0.3671875, 0.21875, 0.0, 1.0),
	vec4(0.4609375, 0.265625, 0.0, 1.0),
	vec4(0.59375, 0.3359375, 0.0, 1.0),
	vec4(0.31640625, 0.203125, 0.1015625, 1.0),
	vec4(0.41015625, 0.26953125, 0.1328125, 1.0),
	vec4(0.4765625, 0.31640625, 0.15625, 1.0),
	vec4(0.5859375, 0.39453125, 0.1953125, 1.0),
	vec4(0.484375, 0.33203125, 0.21484375, 1.0),
	vec4(0.5859375, 0.40625, 0.26171875, 1.0),
	vec4(0.75390625, 0.53125, 0.33984375, 1.0),
	vec4(0.90234375, 0.64453125, 0.41015625, 1.0),
	vec4(0.16796875, 0.13671875, 0.0859375, 1.0),
	vec4(0.234375, 0.1953125, 0.12109375, 1.0),
	vec4(0.29296875, 0.24609375, 0.15625, 1.0),
	vec4(0.39453125, 0.3359375, 0.21484375, 1.0),
	vec4(0.41796875, 0.2890625, 0.0, 1.0),
	vec4(0.51171875, 0.36328125, 0.0, 1.0),
	vec4(0.62109375, 0.453125, 0.0, 1.0),
	vec4(0.74609375, 0.55859375, 0.0, 1.0),
	vec4(0.2109375, 0.1171875, 0.1015625, 1.0),
	vec4(0.28515625, 0.1640625, 0.140625, 1.0),
	vec4(0.359375, 0.2109375, 0.1796875, 1.0),
	vec4(0.4453125, 0.265625, 0.22265625, 1.0),
	vec4(0.3359375, 0.1640625, 0.0859375, 1.0),
	vec4(0.41015625, 0.203125, 0.10546875, 1.0),
	vec4(0.55078125, 0.27734375, 0.14453125, 1.0),
	vec4(0.703125, 0.3671875, 0.18359375, 1.0),
	vec4(0.19921875, 0.19921875, 0.19921875, 1.0),
	vec4(0.56640625, 0.21484375, 0.62109375, 1.0),
	vec4(0.18359375, 0.19140625, 0.62890625, 1.0),
	vec4(0.23046875, 0.53515625, 0.5078125, 1.0),
	vec4(0.09765625, 0.51171875, 0.09765625, 1.0),
	vec4(0.77734375, 0.6328125, 0.0, 1.0),
	vec4(0.828125, 0.35546875, 0.0, 1.0),
	vec4(0.66796875, 0.15625, 0.15625, 1.0),
	vec4(0.16015625, 0.16015625, 0.16015625, 1.0),
	vec4(0.40625, 0.13671875, 0.4609375, 1.0),
	vec4(0.296875, 0.28125, 0.6953125, 1.0),
	vec4(0.0, 0.51171875, 0.4765625, 1.0),
	vec4(0.08203125, 0.4453125, 0.08203125, 1.0),
	vec4(0.7109375, 0.58984375, 0.0, 1.0),
	vec4(0.62890625, 0.26953125, 0.0, 1.0),
	vec4(0.4921875, 0.1015625, 0.1015625, 1.0),
	vec4(0.1171875, 0.1171875, 0.1171875, 1.0),
	vec4(0.49609375, 0.14453125, 0.578125, 1.0),
	vec4(0.2421875, 0.23046875, 0.5703125, 1.0),
	vec4(0.390625, 0.54296875, 0.5234375, 1.0),
	vec4(0.09375, 0.3671875, 0.08984375, 1.0),
	vec4(0.8046875, 0.68359375, 0.14453125, 1.0),
	vec4(0.7109375, 0.3515625, 0.09765625, 1.0),
	vec4(0.5703125, 0.125, 0.125, 1.0),
	vec4(0.2109375, 0.1171875, 0.1015625, 1.0),
	vec4(0.28515625, 0.1640625, 0.140625, 1.0),
	vec4(0.359375, 0.2109375, 0.1796875, 1.0),
	vec4(0.4453125, 0.265625, 0.22265625, 1.0),
	vec4(0.2421875, 0.23046875, 0.5703125, 1.0),
	vec4(0.296875, 0.28125, 0.6953125, 1.0),
	vec4(0.34765625, 0.32421875, 0.796875, 1.0),
	vec4(0.4453125, 0.40625, 1.0, 1.0),
	vec4(0.47265625, 0.58984375, 0.65234375, 1.0),
	vec4(0.51171875, 0.63671875, 0.703125, 1.0),
	vec4(0.5546875, 0.6953125, 0.76171875, 1.0),
	vec4(0.59765625, 0.75, 0.8203125, 1.0),
	vec4(0.20703125, 0.4375, 0.53515625, 1.0),
	vec4(0.2421875, 0.5078125, 0.62109375, 1.0),
	vec4(0.29296875, 0.6171875, 0.74609375, 0.5),
	vec4(0.32421875, 0.6875, 0.8203125, 1.0),
	vec4(0.25, 0.25, 0.25, 1.0),
	vec4(0.87890625, 0.0, 1.0, 1.0),
	vec4(0.0, 0.03125, 1.0, 1.0),
	vec4(0.0, 1.0, 0.9609375, 1.0),
	vec4(0.01171875, 0.8203125, 0.0, 1.0),
	vec4(0.87109375, 0.72265625, 0.0, 1.0),
	vec4(0.828125, 0.35546875, 0.0, 1.0),
	vec4(0.87109375, 0.0, 0.0, 1.0),
	vec4(0.25, 0.25, 0.25, 1.0),
	vec4(0.87890625, 0.0, 1.0, 1.0),
	vec4(0.0, 0.03125, 1.0, 1.0),
	vec4(0.0, 1.0, 0.9609375, 1.0),
	vec4(0.01171875, 0.8203125, 0.0, 1.0),
	vec4(0.87109375, 0.72265625, 0.0, 1.0),
	vec4(0.828125, 0.35546875, 0.0, 1.0),
	vec4(0.87109375, 0.0, 0.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 1.0),
	vec4(1.0, 1.0, 1.0, 1.0),
	vec4(0.0, 0.0, 0.0, 0.0)
);

const vec3 norm_lut[6] = vec3[6](
    vec3(1,0,0),
    vec3(-1,0,0),
    vec3(0,1,0),
    vec3(0,-1,0),
    vec3(0,0,1),
    vec3(0,0,-1)
);

struct Material {
    float smoothness;
    float metalness;
    float reflectance;
    float color_variance;
    float color_variance_scale;
};

const Material mat_lut[15] = Material[15](
    Material( //GlossySmooth
        0.9,
        0.0,
        0.2,
        0.0,
        1.0
    ),
    Material( //GlossyRough
        0.9,
        0.0,
        0.2,
        0.0,
        1.0
    ),
    Material( //MatteSmooth
        0.2,
        0.0,
        0.2,
        0.0,
        1.0
    ),
    Material( //MatteRough
        0.2,
        0.0,
        0.1,
        0.0,
        1.0
    ),
    Material( //MetallicSmooth
        0.8,
        1.0,
        1.0,
        0.0,
        1.0
    ),
    Material( //MetallicRough
        0.92,
        1.0,
        1.0,
        0.0,
        1.0
    ),
    Material( //Snow
        0.35,
        0.0,
        1.0,
        0.0,
        0.5
    ),
    Material( //Stone
        0.15,
        0.0,
        0.2,
        0.0,
        1.0
    ),
    Material( //Grass
        0.15,
        0.0,
        0.2,
        0.25,
        0.75
    ),
    Material( //Leaves
        0.3,
        0.0,
        0.2,
        0.0,
        1.0
    ),
    Material( //Earth
        0.15,
        0.0,
        0.2,
        0.0,
        1.0
    ),
    Material( //Log
        0.2,
        0.0,
        0.2,
        0.0,
        1.0
    ),
    Material( //Sand
        0.4,
        0.0,
        0.5,
        0.2,
        0.5
    ),
    Material( //Water
        0.95,
        0.0,
        0.0,
        0.0,
        1.0
    ),
    Material( //Empty
        0.3,
        0.0,
        0.2,
        0.0,
        1.0
    )
);

# What would a voxel look like?

## just start with perlin noise

these are the simple qualities I would want my voxels to have. Let's go over them one by one.
```
struct Voxel {
	position: vec3[f32; 3],
	normal: f32(?),
	isTransparent: bool,
	texture: Texture,
	isGravityBlock: bool,
}
```

- `position` would be its x, y and z coordinates within the chunk.
- `normal` would be where it's facing.
- `isTransparent` would allow for things like transparent blocks, such as glass.
- `texture`: All blocks would require a texture
- `isGravityBlock` means: Will the block fall if you break blocks underneath it, similar to sand in minecraft?

Now that we have the basis for a voxel, let's look at a chunk:

```

struct Chunk {
	voxels: Voxel,
	position: Vec3[f32; 3],
}
```

This is something that needs further thought. I am thinking of storing 16x16x16 voxels per chunk?

```
struct Chunks {
	chunk: Chunk,
}
```

this would encompass the chunk in a 3d array of chunks. However, this does put into question: How do I store voxel data? Using a SVO probably isn't effective. maybe i could look into doing brickmaps?  

----
I'm thinking for scale, the player would be about 3 voxels or 2 voxels tall.


I also would want to add in stuff such as dynamic blocks like water, leaves blowing in the wind etc


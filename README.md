# this is my voxel engine: Dioxane.

-Ultimately this engine's goal is to be able to host .vxl maps, and set up the infrastructure needed for huge voxel maps that can allow real time destruction/modification.
- this engine should support multiplayer as well as languages like lua for user created content

## TODO: 

- GPU: Use rasterization
- use brickmapping for voxel data structure

- something i do wonder is if i want to use bevy, and if so, there would be very ltitle optimizations i would have to do for the cube rendering. Maybe implement greedy meshing as it's not currently implemented, of course.

## Pipeline: 

Buffer -> Input assembles 
-> Vertex Shader -> Rasterization -> Frag. Shader
-> Color Blending - Frame Buffer
pub fn gvox() {
    let gvox_ctx = gvox_rs::Context::new();

    let scene = gvox_ctx
        .load_from_raw(
            std::path::Path::new("Normandy.vxl"),
            &String::from("ace_of_spades"),
        )
        .unwrap();

    let node_i = 0;
    let node = unsafe { *(scene.nodes.add(node_i)) };

    let xi = 0;
    let yi = 0;
    let zi = 0;

    let voxel_i = xi + yi * node.size_x + zi * node.size_x * node.size_y;

    let voxel = unsafe { *(node.voxels.add(voxel_i)) };
}

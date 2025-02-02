for p in art/*.blend;
do
    filename=$(basename -- "$p")
    echo $filename
    filename="${filename%.*}"
    out="assets/models/$filename.glb"
    mkdir -p assets/models
    if [[ "$p" -nt "$out" ]] && [[ $filename != "basic_material" ]];
    then
        blender -b "$p" --python-expr "import bpy; bpy.ops.export_scene.gltf(filepath='$out', export_apply=True, export_materials='NONE', export_extras=True)"
        gltf-transform dedup $out $out
    fi
done

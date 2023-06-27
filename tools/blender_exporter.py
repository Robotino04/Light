import bpy
import os
import math
import mathutils

from bpy.props import StringProperty, BoolProperty
from bpy_extras.io_utils import ExportHelper
from bpy.types import Operator

def print_meta(type):
    def f(*data):
        for window in bpy.context.window_manager.windows:
            screen = window.screen
            for area in screen.areas:
                if area.type == 'CONSOLE':
                    override = {'window': window, 'screen': screen, 'area': area}
                    bpy.ops.console.scrollback_append(override, text=" ".join(list(map(str, data))), type=type)
    return f

print_output = print_meta("OUTPUT")
print_error = print_meta("ERROR")
print_input = print_meta("INPUT")
print_info = print_meta("INFO")
print = print_output


class ExportOperator(bpy.types.Operator, ExportHelper):
    """Operator for setting a directory path."""

    #: Name of function for calling the nif export operator.
    bl_idname = "export_scene.folder"

    #: How the nif import operator is labelled in the user interface.
    bl_label = "Export to folder"

    filename_ext = ".toml"

    def execute(self, context):
        userpath = self.properties.filepath

        #Insert the desired logic here to write to the directory.
        print(userpath)

        return{'FINISHED'}

def register():
    bpy.utils.register_class(ExportOperator)

def unregister():
    bpy.utils.unregister_class(ExportOperator)

def change_coord_system(vec):
    return [vec[0], vec[2], -vec[1]]
def change_coord_system_rot(vec):
    return [vec[0], vec[1], vec[2]]

def stringify_vec(vec):
     return ";".join(list(map(str, vec)))

def export_frame(frame=None):
    if frame != None:
        bpy.data.scenes["Scene"].frame_current = frame
    with open(f"/tmp/blender_export{frame if frame != None else ''}.toml", "w") as f:
        def print_and_write(*data):
            nonlocal f
            print_info(*data)
            f.write(" ".join(list(map(str, [*data]))) + "\n")
        
        bpy.ops.object.select_all(action='DESELECT')
        
        print((f"Frame {frame}" if frame != None else "Scene").center(50, "="))

        for obj in [ob for ob in bpy.context.view_layer.objects if ob.visible_get()]:
            if obj.type == "MESH":
                obj_path = f"/tmp/blender_export_{obj.name}{frame}.obj"
                print_and_write("[mesh]")
                obj.select_set(True)
                bpy.ops.wm.obj_export(
                    filepath=obj_path,
                    export_selected_objects=True,
                    export_triangulated_mesh=True,
                    export_materials=False,
                    export_smooth_groups=True,
                )
                obj.select_set(False)
                
                ## Mesh objects don't need any position/rotation/scale. That gets encoded in the obj file.
                print_and_write("mesh_file = " + obj_path)
                shader = obj.data.materials[0].node_tree.nodes["Principled BSDF"]
                if (shader.inputs[6].default_value == 0):
                    print_and_write("material_type = diffuse_material")
                    print_and_write("albedo =", stringify_vec(shader.inputs[0].default_value[0:3]))
                else:
                    print_and_write("material_type = metallic_material")
                    print_and_write("albedo =", stringify_vec(shader.inputs[0].default_value[0:3]))
                    print_and_write("roughness =", shader.inputs[9].default_value)
                
                if (any(shader.inputs[19].default_value[0:3])):
                    print_and_write("material_type = emissive_material")
                    print_and_write("emission_color =", stringify_vec(shader.inputs[19].default_value[0:3]))
                    print_and_write("strength =", shader.inputs[20].default_value)
                if (shader.inputs[17].default_value):
                    print_and_write("material_type = dielectric_material")
                    print_and_write("albedo =", stringify_vec(shader.inputs[0].default_value[0:3]))
                    print_and_write("ior =", shader.inputs[16].default_value)
                
            elif obj.type == "CAMERA":
                vec = mathutils.Vector((0.0, 0.0,-1.0))
                inv = obj.matrix_world.copy()
                inv.invert()
                vec_rot = vec @ inv
                target = obj.location + vec_rot
                
                print_and_write("[camera]")
                print_and_write("fov =", math.degrees(obj.data.angle_x))
                print_and_write("position =", stringify_vec(change_coord_system(obj.location)))
                print_and_write("target =", stringify_vec(change_coord_system(target)))
                print_and_write("width =", bpy.data.scenes[0].render.resolution_x)
                print_and_write("height =", bpy.data.scenes[0].render.resolution_y)
            else: continue
                
    #        print("scale =", list(obj.scale))
                
        print("="*50)


if __name__ == "__main__":
    register()
    for i in range(bpy.data.scenes["Scene"].frame_start, bpy.data.scenes["Scene"].frame_end+1):
        export_frame(i)

    # test call
#    print(bpy.ops.export_scene.folder('INVOKE_DEFAULT'))

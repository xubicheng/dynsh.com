import bpy
import os

# 清默认场景
bpy.ops.object.select_all(action='SELECT')
bpy.ops.object.delete(use_global=False)

# 创建 2D 场景
scene = bpy.context.scene
scene.render.engine = 'BLENDER_EEVEE'
scene.render.resolution_x = 1440
scene.render.resolution_y = 1000
scene.render.resolution_percentage = 100
scene.render.image_settings.file_format = 'PNG'
scene.render.image_settings.color_mode = 'RGBA'
scene.frame_start = 1
scene.frame_end = 60
scene.render.filepath = "/tmp/demo_frames/frame_####"

# 关掉 3D 不需要的东西
scene.eevee.use_bloom = False
scene.eevee.use_ssr = False
scene.render.film_transparent = True  # 2D 动画常用透明背景

# 添加一个简单平面（代表阿布角色占位）
bpy.ops.mesh.primitive_plane_add(size=2, location=(0, 0, 0))
obj = bpy.context.active_object
obj.name = "abu_placeholder"
# 缩放变窄 (x=1.2, y=0.3) 代表横版 2D 角色
obj.scale = (1.2, 0.3, 1)

# 给它一个材质（扁平色: 浅蓝 #5d8aa8 — wayhug 品牌蓝）
mat = bpy.data.materials.new(name="flat_color")
mat.use_nodes = False
mat.diffuse_color = (0.365, 0.541, 0.659, 1.0)  # #5d8aa8 RGBA
obj.data.materials.append(mat)

# 加关键帧动画: 从左到右滑过 (x: -5 → 5)
obj.location = (-5, 0, 0)
obj.keyframe_insert(data_path="location", frame=1)
obj.location = (5, 0, 0)
obj.keyframe_insert(data_path="location", frame=60)
# 简单线性插值
for f in range(1, 61):
    scene.frame_set(f)

# 摄像机对齐到 2D 俯视/侧视（XZ 平面）
cam_data = bpy.data.cameras.new("cam")
cam_obj = bpy.data.objects.new("Camera", cam_data)
cam_obj.location = (0, -8, 0)
cam_obj.rotation_euler = (1.5708, 0, 0)  # 90° 俯视
scene.camera = cam_obj

# 输出 blend 文件
os.makedirs("/tmp", exist_ok=True)
bpy.ops.wm.save_as_mainfile(filepath="/tmp/demo.blend")
print(f"✅ demo.blend 已创建 (frame 1-60, 1440×1000)")

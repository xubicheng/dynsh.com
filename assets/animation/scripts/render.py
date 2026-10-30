#!/usr/bin/env python3
"""
render.py — Blender headless 渲染入口
用法: blender --background abu.blend --python render.py -- --out frames/001/ --start 1 --end 60

或直接调: python3 render.py abu.blend frames/001/ 1 60 --res 1440 1000

MIT License — part of dynsh.com 大隐士 project
"""

import sys
import os

# Blender 内部运行会注入 bpy
try:
    import bpy
    IN_BLENDER = True
except ImportError:
    IN_BLENDER = False

RES_X, RES_Y = 1440, 1000
FORMAT = "PNG"  # PNG / EXR / JPEG

def setup_scene(res_x=RES_X, res_y=RES_Y, frame_start=1, frame_end=60, out_dir="frames"):
    scene = bpy.context.scene
    # 分辨率
    scene.render.resolution_x = res_x
    scene.render.resolution_y = res_y
    scene.render.resolution_percentage = 100
    # 2D 色彩: sRGB，256 色目标 → PNG 8-bit + 调色板
    scene.render.image_settings.file_format = FORMAT
    scene.render.image_settings.color_mode = 'RGBA'
    # 帧范围
    scene.frame_start = frame_start
    scene.frame_end = frame_end
    # 输出路径
    os.makedirs(out_dir, exist_ok=True)
    scene.render.filepath = os.path.join(out_dir, "frame_####")
    # 引擎: 2D 动画用 EEVEE 快，复杂用 CYCLES
    scene.render.engine = 'BLENDER_EEVEE'
    scene.eevee.use_bloom = False       # 干净平涂色
    scene.eevee.use_ssr = False         # 无反射
    scene.render.use_border = False
    print(f"[render] {res_x}x{res_y}, frames {frame_start}-{frame_end}, out={out_dir}")

if IN_BLENDER:
    # Blender 内部: 解析 -- --out --start --end
    args = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    out_dir = args[0] if len(args) > 0 else "frames"
    frame_start = int(args[1]) if len(args) > 1 else 1
    frame_end = int(args[2]) if len(args) > 2 else 60
    res_x = int(args[3]) if len(args) > 3 else RES_X
    res_y = int(args[4]) if len(args) > 4 else RES_Y
    setup_scene(res_x, res_y, frame_start, frame_end, out_dir)
    bpy.ops.render.render(animation=True)
    print("[render] DONE")
else:
    # 外部调用壳
    print("""render.py — Blender headless 渲染脚本
请在 blender --background 模式下调用，或使用 render.sh 包装器。""")

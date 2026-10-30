#!/usr/bin/env python3
"""
render.py — Blender headless 渲染入口 (dynsh.com 256 色动画)

用法: blender --background file.blend --python render.py -- <out_dir> <start> <end> [res_x] [res_y]

优化 (2D 扁平色动画专属):
  - taa_render_samples = 2        ← 默认 64, 2D 无噪点场景用 2 即可
  - use_fast_gi = False            ← 关掉全局光照 (2D 不需要)
  - use_shadows = False            ← 关掉阴影
  - use_taa_reprojection = False
  - volumetric_samples = 1
  - 2D 材质强制 use_nodes = False
  - film_transparent = True

MIT License — part of dynsh.com 大隐士 project
"""
import sys, os

try:
    import bpy
    IN_BLENDER = True
except ImportError:
    IN_BLENDER = False

RES_X, RES_Y = 1440, 1000

def setup_scene(res_x=RES_X, res_y=RES_Y, frame_start=1, frame_end=60, out_dir="frames"):
    scene = bpy.context.scene
    scene.render.resolution_x = res_x
    scene.render.resolution_y = res_y
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    scene.render.image_settings.color_mode = 'RGBA'
    scene.render.film_transparent = True
    scene.frame_start = frame_start
    scene.frame_end = frame_end
    os.makedirs(out_dir, exist_ok=True)
    scene.render.filepath = os.path.join(out_dir, "frame_####")
    scene.render.engine = 'BLENDER_EEVEE'
    ev = scene.eevee
    ev.taa_render_samples = 2
    ev.taa_samples = 2
    ev.use_fast_gi = False
    ev.use_shadows = False
    ev.use_taa_reprojection = False
    ev.volumetric_samples = 1
    # Blender 5.x 去掉了 use_bloom/use_ssr (try/except 兼容)
    for attr in ['use_bloom', 'use_ssr', 'use_ssgi']:
        if hasattr(ev, attr):
            try: setattr(ev, attr, False)
            except: pass
    # 材质强制扁平色
    for mat in bpy.data.materials:
        try: mat.use_nodes = False
        except: pass
    print(f"[setup] {res_x}x{res_y}, frames {frame_start}-{frame_end}, samples=2")

if IN_BLENDER:
    args = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
    out_dir     = args[0] if len(args) > 0 else "frames"
    frame_start = int(args[1]) if len(args) > 1 else 1
    frame_end   = int(args[2]) if len(args) > 2 else 60
    res_x       = int(args[3]) if len(args) > 3 else RES_X
    res_y       = int(args[4]) if len(args) > 4 else RES_Y
    setup_scene(res_x, res_y, frame_start, frame_end, out_dir)
    print(f"[render] headless → {out_dir} ({frame_end - frame_start + 1} frames)")
    bpy.ops.render.render(animation=True)
    print("[render] DONE")
else:
    print("render.py — Blender headless 脚本, 需 blender --background 调用")

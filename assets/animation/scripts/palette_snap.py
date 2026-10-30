#!/usr/bin/env python3
"""
palette_snap.py — dynsh.com 256 色调色板对齐工具
=================================================
来源: Rust 端 src/palette.rs（6×6×6 色立方 + 40 级灰阶 = 256 色）

用途:
  1. 把任意 hex 颜色 snap 到固定 256 色表（与 palette.rs 完全一致）
  2. Blender 材质创建时用这个工具确保色彩一致
  3. ffmpeg 合成 GIF/WebM 前可做帧级量化（可选）

用法:
  python3 palette_snap.py #5d8aa8        # 单色查询 → 打印 #cc9966
  python3 palette_snap.py --blender demo.blend   # Blender 文件内所有材质 snap
  python3 palette_snap.py --list         # 打印完整 256 色表
  python3 palette_snap.py --validate assets/animation/frames/*.png  # 帧是否用了 256 色

MIT License — part of dynsh.com 大隐士 project
"""

import sys
import os
import re

# ============================================================
# 与 Rust palette.rs 完全一致的实现
# ============================================================

LEVELS = [0, 51, 102, 153, 204, 255]
SIZE = 256

def rgb(index: int):
    """取第 i 个颜色的 (r, g, b)"""
    if index < 216:
        return (LEVELS[index // 36],
                LEVELS[(index % 36) // 6],
                LEVELS[index % 6])
    else:
        level = ((index - 216) * 255 // 39)
        return (level, level, level)

def hex_of(index: int) -> str:
    r, g, b = rgb(index)
    return f"#{r:02x}{g:02x}{b:02x}"

def nearest(r: int, g: int, b: int) -> int:
    """找最接近的调色板索引（确定性: 距离相同时取较小索引）"""
    best_i = 0
    best_d = float('inf')
    for i in range(SIZE):
        pr, pg, pb = rgb(i)
        dr = r - pr
        dg = g - pg
        db = b - pb
        d = dr*dr + dg*dg + db*db
        if d < best_d:
            best_d = d
            best_i = i
    return best_i

def snap(hex_color: str) -> str:
    """任意 hex → 贴到 256 色表的 hex"""
    s = hex_color.strip().lstrip('#')
    try:
        if len(s) == 6:
            r, g, b = int(s[0:2], 16), int(s[2:4], 16), int(s[4:6], 16)
        elif len(s) == 3:
            r = int(s[0:1], 16) * 17
            g = int(s[1:2], 16) * 17
            b = int(s[2:3], 16) * 17
        else:
            return "#000000"
    except ValueError:
        return "#000000"
    idx = nearest(r, g, b)
    return hex_of(idx)

def snap_with_info(hex_color: str):
    """返回 (输入hex, 输出hex, 调色板索引, 色差)"""
    s = hex_color.strip().lstrip('#')
    try:
        if len(s) == 6:
            r, g, b = int(s[0:2], 16), int(s[2:4], 16), int(s[4:6], 16)
        elif len(s) == 3:
            r = int(s[0:1], 16) * 17
            g = int(s[1:2], 16) * 17
            b = int(s[2:3], 16) * 17
        else:
            return (hex_color, "#000000", 0, 0)
    except ValueError:
        return (hex_color, "#000000", 0, 0)
    idx = nearest(r, g, b)
    pr, pg, pb = rgb(idx)
    dr, dg, db = r - pr, g - pg, b - pb
    dist = (dr*dr + dg*dg + db*db) ** 0.5
    return (hex_color, hex_of(idx), idx, dist)

# ============================================================
# Blender 材质色 snap（可选模块）
# ============================================================

def blender_snap(blend_path: str):
    """在 Blender 内运行，把所有材质 diffuse_color 贴到 256 色表"""
    try:
        import bpy
    except ImportError:
        print("⚠️  blender_snap 必须在 Blender 内部运行!")
        print("   用法: blender --background file.blend --python palette_snap.py -- --blender")
        return

    count = 0
    for mat in bpy.data.materials:
        if mat.use_nodes:
            # 尝试从 Principled BSDF 取 Base Color
            if mat.node_tree:
                for node in mat.node_tree.nodes:
                    if node.type == 'BSDF_PRINCIPLED':
                        color = node.inputs['Base Color'].default_value
                        r8 = int(round(color[0] * 255))
                        g8 = int(round(color[1] * 255))
                        b8 = int(round(color[2] * 255))
                        idx = nearest(r8, g8, b8)
                        pr, pg, pb = rgb(idx)
                        node.inputs['Base Color'].default_value = (pr/255, pg/255, pb/255, color[3])
                        count += 1
                        print(f"  [node] {mat.name}: #{r8:02x}{g8:02x}{b8:02x} → {hex_of(idx)}")
        else:
            # Legacy diffuse_color
            color = mat.diffuse_color
            r8 = int(round(color[0] * 255))
            g8 = int(round(color[1] * 255))
            b8 = int(round(color[2] * 255))
            idx = nearest(r8, g8, b8)
            pr, pg, pb = rgb(idx)
            mat.diffuse_color = (pr/255, pg/255, pb/255, color[3])
            count += 1
            print(f"  [legacy] {mat.name}: #{r8:02x}{g8:02x}{b8:02x} → {hex_of(idx)}")

    print(f"✅ snap 了 {count} 个材质到 256 色表")
    bpy.ops.wm.save_as_mainfile(filepath=blend_path)
    print(f"✅ 已保存 {blend_path}")

# ============================================================
# 命令行
# ============================================================

def main():
    args = sys.argv[1:]

    if not args or args[0] in ('-h', '--help'):
        print(__doc__)
        return

    if args[0] == '--list':
        # 打印完整 256 色表
        for i in range(SIZE):
            print(f"{i:3d}: {hex_of(i)}  rgb{rgb(i)}")
        return

    if args[0] == '--blender':
        # 在 Blender 内运行
        blend_path = args[1] if len(args) > 1 else None
        if blend_path:
            blender_snap(blend_path)
        else:
            blender_snap(bpy.data.filepath)
        return

    if args[0] == '--validate':
        import glob
        try:
            from PIL import Image
        except ImportError:
            print("⚠️  validate 需要 Pillow: pip install Pillow")
            return
        total_checked = 0
        total_violations = 0
        for pattern in args[1:]:
            for path in glob.glob(pattern):
                img = Image.open(path).convert('RGBA')
                pixels = list(img.getdata())
                unique = set()
                for px in pixels:
                    # 跳过完全透明
                    if px[3] == 0:
                        continue
                    unique.add((px[0], px[1], px[2]))
                violations = 0
                for (r, g, b) in unique:
                    idx = nearest(r, g, b)
                    pr, pg, pb = rgb(idx)
                    if (r, g, b) != (pr, pg, pb):
                        violations += 1
                total_checked += 1
                total_violations += violations
                print(f"  {os.path.basename(path)}: {len(unique)} 色, {violations} 个不在 256 色表")
        print(f"\n总计: {total_checked} 帧, {total_violations} 个非对齐颜色")
        return

    # 默认: 单色查询（支持多个）
    for hex_color in args:
        result = snap_with_info(hex_color)
        in_hex, out_hex, idx, dist = result
        tag = "✅" if dist < 1 else "❌需贴"
        print(f"  {tag} {in_hex} → {out_hex}  [palette #{idx}, Δ={dist:.1f}]")

if __name__ == '__main__':
    main()

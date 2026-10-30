#!/usr/bin/env python3
"""
pixel_snap.py — 把图像所有像素贴到 dynsh.com 256 色调色板
与 Rust palette.rs 完全一致的 nearest 算法

用法: python3 pixel_snap.py <input.png> <output.png>
      python3 pixel_snap.py --batch frames_dir/
      python3 pixel_snap.py --validate frames_dir/   # 只检查不修改

MIT License
"""
import sys, os, glob, argparse

# 导入 palette 模块
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from palette_snap import nearest, rgb as palette_rgb, SIZE

def snap_image(input_path: str, output_path: str = None) -> tuple:
    """单张图像 snap。返回 (输入路径, 像素数, 变化像素数)"""
    from PIL import Image
    img = Image.open(input_path).convert('RGBA')
    pixels = img.load()
    w, h = img.size
    total = w * h
    changed = 0

    for y in range(h):
        for x in range(w):
            r, g, b, a = pixels[x, y]
            if a == 0:
                continue
            idx = nearest(r, g, b)
            pr, pg, pb = palette_rgb(idx)
            if (r, g, b) != (pr, pg, pb):
                changed += 1
                pixels[x, y] = (pr, pg, pb, a)

    out = output_path or input_path
    if output_path:
        os.makedirs(os.path.dirname(output_path) or '.', exist_ok=True)
    img.save(out, 'PNG')
    return (input_path, total, changed)

def batch_snap(dir_path: str, backup: bool = True):
    """批量处理目录下所有 PNG"""
    files = sorted(glob.glob(os.path.join(dir_path, '*.png')))
    if not files:
        print(f"⚠️  {dir_path} 没找到 PNG")
        return

    if backup:
        bak = dir_path + '_original'
        os.makedirs(bak, exist_ok=True)
        print(f"📦 备份到 {bak}/")
        for f in files:
            base = os.path.basename(f)
            if not os.path.exists(os.path.join(bak, base)):
                os.rename(f, os.path.join(bak, base))
        files = sorted(glob.glob(os.path.join(bak, '*.png')))

    print(f"\n🎨 pixel_snap 处理 {len(files)} 张 → {dir_path}/")
    total_changed = 0
    for f in files:
        base = os.path.basename(f)
        out = os.path.join(dir_path, base)
        in_path, n_pixel, n_changed = snap_image(f, out)
        total_changed += n_changed
        pct = n_changed / n_pixel * 100 if n_pixel > 0 else 0
        print(f"  {base}: {n_pixel}px, {n_changed} 需贴 ({pct:.1f}%)")

    print(f"\n✅ 完成: 总计 {total_changed} 像素贴到 256 色表")

def validate_dir(dir_path: str):
    """只检查，不修改"""
    from PIL import Image
    files = sorted(glob.glob(os.path.join(dir_path, '*.png')))
    if not files:
        print(f"⚠️  {dir_path} 没找到 PNG")
        return
    print(f"\n🔍 验证 256 色合规性: {len(files)} 张")
    all_ok = True
    for f in files:
        img = Image.open(f).convert('RGBA')
        pixels = set()
        for px in img.getdata():
            if px[3] > 0:
                pixels.add((px[0], px[1], px[2]))
        bad = 0
        for (r, g, b) in pixels:
            idx = nearest(r, g, b)
            pr, pg, pb = palette_rgb(idx)
            if (r, g, b) != (pr, pg, pb):
                bad += 1
        status = "✅" if bad == 0 else "❌"
        if bad > 0: all_ok = False
        print(f"  {os.path.basename(f)}: {len(pixels)} 独特色, {bad} 个不在表 {status}")
    print(f"\n{'✅ 全部合规' if all_ok else '❌ 存在非对齐像素'}")

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='dynsh.com 256 色像素 snap')
    parser.add_argument('target', help='PNG 文件或目录')
    parser.add_argument('--batch', action='store_true', help='目录批量处理')
    parser.add_argument('--validate', action='store_true', help='只检查不修改')
    parser.add_argument('--no-backup', action='store_true', help='批量处理不备份原图')
    args = parser.parse_args()

    if args.validate:
        validate_dir(args.target)
    elif args.batch or os.path.isdir(args.target):
        batch_snap(args.target, backup=not args.no_backup)
    else:
        snap_image(args.target)

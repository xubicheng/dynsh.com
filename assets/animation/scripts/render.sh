#!/bin/bash
# render.sh — 一键渲染: ./render.sh abu.blend frames/001 1 120
# 然后自动 ffmpeg 合成为 webm + gif
# MIT License
set -euo pipefail

BLEND="${1:?usage: render.sh <file.blend> <out_dir> <start> <end> [res_x] [res_y]}"
OUT="${2:-frames}"
START="${3:-1}"
END="${4:-60}"
RX="${5:-1440}"
RY="${6:-1000}"

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(dirname "$SCRIPT_DIR")"
BLENDER="$(which blender 2>/dev/null || echo /snap/bin/blender)"

mkdir -p "$OUT"
echo "[1/3] Blender headless 渲染 $BLEND → $OUT ($RX×$RY, $START-$END)"
"$BLENDER" --background "$BLEND" --python "$SCRIPT_DIR/render.py" -- "$OUT" "$START" "$END" "$RX" "$RY" 2>&1 | tail -10

# 找到帧序列
FIRST_FRAME=$(ls "$OUT"/frame_*.png 2>/dev/null | sort | head -1)
if [ -z "$FIRST_FRAME" ]; then
    echo "❌ 没找到渲染出的帧！"
    exit 1
fi
FRAME_NAME=$(basename "$FIRST_FRAME")
# frame_0001.png → frame_%04d.png
PATTERN="${FRAME_NAME/_0001/_%04d}"

echo "[2/3] ffmpeg 合成 WebM (264) → $(basename "$OUT").webm"
ffmpeg -y -framerate 24 -i "$OUT/$PATTERN" \
    -c:v libvpx-vp9 -pix_fmt yuva420p -b:v 0 -crf 28 -auto-alt-ref 0 \
    "$ROOT/webm/$(basename "$OUT").webm" 2>&1 | tail -3

echo "[3/3] ffmpeg 合成 GIF → $(basename "$OUT").gif (12fps, 256 色)"
ffmpeg -y -framerate 12 -i "$OUT/$PATTERN" \
    -vf "palettegen=max_colors=256:reserve_transparent=on" /tmp/palette.png
ffmpeg -y -framerate 12 -i "$OUT/$PATTERN" -i /tmp/palette.png \
    -lavfi "paletteuse=dither=sierra2_4a" \
    "$ROOT/gif/$(basename "$OUT").gif" 2>&1 | tail -3

echo "✅ DONE"
echo "  WebM: $ROOT/webm/$(basename "$OUT").webm"
echo "  GIF:  $ROOT/gif/$(basename "$OUT").gif"
ls -lh "$ROOT/webm/$(basename "$OUT").webm" "$ROOT/gif/$(basename "$OUT").gif"

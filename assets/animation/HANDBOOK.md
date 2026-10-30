# dynsh.com 大隐士 · Blender 动画制作操作手册

> MIT License · 为消除隔阂而做 · 合和和波

---

## 一、工具链总览

| 工具 | 版本 | 用途 | 安装 |
|---|---|---|---|
| Blender | 5.2.2 LTS | 动画核心（建模/动画/渲染） | `sudo snap install blender --classic` |
| Inkscape | 1.1.2 | 矢量插画（角色设定/场景原画） | `sudo apt install inkscape` |
| Krita | 5.x | 数字绘画 | `sudo apt install krita` |
| ffmpeg | 4.4 | 帧序列 → WebM/GIF | 系统自带 |
| Python | 3.10 | 流水线脚本 | 系统自带 |

**核心原则：本地做 .blend → 服务器 headless 渲染 → ffmpeg 合成 → 网站前端引用**

服务器 2GB RAM 渲染速度约 55 秒/帧（1440×1000），**不适合本地 Blender 界面**（无显示器）。

---

## 二、目录结构

```
/opt/dynsh.com/assets/animation/
├── blend/                      # Blender 源文件（版本库）
│   ├── 001-abu-early/
│   │   └── demo.blend
│   └── 002-tiantai-mountain/
│       └── tiantai-he.blend
├── frames/                     # 渲染帧序列（.gitignore）
├── webm/                       # VP9 动画 (.gitignore)
├── gif/                        # 256 色 GIF (.gitignore)
├── sprites/{characters,objects,ui}/
├── bg/scenes/
├── audio/
└── scripts/                    # 自动化脚本
    ├── render.py          ← Blender headless 入口
    ├── render.sh          ← 一键 blend → frames → webm → gif
    ├── make_demo.py       ← 创建 demo blend
    └── make_tiantai.py    ← 创建天台山 blend
```

---

## 三、Systemd Timer 自动渲染

| 组件 | 路径 | 触发 |
|---|---|---|
| **service** | `/etc/systemd/system/dynsh-render.service` | oneshot |
| **timer** | `/etc/systemd/system/dynsh-render.timer` | 每天 **02:00 CST** |
| **脚本** | `/opt/ops/dynsh-render.sh` | 扫 blend → 渲染 → 合成 → 日志 |
| **日志** | `/var/log/dynsh-render.log` | 追加式 |

### 手动控制

```bash
# 立即跑一次
sudo systemctl start dynsh-render.service

# 查 timer 状态
sudo systemctl status dynsh-render.timer
sudo systemctl list-timers dynsh-render.timer

# 看日志
tail -f /var/log/dynsh-render.log

# 禁用
sudo systemctl disable dynsh-render.timer
```

### Analytics 日报

渲染状态会并入每日 08:30 发送的四站 analytics 日报（`/opt/ops/analytics-report.py` 已包含 dynsh.com）。

---

## 四、动画制作标准流程

### Step 1: 本地 Blender 桌面版做 .blend

**分辨率固定：1440×1000**（2D 横版）

```
Render Properties → Output Properties
  Resolution: X=1440, Y=1000, %=100
  Frame Rate: 24 fps

Render Engine: BLENDER_EEVEE（平涂色，快速）
  Bloom: OFF
  Screen Space Reflections: OFF
  Film Transparent: ON（2D 动画需要透明底）
```

**色彩规范（256 色目标）：**
- 全部用 `use_nodes = False` 的扁平色材质
- 避免渐变、纹理、阴影
- 色值取自 wayhug 色板：品牌蓝 `#5d8aa8` = `(0.365, 0.541, 0.659, 1.0)`

### Step 2: 上传到服务器

```bash
# 本地→服务器
scp "002-tiantai-mountain.blend" \
  ubuntu@129.211.170.72:/opt/dynsh.com/assets/animation/blend/002-tiantai-mountain/
```

### Step 3: Headless 渲染

```bash
# 前台渲染（看进度）
/snap/bin/blender --background /opt/dynsh.com/assets/animation/blend/XXX/xxx.blend \
  --render-output /opt/dynsh.com/assets/animation/frames/XXX/frame_#### \
  --render-frame 1..48

# 后台跑（推荐 2GB 服务器）
nohup /snap/bin/blender --background xxx.blend \
  --render-output frames/frame_#### --render-frame 1..48 \
  > render.log 2>&1 &
# 每帧约 55 秒，48 帧 ≈ 44 分钟
```

### Step 4: ffmpeg 合成

```bash
# WebM (VP9, 透明通道, 体积最小)
ffmpeg -y -framerate 24 -i frames/frame_%04d.png \
  -c:v libvpx-vp9 -pix_fmt yuva420p -b:v 0 -crf 28 -auto-alt-ref 0 \
  webm/xxx.webm

# GIF (256 色, Sierra2-4a 抖动)
ffmpeg -y -framerate 12 -i frames/frame_%04d.png \
  -vf "palettegen=max_colors=256:reserve_transparent=on" /tmp/palette.png
ffmpeg -y -framerate 12 -i frames/frame_%04d.png -i /tmp/palette.png \
  -lavfi "paletteuse=dither=sierra2_4a" gif/xxx.gif
```

### Step 5: 前端引用

```html
<!-- 推荐：WebM 优先 + GIF fallback -->
<video autoplay muted loop playsinline>
  <source src="/assets/animation/webm/xxx.webm" type="video/webm">
  <img src="/assets/animation/gif/xxx.gif" alt="...">
</video>
```

### 一键脚本

```bash
bash /opt/dynsh.com/assets/animation/scripts/render.sh \
  blend/002-tiantai-mountain/tiantai-he.blend \
  frames/tiantai-he 1 48
```

---

## 五、Blender 5.x 踩坑记录

| 坑 | 旧 API | Blender 5.x 新写法 |
|---|---|---|
| 材质节点 | `mat.use_nodes = False` | 加 try/except 包（已废弃但还能用） |
| 关键帧插值 | `action.fcurves` | 路径变了，暂不调（默认缓动可接受） |
| classic snap | `snap install blender` | 必须 `snap install blender --classic`（headless 要读外部文件） |
| 创建摄像机 | `bpy.data.objects.new` + `collection.link` | 用 `bpy.ops.object.camera_add()` 自动 link |
| headless 日志 | stdout/stderr 直接看 | 加 `2>&1 | tail -20` 过滤噪音 |

---

## 六、2GB 服务器渲染性能实测

| 分辨率 | 帧数 | 单帧耗时 | 总耗时 | 内存峰值 |
|---|---|---|---|---|
| 1440×1000 | 1 | ~50s | ~50s | ~1.2GB |
| 1440×1000 | 24 | ~55s | ~22min | ~1.4GB |
| 1440×1000 | 60 | ~55s | ~55min | OOM 风险 |

**策略：**
1. **分短片段**（每段 24-48 帧 = 1-2 秒 @24fps）
2. **后台渲染**（`nohup &` + `tail -f` 看进度）
3. **系统在凌晨自动跑**（dynsh-render.timer → 02:00）
4. 内存紧张时 **Swap 2GB 可用**（系统已有）

**推荐日常流程：** 本地 Blender 渲染完帧序列 → scp 帧到服务器 → 服务器只做 ffmpeg 合成（秒级完成）

---

## 七、已完成动画清单

| ID | 名称 | 文件 | 状态 | 说明 |
|---|---|---|---|---|
| 001 | demo-abu | blend/001-abu-early/demo.blend | ✅ | 阿布占位滑行 60 帧（后台渲染中） |
| 002 | 天台山·合和初现 | blend/002-tiantai-mountain/tiantai-he.blend | 🔄 渲染中 | 山峰升起 + 合字出现 + 阿布走入 + 波纹 |

---

## 八、Git 规则

```bash
# 入库 (.blend 源文件 + .py 脚本)
git add assets/animation/blend/ assets/animation/scripts/

# 不入库（产物太大）
# .gitignore 已配:
#   assets/animation/frames/
#   assets/animation/webm/
#   assets/animation/gif/
#   assets/animation/blend/*/*.blend
```

---

## 九、下一步待办

- [ ] 天台山完整故事（合和二仙寒山拾得相遇）
- [ ] 标语"合和和波"落地页面
- [ ] 前端 /studio 集成动画预览
- [ ] animation-report 单独邮件主题
- [ ] Krita/Inkscape 素材库整理（先本地做原画）
- [ ] 备案号补齐（公安 + ICP）

---

*让远山淡影轻轻升起，让合字在薄雾中浮现，让小小阿布从画面一端走来——这就是消除隔阂的第一步。*

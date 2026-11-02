// ============================================================
// 氛围灯 · 多语译读 · 项目集互链 · 外链分享
//
// 设计思路：
//   1. 「氛围灯」——美术化的前端设计技巧。首次悬停/点触时，
//      文字周围亮起一层柔和的呼吸光，译读卡片像灯下的一张纸，
//      语种按随机顺序依次浮现，不打断阅读。
//   2. 「翻译悬停」——默认界面仍为简体中文；首次悬停（桌面）
//      或首次点触（移动端）才出现译文，之后可点选语种固定。
//   3. 「社交化个人站点」——没有论坛、跟帖、评价等 UGC；
//      只用译读、分享、来信这些有温度的待客设计，让访客留下好印象。
//   4. 「分享外链」——分享入口在首次悬停/点触后浮现；微信内
//      走 JS-SDK 自定义分享卡片，缺凭证时降级为「右上角 … 」引导。
//
// 本文件在四个站点各存一份，仅 SITE 常量不同；其余逻辑保持一致。
// ============================================================

pub const SITE: &str = "nc";

const CSS: &str = r#"
/* ---------- 氛围灯 ---------- */
#aura-lamp{position:fixed;inset:0;pointer-events:none;z-index:2147483000;opacity:.5;
  background:
    radial-gradient(360px 300px at var(--lx,50%) var(--ly,18%),rgba(255,210,150,.20),transparent 68%),
    radial-gradient(420px 380px at 82% 82%,rgba(150,214,206,.16),transparent 72%);
  transition:opacity .8s ease}
[data-aura]{position:relative}
[data-aura] .aura-dot{position:absolute;top:-.6em;right:-1.15em;font:600 10px/1 system-ui,-apple-system,sans-serif;
  color:#8a6a2f;background:rgba(255,238,206,.94);border:1px solid rgba(200,160,90,.42);border-radius:999px;
  padding:2px 4px;opacity:.5;pointer-events:none;transition:opacity .3s ease,transform .3s ease}
[data-aura]:hover .aura-dot{opacity:1;transform:translateY(-1px)}
.aura-lit{position:relative;isolation:isolate}
.aura-lit::before{content:"";position:absolute;inset:-18px -22px;border-radius:18px;pointer-events:none;z-index:-1;
  background:
    radial-gradient(140px 90px at var(--aura-x,50%) var(--aura-y,10%),rgba(255,204,122,.58),transparent 72%),
    radial-gradient(200px 120px at 84% 100%,rgba(132,214,204,.38),transparent 74%);
  filter:blur(8px);opacity:.9;animation:aura-breathe 4.2s ease-in-out infinite}
@keyframes aura-breathe{0%,100%{opacity:.56;transform:scale(.985)}50%{opacity:.96;transform:scale(1.015)}}

/* ---------- 译读 / 分享卡片 ---------- */
.aura-panel{position:fixed;z-index:2147483002;box-sizing:border-box;max-width:calc(100vw - 32px);
  padding:14px 16px 12px;border-radius:16px;border:1px solid rgba(190,160,110,.28);
  background:linear-gradient(160deg,rgba(255,252,245,.97),rgba(247,252,250,.96));color:#26332f;
  box-shadow:0 12px 40px rgba(70,60,40,.16),0 0 0 1px rgba(255,255,255,.6) inset,0 0 42px rgba(255,206,140,.18);
  backdrop-filter:blur(14px) saturate(1.08);-webkit-backdrop-filter:blur(14px) saturate(1.08);
  opacity:0;transform:translateY(10px) scale(.985);pointer-events:none;
  transition:opacity .32s ease,transform .32s cubic-bezier(.2,.8,.2,1);
  font:14px/1.65 system-ui,-apple-system,"Segoe UI","Noto Sans SC","Noto Sans Tibetan",sans-serif}
.aura-panel.open{opacity:1;transform:none;pointer-events:auto}
.aura-panel.sheet{left:12px!important;right:12px!important;top:auto!important;bottom:12px;width:auto!important;max-width:none}
.aura-panel-head{display:flex;align-items:center;gap:8px;margin-bottom:6px}
.aura-lamp-dot{width:9px;height:9px;border-radius:50%;flex:0 0 auto;
  background:radial-gradient(circle at 35% 30%,#fff3cf,#f3b84f 55%,#c98f45);
  box-shadow:0 0 12px rgba(255,190,90,.85)}
.aura-lang{font-weight:650;letter-spacing:.04em;font-size:13px;color:#4a3a22;flex:1}
.aura-close{appearance:none;border:0;background:transparent;color:#8b7a62;font:20px/1 system-ui,sans-serif;
  cursor:pointer;padding:0 2px;border-radius:8px}
.aura-close:hover{color:#3a2f20;background:rgba(180,150,100,.12)}
.aura-text{margin:0;font-size:15px;line-height:1.7;color:#22302c;min-height:1.7em;
  animation:aura-rise .45s ease both}
@keyframes aura-rise{from{opacity:0;transform:translateY(6px)}to{opacity:1;transform:none}}
.aura-chips{display:flex;flex-wrap:wrap;gap:6px;margin-top:10px}
.aura-chip{appearance:none;border:1px solid rgba(150,130,90,.3);background:rgba(255,255,255,.55);
  color:#4a4232;border-radius:999px;padding:4px 9px;font:12px/1.2 inherit;cursor:pointer;
  transition:background .2s,color .2s,border-color .2s,box-shadow .2s}
.aura-chip:hover{border-color:rgba(150,110,50,.55)}
.aura-chip.on{background:#2f3a35;color:#fff3dc;border-color:#2f3a35;box-shadow:0 0 16px rgba(255,200,120,.5)}
.aura-hint{margin:9px 0 0;font-size:11px;line-height:1.5;color:#8b7a62}

/* ---------- 页脚互链 / 分享入口 ---------- */
.aura-foot{display:flex;flex-wrap:wrap;gap:10px 18px;align-items:center;justify-content:center;
  margin:18px 0 12px;font:13px/1.7 inherit;color:inherit}
.aura-links{display:flex;flex-wrap:wrap;gap:4px 12px;align-items:center;justify-content:center}
.aura-links-label{opacity:.6;letter-spacing:.1em;font-size:10px;text-transform:uppercase}
.aura-links a{color:inherit;text-decoration:none;border-bottom:1px dotted currentColor;opacity:.82}
.aura-links a:hover{opacity:1;border-bottom-style:solid}
.aura-share-trigger{color:inherit;text-decoration:none;cursor:pointer;
  opacity:0;transition:opacity .55s ease}
body.aura-awake .aura-share-trigger{opacity:.72}
.aura-share-trigger:hover{opacity:1;text-decoration:underline;text-underline-offset:3px}
.aura-orb{position:fixed;right:14px;bottom:14px;z-index:2147483001;appearance:none;
  width:34px;height:34px;padding:0;border:1px solid rgba(130,115,90,.28);border-radius:50%;
  background:rgba(255,252,246,.9);color:#5a4a30;font:15px/1 system-ui,-apple-system,sans-serif;
  cursor:pointer;box-shadow:0 4px 14px rgba(80,60,30,.12);
  opacity:0;transform:scale(.9);transition:opacity .35s ease,transform .35s ease}
.aura-orb.show{opacity:.9;transform:none}
.aura-orb:hover{opacity:1}

/* ---------- 分享卡片（极简） ---------- */
.aura-share-panel{left:auto!important;right:14px;top:auto!important;bottom:58px;
  width:min(292px,calc(100vw - 28px));padding:10px 12px 11px}
.aura-share-actions{display:flex;flex-wrap:wrap;gap:6px}
.aura-act{appearance:none;border:1px solid rgba(120,100,60,.26);background:transparent;color:#3a3226;
  border-radius:8px;padding:6px 10px;font:12px/1.2 inherit;cursor:pointer;transition:background .2s}
.aura-act:hover{background:rgba(180,150,100,.1)}
.aura-act.primary{background:#2f3a35;color:#fff3dc;border-color:#2f3a35}

/* ---------- 微信引导（极简） ---------- */
.aura-guide{position:fixed;inset:0;z-index:2147483003;display:flex;align-items:center;justify-content:center;
  background:rgba(18,22,20,.38);opacity:0;pointer-events:none;transition:opacity .25s ease}
.aura-guide.open{opacity:1;pointer-events:auto}
.aura-guide-card{width:min(320px,calc(100vw - 40px));box-sizing:border-box;padding:16px 18px;border-radius:14px;
  background:#fffdf8;color:#26332f;border:1px solid rgba(190,160,110,.26);
  box-shadow:0 10px 34px rgba(20,30,25,.2)}
.aura-guide-card h3{margin:0 0 8px;font-size:15px;font-weight:650;color:#2a352f}
.aura-guide-card p{margin:0 0 12px;font-size:12px;line-height:1.7;color:#6a5f4e}
.aura-guide-card .aura-act{margin-right:6px}
.aura-toast{position:fixed;left:50%;bottom:80px;transform:translate(-50%,8px);z-index:2147483004;
  padding:7px 12px;border-radius:999px;background:rgba(44,52,47,.9);color:#fff3dc;
  font:12px/1.2 system-ui,sans-serif;opacity:0;pointer-events:none;transition:opacity .25s,transform .25s}
.aura-toast.show{opacity:1;transform:translate(-50%,0)}

@media (prefers-reduced-motion:reduce){
  .aura-lit::before{animation:none}
  .aura-panel,.aura-orb,.aura-share-trigger,.aura-guide,.aura-toast{transition:none}
  .aura-text{animation:none}
}
"#;

const JS: &str = r##"
(function () {
  'use strict';
  var CFG = window.__AURA__ || { site: '', i18n: {} };
  var reduce = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;
  var touch = window.matchMedia && window.matchMedia('(hover: none)').matches;

  var LANGS = {
    sj: ['es', 'fr', 'ja', 'en', 'bo', 'zh-TW', 'zh-CN'],
    wh: ['es', 'fr', 'ja', 'en', 'bo', 'zh-TW', 'zh-CN'],
    '37': ['en', 'zh-TW', 'zh-CN'],
    nc: []
  };
  var NAMES = {
    es: 'Español', fr: 'Français', ja: '日本語', en: 'English',
    bo: 'བོད་ཡིག', 'zh-TW': '繁體中文', 'zh-CN': '简体中文'
  };
  var PROJECTS = [
    ['sj', 'jshnp.com', '旷野馨诗'],
    ['wh', 'wayhug.com', '王元会 · Wayhug'],
    ['37', '37xxi.com', '三七二十一'],
    ['nc', 'dynsh.com', '大隐士']
  ];
  var langs = LANGS[CFG.site] || [];

  // ---------- 氛围灯：跟随鼠标/指尖的一层柔光 ----------
  var lamp = document.createElement('div');
  lamp.id = 'aura-lamp';
  lamp.setAttribute('aria-hidden', 'true');
  document.body.appendChild(lamp);
  var lx = 50, ly = 18, raf = 0;
  function moveLamp(x, y) {
    lx = x; ly = y;
    if (raf) return;
    raf = requestAnimationFrame(function () {
      raf = 0;
      lamp.style.setProperty('--lx', lx + 'px');
      lamp.style.setProperty('--ly', ly + 'px');
    });
  }
  document.addEventListener('pointermove', function (e) { moveLamp(e.clientX, e.clientY); }, { passive: true });

  // ---------- 译读卡片 ----------
  var trPanel = null, trEl = null, trTimer = null, trPinned = false;

  function ensureTrPanel() {
    if (trPanel) return trPanel;
    trPanel = document.createElement('div');
    trPanel.className = 'aura-panel';
    trPanel.setAttribute('role', 'dialog');
    trPanel.setAttribute('aria-live', 'polite');
    trPanel.innerHTML =
      '<div class="aura-panel-head"><span class="aura-lamp-dot" aria-hidden="true"></span>' +
      '<span class="aura-lang"></span>' +
      '<button class="aura-close" type="button" aria-label="关闭">×</button></div>' +
      '<p class="aura-text"></p><div class="aura-chips"></div>' +
      '<p class="aura-hint">首次悬停或轻触，语种每 2.6 秒依次亮起；点选语种可固定。点击空白处关闭。</p>';
    document.body.appendChild(trPanel);
    trPanel.querySelector('.aura-close').addEventListener('click', closeTr);
    return trPanel;
  }

  function shuffle(a) {
    for (var i = a.length - 1; i > 0; i--) {
      var j = Math.floor(Math.random() * (i + 1));
      var t = a[i]; a[i] = a[j]; a[j] = t;
    }
    return a;
  }

  function renderLang(lang) {
    var dict = (CFG.i18n || {})[trKey] || {};
    var text = dict[lang] || '';
    trPanel.querySelector('.aura-lang').textContent = '译读 · ' + (NAMES[lang] || lang);
    var body = trPanel.querySelector('.aura-text');
    body.textContent = text;
    body.style.animation = 'none';
    void body.offsetWidth;
    body.style.animation = '';
    Array.prototype.forEach.call(trPanel.querySelectorAll('.aura-chip'), function (c) {
      c.classList.toggle('on', c.getAttribute('data-lang') === lang);
    });
    try { sessionStorage.setItem('aura:last:' + CFG.site + ':' + trKey, lang); } catch (e) {}
  }

  var trKey = '';

  function placeTr(el) {
    if (!trPanel) return;
    if (window.innerWidth <= 640 || touch) {
      trPanel.classList.add('sheet');
    } else {
      trPanel.classList.remove('sheet');
      var r = el.getBoundingClientRect();
      var w = Math.min(560, window.innerWidth - 32);
      trPanel.style.width = w + 'px';
      var left = Math.min(Math.max(16, r.left), window.innerWidth - w - 16);
      var ph = trPanel.offsetHeight || 170;
      var top = r.bottom + 10;
      if (top + ph > window.innerHeight - 12) top = Math.max(12, r.top - ph - 10);
      trPanel.style.left = left + 'px';
      trPanel.style.top = top + 'px';
    }
  }

  function closeTr() {
    if (trTimer) { clearInterval(trTimer); trTimer = null; }
    if (trPanel) trPanel.classList.remove('open');
    if (trEl) trEl.classList.remove('aura-lit');
    trEl = null;
  }

  function openTr(el, key, ev) {
    trKey = key;
    var dict = (CFG.i18n || {})[key] || {};
    var avail = langs.filter(function (l) { return dict[l]; });
    if (!avail.length) return;
    ensureTrPanel();
    if (trEl && trEl !== el) trEl.classList.remove('aura-lit');
    trEl = el;
    el.classList.add('aura-lit');
    if (ev) {
      var r = el.getBoundingClientRect();
      el.style.setProperty('--aura-x', Math.max(0, Math.min(100, ((ev.clientX - r.left) / Math.max(1, r.width)) * 100)) + '%');
      el.style.setProperty('--aura-y', Math.max(0, Math.min(100, ((ev.clientY - r.top) / Math.max(1, r.height)) * 100)) + '%');
    }
    var chips = trPanel.querySelector('.aura-chips');
    chips.innerHTML = '';
    avail.forEach(function (l) {
      var b = document.createElement('button');
      b.type = 'button'; b.className = 'aura-chip'; b.textContent = NAMES[l] || l;
      b.setAttribute('data-lang', l);
      b.addEventListener('click', function () {
        trPinned = true;
        if (trTimer) { clearInterval(trTimer); trTimer = null; }
        renderLang(l);
      });
      chips.appendChild(b);
    });
    trPanel.classList.add('open');
    placeTr(el);

    trPinned = false;
    var seen = false;
    try { seen = sessionStorage.getItem('aura:seen:' + CFG.site + ':' + key) === '1'; } catch (e) {}
    var order = shuffle(avail.slice());
    var i = 0;
    if (trTimer) { clearInterval(trTimer); trTimer = null; }
    function step() {
      if (trPinned) return;
      if (i >= order.length) {
        if (trTimer) { clearInterval(trTimer); trTimer = null; }
        try { sessionStorage.setItem('aura:seen:' + CFG.site + ':' + key, '1'); } catch (e) {}
        return;
      }
      renderLang(order[i]); i++;
    }
    if (seen) {
      var last = '';
      try { last = sessionStorage.getItem('aura:last:' + CFG.site + ':' + key) || ''; } catch (e) {}
      renderLang(last && dict[last] ? last : avail[0]);
    } else {
      step();
      if (!reduce) trTimer = setInterval(step, 2600);
    }
  }

  var marked = Array.prototype.slice.call(document.querySelectorAll('[data-aura]'));
  marked.forEach(function (el) {
    if (!touch) {
      var dot = document.createElement('span');
      dot.className = 'aura-dot'; dot.textContent = '译'; dot.setAttribute('aria-hidden', 'true');
      el.appendChild(dot);
      el.setAttribute('title', '悬停或轻触：依次查看多语译读');
    }
    var fired = false;
    function fire(ev) {
      if (fired && trEl === el && trPanel && trPanel.classList.contains('open')) return;
      fired = true;
      openTr(el, el.getAttribute('data-aura'), ev);
    }
    el.addEventListener('pointerenter', function (e) {
      if (e.pointerType !== 'touch') fire(e);
    });
    el.addEventListener('touchstart', function (e) {
      fire(e.touches && e.touches[0] ? e.touches[0] : null);
    }, { passive: true });
    el.addEventListener('click', function (e) {
      if (e.pointerType === 'touch' || touch) {
        fire(e);
        // 标题若被链接包裹（如三七二十一领取录音），首次点触先看译读，不跳转
        if (el.closest('a') && !el.__auraTap2) { el.__auraTap2 = true; e.preventDefault(); }
      }
    });
  });

  // ---------- 分享 ----------
  var shPanel = null, shOpen = false;

  function ensureShPanel() {
    if (shPanel) return shPanel;
    shPanel = document.createElement('div');
    shPanel.className = 'aura-panel aura-share-panel';
    shPanel.setAttribute('role', 'dialog');
    shPanel.innerHTML =
      '<div class="aura-panel-head">' +
      '<span class="aura-lang">分享 / Share</span>' +
      '<button class="aura-close" type="button" aria-label="关闭 / Close">×</button></div>' +
      '<div class="aura-share-actions">' +
      '<button class="aura-act primary" type="button" data-share="copy">复制 / Copy</button>' +
      '<button class="aura-act" type="button" data-share="native">系统 / Share</button>' +
      '<button class="aura-act" type="button" data-share="wechat">微信 / WeChat</button>' +
      '</div>';
    document.body.appendChild(shPanel);
    shPanel.querySelector('.aura-close').addEventListener('click', closeShare);
    shPanel.querySelector('[data-share="copy"]').addEventListener('click', function () { copyLink(); });
    shPanel.querySelector('[data-share="native"]').addEventListener('click', nativeShare);
    shPanel.querySelector('[data-share="wechat"]').addEventListener('click', showWxGuide);
    return shPanel;
  }

  function openShare(ev) {
    ensureShPanel();
    if (window.innerWidth <= 640 || touch) shPanel.classList.add('sheet');
    shPanel.classList.add('open');
    shOpen = true;
    track('share_open');
    if (ev && ev.preventDefault) ev.preventDefault();
  }

  function closeShare() {
    if (shPanel) shPanel.classList.remove('open');
    shOpen = false;
  }

  function copyText(text) {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      return navigator.clipboard.writeText(text).then(function () { toast('已复制 / Copied'); }, function () { fallbackCopy(text); });
    }
    fallbackCopy(text);
    return Promise.resolve();
  }
  function fallbackCopy(text) {
    var t = document.createElement('textarea');
    t.value = text; t.style.position = 'fixed'; t.style.opacity = '0';
    document.body.appendChild(t); t.select();
    try { document.execCommand('copy'); toast('已复制 / Copied'); } catch (e) { toast('请手动复制 / Copy manually'); }
    document.body.removeChild(t);
  }
  function copyLink() { copyText(location.href.split('#')[0]); track('share_copy'); }
  function nativeShare() {
    var data = { title: document.title, text: document.title, url: location.href.split('#')[0] };
    if (navigator.share) {
      navigator.share(data).then(function () { track('share_native'); }, function () {});
    } else {
      copyText(data.url);
      toast('已复制，可粘贴 / Copied');
      track('share_copy');
    }
  }

  var toastEl = null;
  function toast(msg) {
    if (!toastEl) { toastEl = document.createElement('div'); toastEl.className = 'aura-toast'; document.body.appendChild(toastEl); }
    toastEl.textContent = msg;
    toastEl.classList.add('show');
    clearTimeout(toast._t);
    toast._t = setTimeout(function () { toastEl.classList.remove('show'); }, 1800);
  }

  var guide = null;
  function showWxGuide() {
    if (!guide) {
      guide = document.createElement('div');
      guide.className = 'aura-guide';
      guide.innerHTML =
        '<div class="aura-guide-card"><h3>分享到微信 / Share to WeChat</h3>' +
        '<p>微信内点右上角「···」，发送给朋友 / Tap ··· at top right in WeChat to forward</p>' +
        '<button class="aura-act primary" type="button" data-share="copy">复制 / Copy</button>' +
        '<button class="aura-act" type="button" data-share="close">关闭 / Close</button></div>';
      document.body.appendChild(guide);
      guide.addEventListener('click', function (e) {
        if (e.target === guide) guide.classList.remove('open');
      });
      guide.querySelector('[data-share="copy"]').addEventListener('click', copyLink);
      guide.querySelector('[data-share="close"]').addEventListener('click', function () { guide.classList.remove('open'); });
    }
    guide.classList.add('open');
    track('share_wechat_guide');
  }

  function track(event) {
    try {
      fetch('/api/track', {
        method: 'POST', credentials: 'same-origin',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ kind: 'event', event: event, extra: { site: CFG.site } }),
        keepalive: true
      }).catch(function () {});
    } catch (e) {}
  }

  // 页脚：项目集互链（服务端已渲染时跳过）
  function ensureFooter() {
    if (document.querySelector('.aura-foot')) return;
    var host = document.querySelector('footer') || document.body;
    var links = PROJECTS.filter(function (p) { return p[0] !== CFG.site; }).map(function (p) {
      return '<a href="https://' + p[1] + '/" target="_blank" rel="noopener noreferrer">' + p[2] + '</a>';
    }).join('');
    var div = document.createElement('div');
    div.className = 'aura-foot';
    div.innerHTML = '<span class="aura-links-label">项目集</span><div class="aura-links">' + links + '</div>' +
      '<a href="#" class="aura-share-trigger" data-aura-share>分享 / Share</a>';
    host.appendChild(div);
    bindShareTriggers(div);
  }

  function bindShareTriggers(root) {
    Array.prototype.forEach.call((root || document).querySelectorAll('[data-aura-share]'), function (el) {
      if (el.__auraShare) return;
      el.__auraShare = true;
      el.addEventListener('click', function (e) { openShare(e); });
      if (!touch) el.addEventListener('pointerenter', function (e) { openShare(e); });
      el.addEventListener('touchstart', function (e) { openShare(e); }, { passive: true });
    });
  }

  // 首次悬停/点触：唤醒页面分享入口
  var awake = false;
  function wake() {
    if (awake) return;
    awake = true;
    document.body.classList.add('aura-awake');
    var orb = document.createElement('button');
    orb.type = 'button'; orb.className = 'aura-orb'; orb.textContent = '↗';
    orb.setAttribute('aria-label', '分享 / Share');
    document.body.appendChild(orb);
    setTimeout(function () { orb.classList.add('show'); }, 140);
    orb.addEventListener('click', function (e) { openShare(e); });
  }
  document.addEventListener('pointerover', wake, { once: true, passive: true });
  document.addEventListener('touchstart', wake, { once: true, passive: true });
  document.addEventListener('scroll', wake, { once: true, passive: true });

  document.addEventListener('pointerdown', function (e) {
    var t = e.target;
    if (shOpen && shPanel && !shPanel.contains(t) && !(t.closest && t.closest('[data-aura-share]'))) closeShare();
    if (trPanel && trPanel.classList.contains('open') && !trPanel.contains(t) && !(t.closest && t.closest('[data-aura]'))) closeTr();
  }, true);
  document.addEventListener('keydown', function (e) {
    if (e.key === 'Escape') { closeTr(); closeShare(); if (guide) guide.classList.remove('open'); }
  });
  window.addEventListener('resize', function () { closeTr(); closeShare(); }, { passive: true });

  // ---------- 微信 JS-SDK（有凭证时自定义分享卡片） ----------
  function setupWx() {
    if (!/MicroMessenger/i.test(navigator.userAgent)) return;
    var pageUrl = location.href.split('#')[0];
    fetch('/api/wx/signature?url=' + encodeURIComponent(pageUrl), { credentials: 'same-origin' })
      .then(function (r) { return r.json(); })
      .then(function (d) {
        if (!d || !d.ok) return;
        loadScript('https://res.wx.qq.com/open/js/jweixin-1.6.0.js', function () {
          if (!window.wx) return;
          window.wx.config({
            debug: false, appId: d.appId, timestamp: d.timestamp,
            nonceStr: d.nonceStr, signature: d.signature,
            jsApiList: ['updateAppMessageShareData', 'updateTimelineShareData']
          });
          window.wx.ready(function () {
            var icon = pickIcon();
            var share = { title: document.title, desc: pickDesc(), link: pageUrl };
            if (icon) share.imgUrl = icon;
            window.wx.updateAppMessageShareData(share);
            window.wx.updateTimelineShareData(share);
          });
        });
      })
      .catch(function () {});
  }
  function pickDesc() {
    var m = document.querySelector('meta[name="description"]') || document.querySelector('meta[property="og:description"]');
    return m ? (m.getAttribute('content') || '') : '';
  }
  function pickIcon() {
    var links = document.querySelectorAll('link[rel="icon"],link[rel="apple-touch-icon"]');
    for (var i = links.length - 1; i >= 0; i--) {
      var href = links[i].getAttribute('href') || '';
      if (href && href.indexOf('data:') !== 0) {
        try { return new URL(href, location.href).href; } catch (e) {}
      }
    }
    return '';
  }
  function loadScript(src, done) {
    var s = document.createElement('script');
    s.src = src; s.async = true; s.onload = done;
    document.head.appendChild(s);
  }

  function boot() {
    ensureFooter();
    bindShareTriggers(document);
    setupWx();
  }
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', boot);
  else boot();
})();
"##;

/// 译读词表（与服务端页面上的 data-aura 键一一对应）
fn i18n() -> &'static str {
    match SITE {
        "sj" => r#"{
  "name":{"es":"Poesía del Campo Abierto","fr":"Poésie des Champs Ouverts","ja":"旷野馨詩","en":"Joy Surrounded Haven Near Poetry","bo":"ཐང་ཆེན་སྙན་ངག","zh-TW":"曠野馨詩","zh-CN":"旷野馨诗"},
  "desc":{"es":"Un blog literario personal que publica por correo: ensayos, poemas, notas de lectura y opiniones.","fr":"Un blog littéraire personnel alimenté par courriel : essais, poèmes, notes de lecture et opinions.","ja":"メールで更新する個人文学ブログ。随筆、詩、読書ノート、意見。","en":"A personal literary blog published by email: essays, poems, reading notes, and opinions.","bo":"གློག་འཕྲིན་ཐོག་ནས་པར་སྐྲུན་བྱེད་པའི་སྙན་ངག་དང་རྩོམ་ཡིག་སྒེར་གྱི་དྲ་བ།","zh-TW":"曠野馨詩 — 以郵件發文的個人文字部落格，隨筆、詩、讀書筆記、觀點","zh-CN":"旷野馨诗 — 以邮件发文的个人文字博客，随笔、诗、读书笔记、观点"},
  "slogan":{"es":"Somos amigos.","fr":"Nous sommes amis.","ja":"私たちは友達です。","en":"We are friends.","bo":"ང་ཚོ་གྲོགས་པོ་ཡིན།","zh-TW":"我們是朋友。","zh-CN":"我们是朋友。"},
  "tag1":{"es":"Un viaje por la sinología, las humanidades y la filosofía natural.","fr":"Un voyage dans la sinologie, les sciences humaines et la philosophie de la nature.","ja":"漢学・人文科学・自然哲学への旅。","en":"Journey to Sinology, Humanities & Natural Philosophy","bo":"རྒྱ་ནག་རིག་གནས་དང་མི་རིགས་རིག་པ། རང་བྱུང་གི་ཚན་རིག་གི་ལམ།","zh-TW":"漢學、人文科學與自然哲學之旅","zh-CN":"汉学、人文科学与自然哲学之旅"},
  "tag2":{"es":"La belleza serena y eterna.","fr":"La beauté sereine et éternelle.","ja":"安らかで永遠の美。","en":"Serene and eternal beauty.","bo":"ཞི་བདེ་དང་རྟག་བརྟན་གྱི་མཛེས་སྡུག","zh-TW":"安定和永恆的美好","zh-CN":"安定和永恒的美好"},
  "tag3":{"es":"Trazando la evolución pacífica que viene para la humanidad.","fr":"Tracer l'évolution pacifique à venir de l'humanité.","ja":"人類に訪れる平和の進化を描く。","en":"Charting humanity's coming evolution in peace.","bo":"མི་རིགས་ཀྱི་ཞི་བདེའི་འཕེལ་རྒྱས་གཏོང་བ།","zh-TW":"佈局人類正在到來的和平的演進","zh-CN":"布局人类正在到来的和平的演进"},
  "tag4":{"es":"Reflejos, gota a gota.","fr":"Des reflets, goutte à goutte.","ja":"点滴の反映。","en":"Reflections, drop by drop.","bo":"ཐིག་རེ་ཐིག་རེའི་གཟུགས་བརྙན།","zh-TW":"點點滴滴的倒影","zh-CN":"点点滴滴的倒影"}
}"#,
        "wh" => r#"{
  "name":{"es":"Wang Yuanhui · Wayhug","fr":"Wang Yuanhui · Wayhug","ja":"王元会 · Wayhug","en":"Wang Yuanhui · Wayhug","bo":"ཝང་ཡོན་ཧུའི་ · Wayhug","zh-TW":"王元會 · Wayhug","zh-CN":"王元会 · Wayhug"},
  "slogan":{"es":"Flores que hablan, pétalos que caen.","fr":"Langage des fleurs, pétales à zéro.","ja":"花の言葉、花は零へ。","en":"Flower language, blooming to zero.","bo":"མེ་ཏོག་གི་བརྗོད་པ། མེ་ཏོག་ཀླད་ཀོར་དུ་འགྲོ།","zh-TW":"花語花零","zh-CN":"花语花零"},
  "h1":{"es":"Convierte un texto en una animación que respira.","fr":"Transformez un texte en une animation qui respire.","ja":"ひとつの文章を、息づくアニメーションへ。","en":"Turn a piece of writing into an animation that breathes.","bo":"ཡིག་ཆ་ཞིག་ལ་དབུགས་འབྱིན་པའི་འགུལ་བརྙན་དུ་བསྒྱུར།","zh-TW":"把一段文字，變成一部會呼吸的動畫。","zh-CN":"把一段文字，变成一部会呼吸的动画。"},
  "lede":{"es":"Mira un TXT convertirse en una animación plana en CSS/HTML5 y descarga un certificado A4 imprimible y verificable. Licencia MIT gratuita.","fr":"Voyez un TXT devenir une animation plate en CSS/HTML5 et téléchargez un certificat A4 imprimable et vérifiable. Licence MIT gratuite.","ja":"TXTがフラットなCSS／HTML5アニメーションに育つ様子を見て、印刷・検証できるA4記念証書をダウンロード。MITライセンスで無償公開。","en":"Watch a TXT grow into a flat CSS / HTML5 animation, and download a printable, verifiable A4 keepsake certificate. Free under the MIT license.","bo":"TXT ཡིག་ཆ་ནས་CSS/HTML5 གི་འགུལ་བརྙན་དུ་འཚར་བ་ལྟ་ཞིག ཤོག་ལེབ་A4 གི་དྲན་གསོའི་ལག་ཁྱེར་ཕབ་ལེན་བྱོས། MIT ཆོག་མཆན་གྱིས་རིན་མེད།","zh-TW":"看一段 TXT 長成扁平的 CSS / HTML5 動畫，下載一張可列印、可校驗的 A4 紀念證書。以 MIT 協議免費授權。","zh-CN":"看一段 TXT 长成扁平的 CSS / HTML5 动画，下载一张可打印、可校验的 A4 纪念证书。按 MIT 协议免费授权。"},
  "watch":{"es":"Ver","fr":"Voir","ja":"見る","en":"Watch","bo":"ལྟ་བ།","zh-TW":"看","zh-CN":"看"},
  "world":{"es":"Cuatro fases · Cuatro utensilios","fr":"Quatre phases · Quatre ustensiles","ja":"四相・四具","en":"Four phases · Four vessels","bo":"དུས་སྐབས་བཞི། ཡོ་བྱད་བཞི།","zh-TW":"四相 · 四具","zh-CN":"四相 · 四具"}
}"#,
        "37" => r#"{
  "name":{"es":"Tres por siete: veintiuno","fr":"Trois fois sept : vingt et un","ja":"三七二十一（さんしちにじゅういち）","en":"Three Seven Twenty-One","bo":"གསུམ་བདུན་ཉི་ཤུ་རྩ་གཅིག","zh-TW":"三七二十一","zh-CN":"三七二十一"},
  "slogan":{"es":"Ven, siéntate.","fr":"Venez, asseyez-vous.","ja":"さあ、お座りください。","en":"Come, take a seat.","bo":"ཤོག་དང་འདུག","zh-TW":"來，請坐！","zh-CN":"来，请坐！"},
  "hint":{"es":"Toca el título para recibir una grabación aleatoria · una al día.","fr":"Touchez le titre pour recevoir un enregistrement au hasard · un par jour.","ja":"タイトルをタップすると、録音をランダムに受け取れます・1日1本。","en":"Tap the title to receive a random recording · one per day.","bo":"མགོ་བརྗོད་ལ་མནན་ན་སྒྲ་ཕབ་རྩ་ཆག་ཞིག་ཐོབ། ཉིན་རེར་གཅིག","zh-TW":"點擊標題，隨機領取一段錄音 · 每天一段","zh-CN":"点击标题，随机领取一段录音 · 每天一段"},
  "letters":{"es":"Una conversación de un turno.","fr":"Une conversation en un tour.","ja":"一往復の対話。","en":"A one-round conversation.","bo":"ཐེངས་གཅིག་གི་གླེང་མོལ།","zh-TW":"一 回 合 的 對 話","zh-CN":"一 回 合 的 对 话"},
  "lettersSub":{"es":"A plantea una preocupación, B responde. Las cartas se acumulan una a una.","fr":"A expose un souci, B répond. Les lettres s'accumulent une à une.","ja":"Aが悩みを語り、Bが応える。手紙は一通ずつ積み重なっていきます。","en":"A raises a concern, B responds. Letters accumulate, one by one.","bo":"A ཡིས་དཀའ་ངལ་བཤད། B ཡིས་ལན་འདེབས། ཡིག་བརྡ་རེ་རེ་བཞིན་འཚོགས།","zh-TW":"A 提出困擾，B 給出回應。來信會一封封累積。","zh-CN":"A 提出困扰，B 给出回应。来信会一封封累积。"}
}"#,
        _ => "{}",
    }
}

/// 头部样式（插入 <head>）
pub fn head() -> String {
    format!("<style>{}</style>", CSS)
}

/// 页脚互链 + 分享入口（可选的 SSR 版本；JS 会在缺失时自动补齐）
pub fn footer_extra() -> String {
    let mut links = String::new();
    for (id, host, name) in [
        ("sj", "jshnp.com", "旷野馨诗"),
        ("wh", "wayhug.com", "王元会 · Wayhug"),
        ("37", "37xxi.com", "三七二十一"),
        ("nc", "dynsh.com", "大隐士"),
    ] {
        if id == SITE {
            continue;
        }
        links.push_str(&format!(
            r#"<a href="https://{host}/" target="_blank" rel="noopener noreferrer">{name}</a>"#
        ));
    }
    format!(
        r##"<div class="aura-foot"><span class="aura-links-label">项目集</span><div class="aura-links">{links}</div><a href="#" class="aura-share-trigger" data-aura-share>分享 / Share</a></div>"##
    )
}

/// 注入配置与前端逻辑（插入 </body> 之前）
pub fn init() -> String {
    format!(
        "<script>window.__AURA__={{site:'{site}',i18n:{i18n}}};</script>\n<script>{js}</script>",
        site = SITE,
        i18n = i18n(),
        js = JS
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i18n_is_valid_json() {
        let v: serde_json::Value = serde_json::from_str(i18n()).expect("i18n 必须是合法 JSON");
        assert!(v.is_object());
    }

    #[test]
    fn footer_excludes_self() {
        let f = footer_extra();
        assert!(f.contains("aura-links"));
        assert_eq!(f.matches("target=\"_blank\"").count(), 3);
    }
}

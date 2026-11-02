// ============================================================
// 微信外链分享 · JS-SDK 签名端点
//
// GET /api/wx/signature?url=<当前页面完整地址>
//
// 需要服务端 .env 配置：
//   WX_APPID=公众号 AppID
//   WX_APP_SECRET=公众号 AppSecret
//
// 未配置时返回 {ok:false, code:"not_configured"}，前端自动降级为
// 「点右上角 … 分享」的引导，不影响站点其他功能。
//
// 说明：微信 JS-SDK 只允许在公众号后台「JS 接口安全域名」中登记的
// 域名下调用；四个站点域名需逐个登记（公众号后台有数量上限）。
// 分享内容本身仍是外链页面，不会变成微信内部文章。
// ============================================================

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
pub struct SignQuery {
    pub url: Option<String>,
}

struct CachedTicket {
    appid: String,
    ticket: String,
    at: Instant,
}

static CACHE: OnceLock<Mutex<Option<CachedTicket>>> = OnceLock::new();

fn cache() -> &'static Mutex<Option<CachedTicket>> {
    CACHE.get_or_init(|| Mutex::new(None))
}

fn cached_ticket(appid: &str) -> Option<String> {
    let guard = cache().lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().and_then(|c| {
        if c.appid == appid && c.at.elapsed() < Duration::from_secs(7000) {
            Some(c.ticket.clone())
        } else {
            None
        }
    })
}

fn store_ticket(appid: &str, ticket: &str) {
    let mut guard = cache().lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(CachedTicket {
        appid: appid.to_string(),
        ticket: ticket.to_string(),
        at: Instant::now(),
    });
}

/// 只允许为本项目集域名签名，避免被当成通用跳板
fn allowed_url(url: &str) -> bool {
    let Some(idx) = url.find("://") else {
        return false;
    };
    let rest = &url[idx + 3..];
    let host_port = rest
        .split(|c| c == '/' || c == '?' || c == '#')
        .next()
        .unwrap_or("");
    let host = host_port.split(':').next().unwrap_or("");
    matches!(
        host,
        "jshnp.com" | "wayhug.com" | "37xxi.com" | "dynsh.com" | "localhost" | "127.0.0.1"
    )
}

fn curl_json(url: &str) -> Option<Value> {
    let out = Command::new("curl")
        .args(["-sS", "--max-time", "10", url])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    serde_json::from_slice(&out.stdout).ok()
}

fn fetch_ticket(appid: &str, secret: &str) -> Result<String, String> {
    if let Some(t) = cached_ticket(appid) {
        return Ok(t);
    }
    let token_url = format!(
        "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
        appid, secret
    );
    let token_body = curl_json(&token_url).ok_or_else(|| "无法连接微信 access_token 接口".to_string())?;
    let token = token_body
        .get("access_token")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            token_body
                .get("errmsg")
                .and_then(Value::as_str)
                .unwrap_or("微信未返回 access_token")
                .to_string()
        })?;

    let ticket_url = format!(
        "https://api.weixin.qq.com/cgi-bin/ticket/getticket?access_token={}&type=jsapi",
        token
    );
    let ticket_body = curl_json(&ticket_url).ok_or_else(|| "无法连接微信 jsapi_ticket 接口".to_string())?;
    let ticket = ticket_body
        .get("ticket")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            ticket_body
                .get("errmsg")
                .and_then(Value::as_str)
                .unwrap_or("微信未返回 jsapi_ticket")
                .to_string()
        })?;
    store_ticket(appid, ticket);
    Ok(ticket.to_string())
}

fn nonce_str() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    format!("{:08x}{:08x}", std::process::id(), nanos)
}

/// 纯 Rust SHA-1（避免为签名引入额外 crate）
fn sha1_hex(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut msg = bytes.to_vec();
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    let bit_len = (bytes.len() as u64).wrapping_mul(8);
    msg.extend_from_slice(&bit_len.to_be_bytes());

    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xefcdab89;
    let mut h2: u32 = 0x98badcfe;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xc3d2e1f0;

    for chunk in msg.chunks(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let (mut a, mut b, mut c, mut d, mut e) = (h0, h1, h2, h3, h4);
        for i in 0..80 {
            let (f, k) = if i < 20 {
                ((b & c) | ((!b) & d), 0x5a827999)
            } else if i < 40 {
                (b ^ c ^ d, 0x6ed9eba1)
            } else if i < 60 {
                ((b & c) | (b & d) | (c & d), 0x8f1bbcdc)
            } else {
                (b ^ c ^ d, 0xca62c1d6)
            };
            let t = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = t;
        }
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }
    format!("{:08x}{:08x}{:08x}{:08x}{:08x}", h0, h1, h2, h3, h4)
}

pub async fn signature(Query(q): Query<SignQuery>) -> Response {
    let appid = std::env::var("WX_APPID").unwrap_or_default();
    let secret = std::env::var("WX_APP_SECRET").unwrap_or_default();
    if appid.is_empty() || secret.is_empty() {
        return Json(json!({
            "ok": false,
            "code": "not_configured",
            "message": "尚未配置微信公众号 AppID / AppSecret"
        }))
        .into_response();
    }

    let url = q.url.unwrap_or_default();
    if url.is_empty() || !allowed_url(&url) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "ok": false, "code": "bad_url" })),
        )
            .into_response();
    }

    let ticket = match fetch_ticket(&appid, &secret) {
        Ok(t) => t,
        Err(e) => {
            return Json(json!({ "ok": false, "code": "upstream", "message": e })).into_response();
        }
    };

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let nonce = nonce_str();
    let raw = format!(
        "jsapi_ticket={}&noncestr={}&timestamp={}&url={}",
        ticket, nonce, ts, url
    );
    let signature = sha1_hex(&raw);

    Json(json!({
        "ok": true,
        "appId": appid,
        "timestamp": ts,
        "nonceStr": nonce,
        "signature": signature
    }))
    .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha1_matches_reference() {
        assert_eq!(sha1_hex("abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert_eq!(
            sha1_hex("The quick brown fox jumps over the lazy dog"),
            "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12"
        );
    }

    #[test]
    fn url_allow_list() {
        assert!(allowed_url("https://jshnp.com/"));
        assert!(allowed_url("https://wayhug.com/watch/demo?x=1"));
        assert!(allowed_url("http://localhost:3003/"));
        assert!(!allowed_url("https://evil.example/"));
        assert!(!allowed_url("https://jshnp.com.evil.example/"));
    }
}

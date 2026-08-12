use crate::*;

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

pub const MAX_CACHED_URLS: usize = 256;

#[derive(Deserialize)]
pub struct CachedImagesRequest {
    urls: Vec<String>,
}

fn parse_mirror_url(url: &str) -> Option<(ImageType, String)> {
    let path = match url.split_once('?') {
        Some((path, _)) => path,
        None => url,
    };
    let mut segments = path.rsplit('/').filter(|segment| !segment.is_empty());
    let base64_url = segments.next()?;
    if !base64_url
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'=')
    {
        return None;
    }
    let image_type = match segments.next() {
        Some("small") => ImageType::small(),
        Some("medium") => ImageType::medium(),
        _ => ImageType::Normal,
    };
    Some((image_type, base64_url.to_string()))
}

#[post("/cached")]
pub async fn handle_cached_images(request: web::Json<CachedImagesRequest>) -> HttpResponse {
    let urls = request.into_inner().urls;
    if urls.len() > MAX_CACHED_URLS {
        return HttpResponse::PayloadTooLarge().finish();
    }

    let mut cached = HashMap::with_capacity(urls.len());
    for url in urls {
        if cached.contains_key(&url) {
            continue;
        }
        let Some((image_type, base64_url)) = parse_mirror_url(&url) else {
            continue;
        };
        if !Path::new(&image_type.local_path(&base64_url)).exists() {
            continue;
        }
        cached.insert(url, image_type.extern_path(&base64_url));
    }

    HttpResponse::Ok().json(cached)
}

use std::hash::{DefaultHasher, Hash, Hasher};
use deterministic_hash::DeterministicHasher;
use macroquad::miniquad::conf::Icon;
use image::imageops::FilterType;
use macroquad::prelude::Conf;

pub fn load_icon_from_png(path: &str) -> Icon {
    let img = image::open(path)
        .expect("Failed to load icon PNG")
        .to_rgba8();

    fn resize(img: &image::RgbaImage, size: u32) -> Vec<u8> {
        let resized = image::imageops::resize(
            img,
            size,
            size,
            FilterType::Lanczos3,
        );
        resized.into_raw()
    }

    fn into_array<const N: usize>(v: Vec<u8>) -> [u8; N] {
        let mut arr = [0u8; N];
        arr.copy_from_slice(&v[..N]);
        arr
    }

    let small = into_array(resize(&img, 16));
    let medium = into_array(resize(&img, 32));
    let big = into_array(resize(&img, 64));

    Icon {
        small,
        medium,
        big,
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Crabsweeper".to_string(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        #[cfg(not(target_arch = "wasm32"))]
        icon: Some(load_icon_from_png("assets/icon64.png")),
        ..Default::default()
    }
}

pub fn hash<T: Hash>(value: &T) -> u64 {
    let hasher = DefaultHasher::new();
    let mut hasher = DeterministicHasher::new(hasher);
    value.hash(&mut hasher);
    hasher.finish()
}

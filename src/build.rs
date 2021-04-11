use std::{
    env, fmt, fs,
    fs::File,
    path::{Path, PathBuf}
};

const DRIVER_VERSION: &str = "1.11.0-next-1617207797000";
// playwright-python ea66054d

fn main() {
    let out_dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let dest = out_dir.join("driver.zip");
    let platform = PlaywrightPlatform::default();
    fs::write(out_dir.join("platform"), platform.to_string()).unwrap();
    download(&url(platform), &dest);
    println!("cargo:rerun-if-changed=src/build.rs");
}

#[cfg(not(feature = "only-for-docs-rs"))]
fn download(url: &str, dest: &Path) {
    let cache_dir: &Path = "/tmp/build-playwright-rust".as_ref();
    let cached = cache_dir.join("driver.zip");
    if cached.is_file() {
        fs::copy(cached, dest).unwrap();
        return;
    }
    let mut resp = reqwest::blocking::get(url).unwrap();
    let mut dest = File::create(dest).unwrap();
    if cfg!(debug_assertions) {
        fs::create_dir_all(cache_dir).ok();
        File::create(cached)
            .ok()
            .and_then(|mut cached| resp.copy_to(&mut cached).ok());
    }
    resp.copy_to(&mut dest).unwrap();
}

// No network access
#[cfg(feature = "only-for-docs-rs")]
fn download(_url: &str, dest: &Path) { File::create(dest).unwrap(); }

fn url(platform: PlaywrightPlatform) -> String {
    format!(
        "https://playwright.azureedge.net/builds/driver/\
        next/playwright-{}-{}.zip",
        DRIVER_VERSION, platform
    )
}

#[derive(Clone, Copy)]
enum PlaywrightPlatform {
    Linux,
    Win32,
    Win32x64,
    Mac
}

impl fmt::Display for PlaywrightPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linux => write!(f, "linux"),
            Self::Win32 => write!(f, "win32"),
            Self::Win32x64 => write!(f, "win32_x64"),
            Self::Mac => write!(f, "mac")
        }
    }
}

impl Default for PlaywrightPlatform {
    fn default() -> Self {
        if cfg!(target_os = "linux") {
            PlaywrightPlatform::Linux
        } else if cfg!(target_os = "macos") {
            PlaywrightPlatform::Mac
        } else if cfg!(windows) {
            if cfg!(target_pointer_width = "64") {
                PlaywrightPlatform::Win32x64
            } else {
                PlaywrightPlatform::Win32
            }
        } else if cfg!(unix) {
            PlaywrightPlatform::Linux
        } else {
            panic!("Unsupported plaform");
        }
    }
}

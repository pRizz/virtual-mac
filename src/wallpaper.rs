use leptos::prelude::*;

/// A wallpaper definition with id, name, and gradient
#[derive(Clone, Debug, PartialEq)]
pub struct Wallpaper {
    pub id: &'static str,
    pub name: &'static str,
    pub gradient: &'static str,
}

impl Wallpaper {
    pub const fn new(id: &'static str, name: &'static str, gradient: &'static str) -> Self {
        Self { id, name, gradient }
    }
}

/// Predefined wallpapers
pub const WALLPAPERS: &[Wallpaper] = &[
    Wallpaper::new(
        "sonoma-dark",
        "Sonoma Dark",
        "linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%)",
    ),
    Wallpaper::new(
        "sonoma-light",
        "Sonoma Light",
        "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    ),
    Wallpaper::new(
        "ventura",
        "Ventura",
        "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)",
    ),
    Wallpaper::new(
        "monterey",
        "Monterey",
        "linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)",
    ),
    Wallpaper::new(
        "big-sur",
        "Big Sur",
        "linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)",
    ),
    Wallpaper::new(
        "catalina",
        "Catalina",
        "linear-gradient(135deg, #fa709a 0%, #fee140 100%)",
    ),
    Wallpaper::new(
        "mojave-day",
        "Mojave Day",
        "linear-gradient(135deg, #ffecd2 0%, #fcb69f 100%)",
    ),
    Wallpaper::new(
        "mojave-night",
        "Mojave Night",
        "linear-gradient(135deg, #2c3e50 0%, #4ca1af 100%)",
    ),
    Wallpaper::new(
        "high-sierra",
        "High Sierra",
        "linear-gradient(135deg, #a8edea 0%, #fed6e3 100%)",
    ),
    Wallpaper::new(
        "sierra",
        "Sierra",
        "linear-gradient(135deg, #d299c2 0%, #fef9d7 100%)",
    ),
    Wallpaper::new(
        "el-capitan",
        "El Capitan",
        "linear-gradient(135deg, #89f7fe 0%, #66a6ff 100%)",
    ),
    Wallpaper::new(
        "yosemite",
        "Yosemite",
        "linear-gradient(135deg, #f6d365 0%, #fda085 100%)",
    ),
];

/// Context for managing wallpaper state
#[derive(Clone, Copy)]
pub struct WallpaperContext {
    pub current: ReadSignal<&'static str>,
    pub set_current: WriteSignal<&'static str>,
}

/// Provide wallpaper context to the application
pub fn provide_wallpaper_context() -> WallpaperContext {
    let (current, set_current) = signal(WALLPAPERS[0].id);
    let ctx = WallpaperContext {
        current,
        set_current,
    };
    provide_context(ctx);
    ctx
}

/// Get the wallpaper context
pub fn use_wallpaper_context() -> WallpaperContext {
    expect_context::<WallpaperContext>()
}

/// Get the gradient CSS for a wallpaper by id
pub fn get_wallpaper_gradient(id: &str) -> &'static str {
    WALLPAPERS
        .iter()
        .find(|w| w.id == id)
        .map(|w| w.gradient)
        .unwrap_or(WALLPAPERS[0].gradient)
}

#[derive(Clone, Debug)]
pub struct Config {
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            window_title: "Rustboy GB Emulator".into(),
        }
    }
}

pub struct Renderer {
    cfg: Config,

    sdl_ctx: sdl2::Sdl,
    canvas: sdl2::render::WindowCanvas,
}

#[derive(Clone, Debug)]
pub enum Error {
    SDL(String),
    SDLWindowBuildError(sdl2::video::WindowBuildError),
}

impl From<sdl2::IntegerOrSdlError> for Error {
    fn from(value: sdl2::IntegerOrSdlError) -> Self {
        match value {
            sdl2::IntegerOrSdlError::SdlError(err) => Self::SDL(err),
            sdl2::IntegerOrSdlError::IntegerOverflows(str, _) => Self::SDL(str.into()),
        }
    }
}

impl From<sdl2::video::WindowBuildError> for Error {
    fn from(value: sdl2::video::WindowBuildError) -> Self {
        Self::SDLWindowBuildError(value)
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::SDL(value)
    }
}

impl Renderer {
    pub fn new(cfg: Config) -> Result<Self, Error> {
        let sdl_ctx = sdl2::init()?;
        let video_subsystem = sdl_ctx.video()?;
        let window = video_subsystem
            .window(&cfg.window_title, cfg.window_width, cfg.window_height)
            .build()?;
        let mut canvas = window.into_canvas().build()?;

        canvas.clear();
        canvas.present();

        Ok(Self {
            cfg,
            sdl_ctx,
            canvas,
        })
    }
}

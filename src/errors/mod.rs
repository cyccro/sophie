use std::fmt::Display;

pub type ErrorId = u32;
#[derive(Debug)]
pub struct SophieError(ErrorId);
pub type SophieResult<T> = Result<T, SophieError>;

impl SophieError {
    //check list.txt to understand the logic
    pub fn new(id: ErrorId) -> Self {
        let err = Self(id);
        if cfg!(feature = "sph-spanic") {
            panic!("{err}");
        }
        err
    }
    pub fn is_info(&self) -> bool {
        (self.0 & 0xf00000) >> 20 == 2
    }
    fn priority(&self) -> u32 {
        (self.0 & 0xf00000) >> 20
    }
    fn priority_data<'a>(&self) -> (&'a str, u8) {
        //check ansi codes for color
        match self.priority() {
            0 => ("Sophie Error FATAL", 31),
            1 => ("Sophie Error WARN", 33),
            2 => ("Sophie INFO", 36),
            _ => ("Sophie TODO!", 37),
        }
    }
    fn second_quad(&self) -> u32 {
        (self.0 & 0x0f0000) >> 16
    }
    fn third_quad(&self) -> u32 {
        (self.0 & 0x00ff00) >> 8
    }
    fn last_quad(&self) -> u32 {
        self.0 & 0xff
    }
    fn get_third_error(code: u32) -> String {
        match code {
            1 => "Failed to create Wgpu shader",
            2 => "Failed to create Wgpu texture",
            3 => "Failed to create Sophie Perspective Camera",
            4 => "Failed to create Sophie Orthogonal Camera",
            _ => "Todo!",
        }
        .to_string()
    }
    fn get_reason(code: u32) -> String {
        match code {
            0 => "Invalid Path",
            1 => "Invalid UTF-8 encoding",
            2 => "Invalid Bytes",
            3 => "Value not None",
            _ => "Todo!",
        }
        .to_string()
    }
    fn get_error_content(&self) -> String {
        let last_quad = self.second_quad();
        match last_quad {
            0 => {
                let code = Self::get_third_error(self.third_quad());
                let reason = Self::get_reason(self.last_quad());
                format!("{code} due to: \x1b[35m{reason}\x1b[0m")
            }
            1 => "Failed to get Sdl2 display handle".to_string(),
            2 => "Failed to get Sdl2 window handle".to_string(),
            3 => "Failed to get Wgpu adapter".to_string(),
            4 => "Failed to get Wgpu device and queue".to_string(),
            _ => "TODO ERROR CONTENT!".to_string(),
        }
    }
}

impl Display for SophieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (priority, color) = self.priority_data();
        write!(
            f,
            "\x1b[{}m{}: {}\x1b[0m",
            color,
            priority,
            self.get_error_content()
        )
    }
}

pub trait Colorize {
    fn fg_red(self) -> String;
    fn fg_blue(self) -> String;
    fn fg_green(self) -> String;
    fn fg_magenta(self) -> String;

    fn bg_red(self) -> String;
    fn bg_blue(self) -> String;
    fn bg_green(self) -> String;
    fn bg_magenta(self) -> String;

    fn bold(self) -> String;
    fn italic(self) -> String;
    fn suppressed(self) -> String;
}

impl Colorize for String {
    fn fg_red(self) -> String {
        format!("\x1b[31m{}\x1b[39m", &self)
    }
    fn fg_blue(self) -> String {
        format!("\x1b[34m{}\x1b[39m", &self)
    }
    fn fg_green(self) -> String {
        format!("\x1b[32m{}\x1b[39m", &self)
    }
    fn fg_magenta(self) -> String {
        format!("\x1b[35m{}\x1b[39m", &self)
    }
    fn bg_red(self) -> String {
        format!("\x1b[41m{}\x1b[49m", &self)
    }
    fn bg_blue(self) -> String {
        format!("\x1b[44m{}\x1b[49m", &self)
    }
    fn bg_green(self) -> String {
        format!("\x1b[42m{}\x1b[49m", &self)
    }
    fn bg_magenta(self) -> String {
        format!("\x1b[45m{}\x1b[49m", &self)
    }
    fn bold(self) -> String {
        format!("\x1b[1m{}\x1b[22m", &self)
    }
    fn italic(self) -> String {
        format!("\x1b[3m{}\x1b[23m", &self)
    }
    fn suppressed(self) -> String {
        format!("\x1b[2m{}\x1b[22m", &self)
    }
}

impl Colorize for &str {
    fn fg_red(self) -> String {
        format!("\x1b[31m{}\x1b[39m", &self)
    }
    fn fg_blue(self) -> String {
        format!("\x1b[34m{}\x1b[39m", &self)
    }
    fn fg_green(self) -> String {
        format!("\x1b[32m{}\x1b[39m", &self)
    }
    fn fg_magenta(self) -> String {
        format!("\x1b[35m{}\x1b[39m", &self)
    }
    fn bg_red(self) -> String {
        format!("\x1b[41m{}\x1b[49m", &self)
    }
    fn bg_blue(self) -> String {
        format!("\x1b[44m{}\x1b[49m", &self)
    }
    fn bg_green(self) -> String {
        format!("\x1b[42m{}\x1b[49m", &self)
    }
    fn bg_magenta(self) -> String {
        format!("\x1b[45m{}\x1b[49m", &self)
    }
    fn bold(self) -> String {
        format!("\x1b[1m{}\x1b[22m", &self)
    }
    fn italic(self) -> String {
        format!("\x1b[3m{}\x1b[23m", &self)
    }
    fn suppressed(self) -> String {
        format!("\x1b[2m{}\x1b[22m", &self)
    }
}

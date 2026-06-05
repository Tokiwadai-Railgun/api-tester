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
        todo!()
    }

    fn fg_green(self) -> String {
        todo!()
    }

    fn fg_magenta(self) -> String {
        todo!()
    }

    fn bg_red(self) -> String {
        todo!()
    }

    fn bg_blue(self) -> String {
        todo!()
    }

    fn bg_green(self) -> String {
        todo!()
    }

    fn bg_magenta(self) -> String {
        todo!()
    }

    fn bold(self) -> String {
        todo!()
    }

    fn italic(self) -> String {
        todo!()
    }

    fn suppressed(self) -> String {
        todo!()
    }
}

// [Rust lifetime](inkdrop://note/vvsnY6B8)
impl Colorize for &str {
    fn fg_red(self) -> String {
        format!("\x1b[31m{}\x1b[39m", &self)
    }

    fn fg_blue(self) -> String {
        todo!()
    }

    fn fg_green(self) -> String {
        format!("\x1b[32m{}\x1b[39m", &self)
    }

    fn fg_magenta(self) -> String {
        todo!()
    }

    fn bg_red(self) -> String {
        todo!()
    }

    fn bg_blue(self) -> String {
        todo!()
    }

    fn bg_green(self) -> String {
        todo!()
    }

    fn bg_magenta(self) -> String {
        todo!()
    }

    fn bold(self) -> String {
        todo!()
    }

    fn italic(self) -> String {
        todo!()
    }

    fn suppressed(self) -> String {
        todo!()
    }
}

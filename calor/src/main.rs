
use std::fmt;
use std::convert::From;
use std::str::FromStr;
use std::string::String;
enum Color {
    Red,
    Yellow,
    Blue,
}

impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::Red)
    }
}

impl From<String> for Color {
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::Red)
    }
}

impl FromStr for Color {
    type Err = ();
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let src = src.to_lowercase();
        match src.as_ref() {
            "red" => Ok(Color::Red),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

impl Color {
    fn to_fg_str(&self) -> &str {
        match *self {
            Color::Red => "31",
            Color::Yellow => "33",
            Color::Blue => "34",
        }
    }

    fn to_bg_str(&self) -> &str {
        match *self {
            Color::Red => "41",
            Color::Yellow => "43",
            Color::Blue => "44",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
}

trait Colorize {
    // const FG_RED: &'static str = "31";
    // const BG_YELLOW: &'static str = "43";
    // fn red(self) -> ColoredString;
    // fn on_yellow(self) -> ColoredString;
    fn color<S: Into<Color>>(self, color: S) -> ColoredString;
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
}

impl Default for ColoredString {
    fn default () -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
        }
    }
}

impl<'a> Colorize for ColoredString {
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            ..self
        }
    }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
            ..self
        }
    }
    // fn red(self) -> ColoredString {
    //     ColoredString {
    //         fgcolor: String::from(ColoredString::FG_RED), ..self
    //     }
    // }

    // fn on_yellow(self) -> ColoredString {
    //     ColoredString {
    //         bgcolor: String::from(ColoredString::BG_YELLOW), ..self
    //     }
    // }
}

impl<'a> Colorize for &'a str {
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default
        }
    }

    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into),
            input: String::from(self),
            ..ColoredString::default()
        }
    }
    // fn red (self) -> ColoredString {
    //     ColoredString {
    //         fgcolor: String::from(ColoredString::FG_RED),
    //         input: String::from(self),
    //         ..ColoredString::default()
    //     }
    // }

    // fn on_yellow(self) -> ColoredString {
    //     ColoredString {
    //         bgcolor: String::from(ColoredString::BG_YELLOW),
    //         input: String::from(self),
    //         ..ColoredString::default()
    //     }
    // }
}

impl ColoredString {
    fn compute_style(&self) -> String {
        let mut res = String::from("\x1B[");
        let mut has_wrote = false;

        if let Some(ref bgcolor) = self.bgcolor {
            if has_wrote {
                res.push(';');
            }
            res.push_str(bgcolor.to_bg_str());
            has_wrote = true;
        }

        if let Some(ref fgcolor) = self.fgcolor {
            if has_wrote {
                res.push(';');
            }
            res.push_str(fgcolor.to_fg_str());
        }
        res.push('m');
        res
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut input = &self.input.clone();
        try!(f.write_str(&self.compute_style()));
        try!(f.write_str(input));
        try!(f.write_str("\x1B[0m"));
        Ok(())
    }
}

fn main() {
    let hi = "Hello".color("red").on_color("yello");
    println!("{}", hi);
}

#[derive(Clone)]
pub enum Theme {
    Default,
    Light,
    Dark,
}

impl ToString for Theme {
    fn to_string(&self) -> String {
        match self {
            Theme::Default => String::from(""),
            Theme::Light => String::from("light"),
            Theme::Dark => String::from("dark"),
        }
    }
}

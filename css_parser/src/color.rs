use std::default::Default;
use std::fmt;

#[derive(PartialEq, Clone)]
pub struct Color {
    pub original: &'static str,
    pub pattern: &'static str,
    pub data: ColorData,
}
#[derive(PartialEq, Clone, Debug)]
pub enum ColorData {
    RGBA { r: f32, g: f32, b: f32, a: f32 },
    HSLA { h: f32, s: f32, l: f32, a: f32 },
    NONE,
}

impl Color {
    pub fn none(color: &'static str) -> Self {
        Color {
            pattern: "none",
            data: ColorData::NONE,
            original: color,
        }
    }
    pub fn new(color: &'static str) -> Self {
        match color {
            color if color.len() < 4 => Color::none(color),
            color if color.starts_with("#") => match color.len() {
                9 => Color {
                    pattern: "#XXXXXXXX",
                    data: ColorData::RGBA {
                        r: hex_to_digit(
                            &std::iter::repeat(&color[1..3]).take(2).collect::<String>()[..],
                        ),
                        g: hex_to_digit(
                            &std::iter::repeat(&color[3..5]).take(2).collect::<String>()[..],
                        ),
                        b: hex_to_digit(
                            &std::iter::repeat(&color[5..7]).take(2).collect::<String>()[..],
                        ),
                        a: hex_to_digit(
                            &std::iter::repeat(&color[7..9]).take(2).collect::<String>()[..],
                        ) as f32
                            / 255.,
                    },
                    original: color,
                },
                7 => Color {
                    pattern: "#XXXXXX",
                    data: ColorData::RGBA {
                        r: hex_to_digit(&color[1..3]),
                        g: hex_to_digit(&color[3..5]),
                        b: hex_to_digit(&color[5..7]),
                        a: 1.,
                    },
                    original: color,
                },
                5 => Color {
                    pattern: "#XXXX",
                    data: ColorData::RGBA {
                        r: hex_to_digit(
                            &std::iter::repeat(&color[1..2]).take(2).collect::<String>()[..],
                        ),
                        g: hex_to_digit(
                            &std::iter::repeat(&color[2..3]).take(2).collect::<String>()[..],
                        ),
                        b: hex_to_digit(
                            &std::iter::repeat(&color[3..4]).take(2).collect::<String>()[..],
                        ),
                        a: hex_to_digit(
                            &std::iter::repeat(&color[4..5]).take(2).collect::<String>()[..],
                        ) as f32
                            / 255.,
                    },
                    original: color,
                },
                4 => Color {
                    pattern: "#XXX",
                    data: ColorData::RGBA {
                        r: hex_to_digit(
                            &std::iter::repeat(&color[1..2]).take(2).collect::<String>()[..],
                        ),
                        g: hex_to_digit(
                            &std::iter::repeat(&color[2..3]).take(2).collect::<String>()[..],
                        ),
                        b: hex_to_digit(
                            &std::iter::repeat(&color[3..4]).take(2).collect::<String>()[..],
                        ),
                        a: 1.,
                    },
                    original: color,
                },
                _ => Color::default(),
            },
            color if color.starts_with("rgba(") => {
                let channels: Vec<&str> = color
                    .trim_start_matches("rgba(")
                    .trim_end_matches(")")
                    .split(",")
                    .collect();

                match channels.len() {
                    size if size < 3 => Color::none(color),
                    _ => {
                        let color_channels: Vec<f32> = channels
                            .into_iter()
                            .map(|s| {
                                let mut s_trimmed = s.trim();
                                let has_percentage = s_trimmed.ends_with("%");
                                let num = match has_percentage {
                                    true => s_trimmed.trim_end_matches('%'),
                                    _ => s_trimmed,
                                }
                                .parse::<f32>()
                                .expect("Parsing of color channel failed: rgba");

                                match has_percentage {
                                    true => num / 100. * 255.,
                                    _ => num,
                                }
                            })
                            .collect();

                        Color {
                            pattern: "rgba",
                            data: ColorData::RGBA {
                                r: color_channels[0],
                                g: color_channels[1],
                                b: color_channels[2],
                                a: color_channels[3] / 255.,
                            },
                            original: color,
                        }
                    }
                }
            }
            color if color.starts_with("rgb(") => {
                let channels: Vec<&str> = color
                    .trim_start_matches("rgb(")
                    .trim_end_matches(")")
                    .split(",")
                    .collect();

                match channels.len() {
                    size if size < 3 => Color::none(color),
                    _ => {
                        let color_channels: Vec<f32> = channels
                            .into_iter()
                            .map(|s| {
                                let mut s_trimmed = s.trim();
                                let has_percentage = s_trimmed.ends_with("%");
                                let num = match has_percentage {
                                    true => s_trimmed.trim_end_matches('%'),
                                    _ => s_trimmed,
                                }
                                .parse::<f32>()
                                .unwrap();

                                match has_percentage {
                                    true => num / 100. * 255.,
                                    _ => num,
                                }
                            })
                            .collect();

                        Color {
                            pattern: "rgb",
                            data: ColorData::RGBA {
                                r: color_channels[0],
                                g: color_channels[1],
                                b: color_channels[2],
                                a: 1.,
                            },
                            original: color,
                        }
                    }
                }
            }
            color if color.starts_with("hsla(") => {
                let channels: Vec<&str> = color
                    .trim_start_matches("hsla(")
                    .trim_end_matches(")")
                    .split(",")
                    .collect();

                match channels.len() {
                    size if size < 3 => Color::none(color),
                    _ => {
                        let color_channels: Vec<&str> =
                            channels.into_iter().map(|s| s.trim()).collect();

                        Color {
                            pattern: "hsla",
                            data: ColorData::HSLA {
                                h: color_channels[0].parse::<f32>().unwrap(),
                                s: color_channels[1]
                                    .trim_end_matches("%")
                                    .parse::<f32>()
                                    .unwrap(),
                                l: color_channels[2]
                                    .trim_end_matches("%")
                                    .parse::<f32>()
                                    .unwrap(),
                                a: color_channels[3].trim().parse::<f32>().unwrap(),
                            },
                            original: color,
                        }
                    }
                }
            }
            color if color.starts_with("hsl(") => {
                let channels: Vec<&str> = color
                    .trim_start_matches("hsl(")
                    .trim_end_matches(")")
                    .split(",")
                    .collect();

                match channels.len() {
                    size if size < 3 => Color::none(color),
                    _ => {
                        let color_channels: Vec<&str> =
                            channels.into_iter().map(|s| s.trim()).collect();

                        Color {
                            pattern: "hsl",
                            data: ColorData::HSLA {
                                h: color_channels[0].parse::<f32>().unwrap(),
                                s: color_channels[1]
                                    .trim_end_matches("%")
                                    .parse::<f32>()
                                    .unwrap(),
                                l: color_channels[2]
                                    .trim_end_matches("%")
                                    .parse::<f32>()
                                    .unwrap(),
                                a: 1.,
                            },
                            original: color,
                        }
                    }
                }
            }
            _ => Color::none(color),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new("#000")
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nColor:\n");
        write!(f, "  pattern:  {:?}\n", self.pattern);
        write!(f, "  data:     {:?}\n", self.data);
        write!(f, "  original: {:?}\n", self.original)
    }
}

fn hex_to_digit(num: &str) -> f32 {
    (match u8::from_str_radix(num, 16) {
        Ok(n) => n,
        Err(_) => 0,
    }) as f32
}

fn repeat_str(s: &str, num: usize) -> String {
    std::iter::repeat(s).take(num).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hex() {
        let value = "#fff";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "#XXX",
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );

        let value = "#ffff";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "#XXXX",
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );

        let value = "#ffffff";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "#XXXXXX",
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_rgba() {
        let value = "rgba(255, 255, 255, 140)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "rgba",
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 0.54901963,
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );

        let value = "rgba(100%, 100%, 100%, 100%)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "rgba",
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_rgb() {
        let value = "rgb(255, 255, 255)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "rgb",
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );

        let value = "rgb(100%, 100%, 100%)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "rgb",
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_hsla() {
        let value = "hsla(22, 43%, 100%, 0.5)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "hsla",
                data: ColorData::HSLA {
                    h: 22.,
                    s: 43.,
                    l: 100.,
                    a: 0.5
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_hsl() {
        let value = "hsl(231, 34%, 12%)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "hsl",
                data: ColorData::HSLA {
                    h: 231.,
                    s: 34.,
                    l: 12.,
                    a: 1.
                },
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }

    #[test]
    fn parse_hex_incorrect() {
        let value = "#";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "none",
                data: ColorData::NONE,
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_rgba_incorrect() {
        let value = "rgba()";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "none",
                data: ColorData::NONE,
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_rgb_incorrect() {
        let value = "rgb";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "none",
                data: ColorData::NONE,
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_hsla_incorrect() {
        let value = "hsla(";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "none",
                data: ColorData::NONE,
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_hsl_incorrect() {
        let value = "hsl(";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "none",
                data: ColorData::NONE,
                original: value
            },
            "Parse \"{}\"",
            value
        );
    }
}

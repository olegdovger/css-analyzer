use std::default::Default;
use std::fmt;

#[derive(PartialEq, Clone)]
pub struct Color {
    pub original: String,
    pub pattern: String,
    pub data: ColorData,
}
#[derive(PartialEq, Clone, Debug)]
pub enum ColorData {
    RGBA { r: f32, g: f32, b: f32, a: f32 },
    HSLA { h: f32, s: f32, l: f32, a: f32 },
    NONE,
}

impl Color {
    fn original_to_hex(value: &str) -> &str {
        match value {
            "black" => "#000000",
            "silver" => "#c0c0c0",
            "gray" => "#808080",
            "white" => "#ffffff",
            "maroon" => "#800000",
            "red" => "#ff0000",
            "purple" => "#800080",
            "fuchsia" => "#ff00ff",
            "green" => "#008000",
            "lime" => "#00ff00",
            "olive" => "#808000",
            "yellow" => "#ffff00",
            "navy" => "#000080",
            "blue" => "#0000ff",
            "teal" => "#008080",
            "aqua" => "#00ffff",
            "orange" => "#ffa500",
            "aliceblue" => "#f0f8ff",
            "antiquewhite" => "#faebd7",
            "aquamarine" => "#7fffd4",
            "azure" => "#f0ffff",
            "beige" => "#f5f5dc",
            "bisque" => "#ffe4c4",
            "blanchedalmond" => "#ffebcd",
            "blueviolet" => "#8a2be2",
            "brown" => "#a52a2a",
            "burlywood" => "#deb887",
            "cadetblue" => "#5f9ea0",
            "chartreuse" => "#7fff00",
            "chocolate" => "#d2691e",
            "coral" => "#ff7f50",
            "cornflowerblue" => "#6495ed",
            "cornsilk" => "#fff8dc",
            "crimson" => "#dc143c",
            "cyan" => "#00ffff",
            "darkblue" => "#00008b",
            "darkcyan" => "#008b8b",
            "darkgoldenrod" => "#b8860b",
            "darkgray" => "#a9a9a9",
            "darkgreen" => "#006400",
            "darkgrey" => "#a9a9a9",
            "darkkhaki" => "#bdb76b",
            "darkmagenta" => "#8b008b",
            "darkolivegreen" => "#556b2f",
            "darkorange" => "#ff8c00",
            "darkorchid" => "#9932cc",
            "darkred" => "#8b0000",
            "darksalmon" => "#e9967a",
            "darkseagreen" => "#8fbc8f",
            "darkslateblue" => "#483d8b",
            "darkslategray" => "#2f4f4f",
            "darkslategrey" => "#2f4f4f",
            "darkturquoise" => "#00ced1",
            "darkviolet" => "#9400d3",
            "deeppink" => "#ff1493",
            "deepskyblue" => "#00bfff",
            "dimgray" => "#696969",
            "dimgrey" => "#696969",
            "dodgerblue" => "#1e90ff",
            "firebrick" => "#b22222",
            "floralwhite" => "#fffaf0",
            "forestgreen" => "#228b22",
            "gainsboro" => "#dcdcdc",
            "ghostwhite" => "#f8f8ff",
            "gold" => "#ffd700",
            "goldenrod" => "#daa520",
            "greenyellow" => "#adff2f",
            "grey" => "#808080",
            "honeydew" => "#f0fff0",
            "hotpink" => "#ff69b4",
            "indianred" => "#cd5c5c",
            "indigo" => "#4b0082",
            "ivory" => "#fffff0",
            "khaki" => "#f0e68c",
            "lavender" => "#e6e6fa",
            "lavenderblush" => "#fff0f5",
            "lawngreen" => "#7cfc00",
            "lemonchiffon" => "#fffacd",
            "lightblue" => "#add8e6",
            "lightcoral" => "#f08080",
            "lightcyan" => "#e0ffff",
            "lightgoldenrodyellow" => "#fafad2",
            "lightgray" => "#d3d3d3",
            "lightgreen" => "#90ee90",
            "lightgrey" => "#d3d3d3",
            "lightpink" => "#ffb6c1",
            "lightsalmon" => "#ffa07a",
            "lightseagreen" => "#20b2aa",
            "lightskyblue" => "#87cefa",
            "lightslategray" => "#778899",
            "lightslategrey" => "#778899",
            "lightsteelblue" => "#b0c4de",
            "lightyellow" => "#ffffe0",
            "limegreen" => "#32cd32",
            "linen" => "#faf0e6",
            "magenta" => "#ff00ff",
            "mediumaquamarine" => "#66cdaa",
            "mediumblue" => "#0000cd",
            "mediumorchid" => "#ba55d3",
            "mediumpurple" => "#9370db",
            "mediumseagreen" => "#3cb371",
            "mediumslateblue" => "#7b68ee",
            "mediumspringgreen" => "#00fa9a",
            "mediumturquoise" => "#48d1cc",
            "mediumvioletred" => "#c71585",
            "midnightblue" => "#191970",
            "mintcream" => "#f5fffa",
            "mistyrose" => "#ffe4e1",
            "moccasin" => "#ffe4b5",
            "navajowhite" => "#ffdead",
            "oldlace" => "#fdf5e6",
            "olivedrab" => "#6b8e23",
            "orangered" => "#ff4500",
            "orchid" => "#da70d6",
            "palegoldenrod" => "#eee8aa",
            "palegreen" => "#98fb98",
            "paleturquoise" => "#afeeee",
            "palevioletred" => "#db7093",
            "papayawhip" => "#ffefd5",
            "peachpuff" => "#ffdab9",
            "peru" => "#cd853f",
            "pink" => "#ffc0cb",
            "plum" => "#dda0dd",
            "powderblue" => "#b0e0e6",
            "rosybrown" => "#bc8f8f",
            "royalblue" => "#4169e1",
            "saddlebrown" => "#8b4513",
            "salmon" => "#fa8072",
            "sandybrown" => "#f4a460",
            "seagreen" => "#2e8b57",
            "seashell" => "#fff5ee",
            "sienna" => "#a0522d",
            "skyblue" => "#87ceeb",
            "slateblue" => "#6a5acd",
            "slategray" => "#708090",
            "slategrey" => "#708090",
            "snow" => "#fffafa",
            "springgreen" => "#00ff7f",
            "steelblue" => "#4682b4",
            "tan" => "#d2b48c",
            "thistle" => "#d8bfd8",
            "tomato" => "#ff6347",
            "turquoise" => "#40e0d0",
            "violet" => "#ee82ee",
            "wheat" => "#f5deb3",
            "whitesmoke" => "#f5f5f5",
            "yellowgreen" => "#9acd32",
            "rebeccapurple" => "#663399",
            _ => "#000000",
        }
    }

    fn original(original_color: &str) -> Self {
        let hex_color = Color::original_to_hex(original_color);

        Color {
            pattern: "#XXXXXX".to_string(),
            data: ColorData::RGBA {
                r: hex_to_digit(&hex_color[1..3]),
                g: hex_to_digit(&hex_color[3..5]),
                b: hex_to_digit(&hex_color[5..7]),
                a: 1.,
            },
            original: original_color.to_string(),
        }
    }
    pub fn none(color: &str) -> Self {
        Color {
            pattern: "none".to_string(),
            data: ColorData::NONE,
            original: color.to_string(),
        }
    }

    pub fn new(color: &str) -> Self {
        match color {
            color if color == "black" => Color::original("black"),
            color if color == "silver" => Color::original("silver"),
            color if color == "gray" => Color::original("gray"),
            color if color == "white" => Color::original("white"),
            color if color == "maroon" => Color::original("maroon"),
            color if color == "red" => Color::original("red"),
            color if color == "purple" => Color::original("purple"),
            color if color == "fuchsia" => Color::original("fuchsia"),
            color if color == "green" => Color::original("green"),
            color if color == "lime" => Color::original("lime"),
            color if color == "olive" => Color::original("olive"),
            color if color == "yellow" => Color::original("yellow"),
            color if color == "navy" => Color::original("navy"),
            color if color == "blue" => Color::original("blue"),
            color if color == "teal" => Color::original("teal"),
            color if color == "aqua" => Color::original("aqua"),
            color if color == "orange" => Color::original("orange"),
            color if color == "aliceblue" => Color::original("aliceblue"),

            color if color == "antiquewhite" => Color::original("antiquewhite"),
            color if color == "aquamarine" => Color::original("aquamarine"),
            color if color == "azure" => Color::original("azure"),
            color if color == "beige" => Color::original("beige"),
            color if color == "bisque" => Color::original("bisque"),
            color if color == "blanchedalmond" => Color::original("blanchedalmond"),
            color if color == "blueviolet" => Color::original("blueviolet"),
            color if color == "brown" => Color::original("brown"),
            color if color == "burlywood" => Color::original("burlywood"),
            color if color == "cadetblue" => Color::original("cadetblue"),
            color if color == "chartreuse" => Color::original("chartreuse"),
            color if color == "chocolate" => Color::original("chocolate"),
            color if color == "coral" => Color::original("coral"),
            color if color == "cornflowerblue" => Color::original("cornflowerblue"),
            color if color == "cornsilk" => Color::original("cornsilk"),
            color if color == "crimson" => Color::original("crimson"),
            color if color == "cyan" => Color::original("cyan"),
            color if color == "darkblue" => Color::original("darkblue"),
            color if color == "darkcyan" => Color::original("darkcyan"),
            color if color == "darkgoldenrod" => Color::original("darkgoldenrod"),
            color if color == "darkgray" => Color::original("darkgray"),
            color if color == "darkgreen" => Color::original("darkgreen"),
            color if color == "darkgrey" => Color::original("darkgrey"),
            color if color == "darkkhaki" => Color::original("darkkhaki"),
            color if color == "darkmagenta" => Color::original("darkmagenta"),
            color if color == "darkolivegreen" => Color::original("darkolivegreen"),
            color if color == "darkorange" => Color::original("darkorange"),
            color if color == "darkorchid" => Color::original("darkorchid"),
            color if color == "darkred" => Color::original("darkred"),
            color if color == "darksalmon" => Color::original("darksalmon"),
            color if color == "darkseagreen" => Color::original("darkseagreen"),
            color if color == "darkslateblue" => Color::original("darkslateblue"),
            color if color == "darkslategray" => Color::original("darkslategray"),
            color if color == "darkslategrey" => Color::original("darkslategrey"),
            color if color == "darkturquoise" => Color::original("darkturquoise"),
            color if color == "darkviolet" => Color::original("darkviolet"),
            color if color == "deeppink" => Color::original("deeppink"),
            color if color == "deepskyblue" => Color::original("deepskyblue"),
            color if color == "dimgray" => Color::original("dimgray"),
            color if color == "dimgrey" => Color::original("dimgrey"),
            color if color == "dodgerblue" => Color::original("dodgerblue"),
            color if color == "firebrick" => Color::original("firebrick"),
            color if color == "floralwhite" => Color::original("floralwhite"),
            color if color == "forestgreen" => Color::original("forestgreen"),
            color if color == "gainsboro" => Color::original("gainsboro"),
            color if color == "ghostwhite" => Color::original("ghostwhite"),
            color if color == "gold" => Color::original("gold"),
            color if color == "goldenrod" => Color::original("goldenrod"),
            color if color == "greenyellow" => Color::original("greenyellow"),
            color if color == "grey" => Color::original("grey"),
            color if color == "honeydew" => Color::original("honeydew"),
            color if color == "hotpink" => Color::original("hotpink"),
            color if color == "indianred" => Color::original("indianred"),
            color if color == "indigo" => Color::original("indigo"),
            color if color == "ivory" => Color::original("ivory"),
            color if color == "khaki" => Color::original("khaki"),
            color if color == "lavender" => Color::original("lavender"),
            color if color == "lavenderblush" => Color::original("lavenderblush"),
            color if color == "lawngreen" => Color::original("lawngreen"),
            color if color == "lemonchiffon" => Color::original("lemonchiffon"),
            color if color == "lightblue" => Color::original("lightblue"),
            color if color == "lightcoral" => Color::original("lightcoral"),
            color if color == "lightcyan" => Color::original("lightcyan"),
            color if color == "lightgoldenrodyellow" => Color::original("lightgoldenrodyellow"),
            color if color == "lightgray" => Color::original("lightgray"),
            color if color == "lightgreen" => Color::original("lightgreen"),
            color if color == "lightgrey" => Color::original("lightgrey"),
            color if color == "lightpink" => Color::original("lightpink"),
            color if color == "lightsalmon" => Color::original("lightsalmon"),
            color if color == "lightseagreen" => Color::original("lightseagreen"),
            color if color == "lightskyblue" => Color::original("lightskyblue"),
            color if color == "lightslategray" => Color::original("lightslategray"),
            color if color == "lightslategrey" => Color::original("lightslategrey"),
            color if color == "lightsteelblue" => Color::original("lightsteelblue"),
            color if color == "lightyellow" => Color::original("lightyellow"),
            color if color == "limegreen" => Color::original("limegreen"),
            color if color == "linen" => Color::original("linen"),
            color if color == "magenta" => Color::original("magenta"),
            color if color == "mediumaquamarine" => Color::original("mediumaquamarine"),
            color if color == "mediumblue" => Color::original("mediumblue"),
            color if color == "mediumorchid" => Color::original("mediumorchid"),
            color if color == "mediumpurple" => Color::original("mediumpurple"),
            color if color == "mediumseagreen" => Color::original("mediumseagreen"),
            color if color == "mediumslateblue" => Color::original("mediumslateblue"),
            color if color == "mediumspringgreen" => Color::original("mediumspringgreen"),
            color if color == "mediumturquoise" => Color::original("mediumturquoise"),
            color if color == "mediumvioletred" => Color::original("mediumvioletred"),
            color if color == "midnightblue" => Color::original("midnightblue"),
            color if color == "mintcream" => Color::original("mintcream"),
            color if color == "mistyrose" => Color::original("mistyrose"),
            color if color == "moccasin" => Color::original("moccasin"),
            color if color == "navajowhite" => Color::original("navajowhite"),
            color if color == "oldlace" => Color::original("oldlace"),
            color if color == "olivedrab" => Color::original("olivedrab"),
            color if color == "orangered" => Color::original("orangered"),
            color if color == "orchid" => Color::original("orchid"),
            color if color == "palegoldenrod" => Color::original("palegoldenrod"),
            color if color == "palegreen" => Color::original("palegreen"),
            color if color == "paleturquoise" => Color::original("paleturquoise"),
            color if color == "palevioletred" => Color::original("palevioletred"),
            color if color == "papayawhip" => Color::original("papayawhip"),
            color if color == "peachpuff" => Color::original("peachpuff"),
            color if color == "peru" => Color::original("peru"),
            color if color == "pink" => Color::original("pink"),
            color if color == "plum" => Color::original("plum"),
            color if color == "powderblue" => Color::original("powderblue"),
            color if color == "rosybrown" => Color::original("rosybrown"),
            color if color == "royalblue" => Color::original("royalblue"),
            color if color == "saddlebrown" => Color::original("saddlebrown"),
            color if color == "salmon" => Color::original("salmon"),
            color if color == "sandybrown" => Color::original("sandybrown"),
            color if color == "seagreen" => Color::original("seagreen"),
            color if color == "seashell" => Color::original("seashell"),
            color if color == "sienna" => Color::original("sienna"),
            color if color == "skyblue" => Color::original("skyblue"),
            color if color == "slateblue" => Color::original("slateblue"),
            color if color == "slategray" => Color::original("slategray"),
            color if color == "slategrey" => Color::original("slategrey"),
            color if color == "snow" => Color::original("snow"),
            color if color == "springgreen" => Color::original("springgreen"),
            color if color == "steelblue" => Color::original("steelblue"),
            color if color == "tan" => Color::original("tan"),
            color if color == "thistle" => Color::original("thistle"),
            color if color == "tomato" => Color::original("tomato"),
            color if color == "turquoise" => Color::original("turquoise"),
            color if color == "violet" => Color::original("violet"),
            color if color == "wheat" => Color::original("wheat"),
            color if color == "whitesmoke" => Color::original("whitesmoke"),
            color if color == "yellowgreen" => Color::original("yellowgreen"),
            color if color == "rebeccapurple" => Color::original("rebeccapurple"),

            color if color.len() < 4 => Color::none(color),

            color if color.starts_with("#") => match color.len() {
                9 => Color {
                    pattern: "#XXXXXXXX".to_string(),
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
                    original: color.to_string(),
                },
                7 => Color {
                    pattern: "#XXXXXX".to_string(),
                    data: ColorData::RGBA {
                        r: hex_to_digit(&color[1..3]),
                        g: hex_to_digit(&color[3..5]),
                        b: hex_to_digit(&color[5..7]),
                        a: 1.,
                    },
                    original: color.to_string(),
                },
                5 => Color {
                    pattern: "#XXXX".to_string(),
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
                    original: color.to_string(),
                },
                4 => Color {
                    pattern: "#XXX".to_string(),
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
                    original: color.to_string(),
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
                                let s_trimmed = s.trim();
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
                            pattern: "rgba".to_string(),
                            data: ColorData::RGBA {
                                r: color_channels[0],
                                g: color_channels[1],
                                b: color_channels[2],
                                a: color_channels[3] / 255.,
                            },
                            original: color.to_string(),
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
                                let s_trimmed = s.trim();
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
                            pattern: "rgb".to_string(),
                            data: ColorData::RGBA {
                                r: color_channels[0],
                                g: color_channels[1],
                                b: color_channels[2],
                                a: 1.,
                            },
                            original: color.to_string(),
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
                            pattern: "hsla".to_string(),
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
                            original: color.to_string(),
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
                            pattern: "hsl".to_string(),
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
                            original: color.to_string(),
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
        write!(
            f,
            "Color: pattern: {:?}, data: {:?}, original: {:?}",
            self.pattern, self.data, self.original
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.original)
    }
}

fn hex_to_digit(num: &str) -> f32 {
    (match u8::from_str_radix(num, 16) {
        Ok(n) => n,
        Err(_) => 0,
    }) as f32
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
                pattern: "#XXX".to_string(),
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value.to_string()
            },
            "Parse \"{}\"",
            value
        );

        let value = "#ffff";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "#XXXX".to_string(),
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value.to_string()
            },
            "Parse \"{}\"",
            value
        );

        let value = "#ffffff";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "#XXXXXX".to_string(),
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value.to_string()
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
                pattern: "rgba".to_string(),
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 0.54901963,
                },
                original: value.to_string()
            },
            "Parse \"{}\"",
            value
        );

        let value = "rgba(100%, 100%, 100%, 100%)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "rgba".to_string(),
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value.to_string()
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_rgb() {
        let value: &str = "rgb(255, 255, 255)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "rgb".to_string(),
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value.to_string()
            },
            "Parse \"{}\"",
            value
        );

        let value = "rgb(100%, 100%, 100%)";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "rgb".to_string(),
                data: ColorData::RGBA {
                    r: 255.,
                    g: 255.,
                    b: 255.,
                    a: 1.,
                },
                original: value.to_string()
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
                pattern: "hsla".to_string(),
                data: ColorData::HSLA {
                    h: 22.,
                    s: 43.,
                    l: 100.,
                    a: 0.5
                },
                original: value.to_string()
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
                pattern: "hsl".to_string(),
                data: ColorData::HSLA {
                    h: 231.,
                    s: 34.,
                    l: 12.,
                    a: 1.
                },
                original: value.to_string()
            },
            "Parse \"{}\"",
            value
        );
    }
    #[test]
    fn parse_named_color() {
        let colors = vec![
            ["black", "#000000"],
            ["silver", "#c0c0c0"],
            ["gray", "#808080"],
            ["white", "#ffffff"],
            ["maroon", "#800000"],
            ["red", "#ff0000"],
            ["purple", "#800080"],
            ["fuchsia", "#ff00ff"],
            ["green", "#008000"],
            ["lime", "#00ff00"],
            ["olive", "#808000"],
            ["yellow", "#ffff00"],
            ["navy", "#000080"],
            ["blue", "#0000ff"],
            ["teal", "#008080"],
            ["aqua", "#00ffff"],
            ["orange", "#ffa500"],
            ["aliceblue", "#f0f8ff"],
            ["antiquewhite", "#faebd7"],
            ["aquamarine", "#7fffd4"],
            ["azure", "#f0ffff"],
            ["beige", "#f5f5dc"],
            ["bisque", "#ffe4c4"],
            ["blanchedalmond", "#ffebcd"],
            ["blueviolet", "#8a2be2"],
            ["brown", "#a52a2a"],
            ["burlywood", "#deb887"],
            ["cadetblue", "#5f9ea0"],
            ["chartreuse", "#7fff00"],
            ["chocolate", "#d2691e"],
            ["coral", "#ff7f50"],
            ["cornflowerblue", "#6495ed"],
            ["cornsilk", "#fff8dc"],
            ["crimson", "#dc143c"],
            ["cyan", "#00ffff"],
            ["darkblue", "#00008b"],
            ["darkcyan", "#008b8b"],
            ["darkgoldenrod", "#b8860b"],
            ["darkgray", "#a9a9a9"],
            ["darkgreen", "#006400"],
            ["darkgrey", "#a9a9a9"],
            ["darkkhaki", "#bdb76b"],
            ["darkmagenta", "#8b008b"],
            ["darkolivegreen", "#556b2f"],
            ["darkorange", "#ff8c00"],
            ["darkorchid", "#9932cc"],
            ["darkred", "#8b0000"],
            ["darksalmon", "#e9967a"],
            ["darkseagreen", "#8fbc8f"],
            ["darkslateblue", "#483d8b"],
            ["darkslategray", "#2f4f4f"],
            ["darkslategrey", "#2f4f4f"],
            ["darkturquoise", "#00ced1"],
            ["darkviolet", "#9400d3"],
            ["deeppink", "#ff1493"],
            ["deepskyblue", "#00bfff"],
            ["dimgray", "#696969"],
            ["dimgrey", "#696969"],
            ["dodgerblue", "#1e90ff"],
            ["firebrick", "#b22222"],
            ["floralwhite", "#fffaf0"],
            ["forestgreen", "#228b22"],
            ["gainsboro", "#dcdcdc"],
            ["ghostwhite", "#f8f8ff"],
            ["gold", "#ffd700"],
            ["goldenrod", "#daa520"],
            ["greenyellow", "#adff2f"],
            ["grey", "#808080"],
            ["honeydew", "#f0fff0"],
            ["hotpink", "#ff69b4"],
            ["indianred", "#cd5c5c"],
            ["indigo", "#4b0082"],
            ["ivory", "#fffff0"],
            ["khaki", "#f0e68c"],
            ["lavender", "#e6e6fa"],
            ["lavenderblush", "#fff0f5"],
            ["lawngreen", "#7cfc00"],
            ["lemonchiffon", "#fffacd"],
            ["lightblue", "#add8e6"],
            ["lightcoral", "#f08080"],
            ["lightcyan", "#e0ffff"],
            ["lightgoldenrodyellow", "#fafad2"],
            ["lightgray", "#d3d3d3"],
            ["lightgreen", "#90ee90"],
            ["lightgrey", "#d3d3d3"],
            ["lightpink", "#ffb6c1"],
            ["lightsalmon", "#ffa07a"],
            ["lightseagreen", "#20b2aa"],
            ["lightskyblue", "#87cefa"],
            ["lightslategray", "#778899"],
            ["lightslategrey", "#778899"],
            ["lightsteelblue", "#b0c4de"],
            ["lightyellow", "#ffffe0"],
            ["limegreen", "#32cd32"],
            ["linen", "#faf0e6"],
            ["magenta", "#ff00ff"],
            ["mediumaquamarine", "#66cdaa"],
            ["mediumblue", "#0000cd"],
            ["mediumorchid", "#ba55d3"],
            ["mediumpurple", "#9370db"],
            ["mediumseagreen", "#3cb371"],
            ["mediumslateblue", "#7b68ee"],
            ["mediumspringgreen", "#00fa9a"],
            ["mediumturquoise", "#48d1cc"],
            ["mediumvioletred", "#c71585"],
            ["midnightblue", "#191970"],
            ["mintcream", "#f5fffa"],
            ["mistyrose", "#ffe4e1"],
            ["moccasin", "#ffe4b5"],
            ["navajowhite", "#ffdead"],
            ["oldlace", "#fdf5e6"],
            ["olivedrab", "#6b8e23"],
            ["orangered", "#ff4500"],
            ["orchid", "#da70d6"],
            ["palegoldenrod", "#eee8aa"],
            ["palegreen", "#98fb98"],
            ["paleturquoise", "#afeeee"],
            ["palevioletred", "#db7093"],
            ["papayawhip", "#ffefd5"],
            ["peachpuff", "#ffdab9"],
            ["peru", "#cd853f"],
            ["pink", "#ffc0cb"],
            ["plum", "#dda0dd"],
            ["powderblue", "#b0e0e6"],
            ["rosybrown", "#bc8f8f"],
            ["royalblue", "#4169e1"],
            ["saddlebrown", "#8b4513"],
            ["salmon", "#fa8072"],
            ["sandybrown", "#f4a460"],
            ["seagreen", "#2e8b57"],
            ["seashell", "#fff5ee"],
            ["sienna", "#a0522d"],
            ["skyblue", "#87ceeb"],
            ["slateblue", "#6a5acd"],
            ["slategray", "#708090"],
            ["slategrey", "#708090"],
            ["snow", "#fffafa"],
            ["springgreen", "#00ff7f"],
            ["steelblue", "#4682b4"],
            ["tan", "#d2b48c"],
            ["thistle", "#d8bfd8"],
            ["tomato", "#ff6347"],
            ["turquoise", "#40e0d0"],
            ["violet", "#ee82ee"],
            ["wheat", "#f5deb3"],
            ["whitesmoke", "#f5f5f5"],
            ["yellowgreen", "#9acd32"],
            ["rebeccapurple", "#663399"],
        ];

        for data in colors.iter() {
            let color = data[0];
            let hex = data[1];

            let original_color = Color::new(color);
            let hex_color = Color::new(hex);

            assert_eq!(
                original_color.data, hex_color.data,
                "Issue with color \"{}\"",
                color
            );
        }
    }

    #[test]
    fn parse_hex_incorrect() {
        let value = "#";

        assert_eq!(
            Color::new(value),
            Color {
                pattern: "none".to_string(),
                data: ColorData::NONE,
                original: value.to_string()
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
                pattern: "none".to_string(),
                data: ColorData::NONE,
                original: value.to_string()
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
                pattern: "none".to_string(),
                data: ColorData::NONE,
                original: value.to_string()
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
                pattern: "none".to_string(),
                data: ColorData::NONE,
                original: value.to_string()
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
                pattern: "none".to_string(),
                data: ColorData::NONE,
                original: value.to_string()
            },
            "Parse \"{}\"",
            value
        );
    }
}

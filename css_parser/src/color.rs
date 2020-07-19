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
            color if color == "black" => Color::new("#000000"),
            color if color == "silver" => Color::new("#c0c0c0"),
            color if color == "gray" => Color::new("#808080"),
            color if color == "white" => Color::new("#ffffff"),
            color if color == "maroon" => Color::new("#800000"),
            color if color == "red" => Color::new("#ff0000"),
            color if color == "purple" => Color::new("#800080"),
            color if color == "fuchsia" => Color::new("#ff00ff"),
            color if color == "green" => Color::new("#008000"),
            color if color == "lime" => Color::new("#00ff00"),
            color if color == "olive" => Color::new("#808000"),
            color if color == "yellow" => Color::new("#ffff00"),
            color if color == "navy" => Color::new("#000080"),
            color if color == "blue" => Color::new("#0000ff"),
            color if color == "teal" => Color::new("#008080"),
            color if color == "aqua" => Color::new("#00ffff"),
            color if color == "orange" => Color::new("#ffa500"),
            color if color == "aliceblue" => Color::new("#f0f8ff"),
            color if color == "antiquewhite" => Color::new("#faebd7"),
            color if color == "aquamarine" => Color::new("#7fffd4"),
            color if color == "azure" => Color::new("#f0ffff"),
            color if color == "beige" => Color::new("#f5f5dc"),
            color if color == "bisque" => Color::new("#ffe4c4"),
            color if color == "blanchedalmond" => Color::new("#ffebcd"),
            color if color == "blueviolet" => Color::new("#8a2be2"),
            color if color == "brown" => Color::new("#a52a2a"),
            color if color == "burlywood" => Color::new("#deb887"),
            color if color == "cadetblue" => Color::new("#5f9ea0"),
            color if color == "chartreuse" => Color::new("#7fff00"),
            color if color == "chocolate" => Color::new("#d2691e"),
            color if color == "coral" => Color::new("#ff7f50"),
            color if color == "cornflowerblue" => Color::new("#6495ed"),
            color if color == "cornsilk" => Color::new("#fff8dc"),
            color if color == "crimson" => Color::new("#dc143c"),
            color if color == "cyan" => Color::new("#00ffff"),
            color if color == "darkblue" => Color::new("#00008b"),
            color if color == "darkcyan" => Color::new("#008b8b"),
            color if color == "darkgoldenrod" => Color::new("#b8860b"),
            color if color == "darkgray" => Color::new("#a9a9a9"),
            color if color == "darkgreen" => Color::new("#006400"),
            color if color == "darkgrey" => Color::new("#a9a9a9"),
            color if color == "darkkhaki" => Color::new("#bdb76b"),
            color if color == "darkmagenta" => Color::new("#8b008b"),
            color if color == "darkolivegreen" => Color::new("#556b2f"),
            color if color == "darkorange" => Color::new("#ff8c00"),
            color if color == "darkorchid" => Color::new("#9932cc"),
            color if color == "darkred" => Color::new("#8b0000"),
            color if color == "darksalmon" => Color::new("#e9967a"),
            color if color == "darkseagreen" => Color::new("#8fbc8f"),
            color if color == "darkslateblue" => Color::new("#483d8b"),
            color if color == "darkslategray" => Color::new("#2f4f4f"),
            color if color == "darkslategrey" => Color::new("#2f4f4f"),
            color if color == "darkturquoise" => Color::new("#00ced1"),
            color if color == "darkviolet" => Color::new("#9400d3"),
            color if color == "deeppink" => Color::new("#ff1493"),
            color if color == "deepskyblue" => Color::new("#00bfff"),
            color if color == "dimgray" => Color::new("#696969"),
            color if color == "dimgrey" => Color::new("#696969"),
            color if color == "dodgerblue" => Color::new("#1e90ff"),
            color if color == "firebrick" => Color::new("#b22222"),
            color if color == "floralwhite" => Color::new("#fffaf0"),
            color if color == "forestgreen" => Color::new("#228b22"),
            color if color == "gainsboro" => Color::new("#dcdcdc"),
            color if color == "ghostwhite" => Color::new("#f8f8ff"),
            color if color == "gold" => Color::new("#ffd700"),
            color if color == "goldenrod" => Color::new("#daa520"),
            color if color == "greenyellow" => Color::new("#adff2f"),
            color if color == "grey" => Color::new("#808080"),
            color if color == "honeydew" => Color::new("#f0fff0"),
            color if color == "hotpink" => Color::new("#ff69b4"),
            color if color == "indianred" => Color::new("#cd5c5c"),
            color if color == "indigo" => Color::new("#4b0082"),
            color if color == "ivory" => Color::new("#fffff0"),
            color if color == "khaki" => Color::new("#f0e68c"),
            color if color == "lavender" => Color::new("#e6e6fa"),
            color if color == "lavenderblush" => Color::new("#fff0f5"),
            color if color == "lawngreen" => Color::new("#7cfc00"),
            color if color == "lemonchiffon" => Color::new("#fffacd"),
            color if color == "lightblue" => Color::new("#add8e6"),
            color if color == "lightcoral" => Color::new("#f08080"),
            color if color == "lightcyan" => Color::new("#e0ffff"),
            color if color == "lightgoldenrodyellow" => Color::new("#fafad2"),
            color if color == "lightgray" => Color::new("#d3d3d3"),
            color if color == "lightgreen" => Color::new("#90ee90"),
            color if color == "lightgrey" => Color::new("#d3d3d3"),
            color if color == "lightpink" => Color::new("#ffb6c1"),
            color if color == "lightsalmon" => Color::new("#ffa07a"),
            color if color == "lightseagreen" => Color::new("#20b2aa"),
            color if color == "lightskyblue" => Color::new("#87cefa"),
            color if color == "lightslategray" => Color::new("#778899"),
            color if color == "lightslategrey" => Color::new("#778899"),
            color if color == "lightsteelblue" => Color::new("#b0c4de"),
            color if color == "lightyellow" => Color::new("#ffffe0"),
            color if color == "limegreen" => Color::new("#32cd32"),
            color if color == "linen" => Color::new("#faf0e6"),
            color if color == "magenta" => Color::new("#ff00ff"),
            color if color == "mediumaquamarine" => Color::new("#66cdaa"),
            color if color == "mediumblue" => Color::new("#0000cd"),
            color if color == "mediumorchid" => Color::new("#ba55d3"),
            color if color == "mediumpurple" => Color::new("#9370db"),
            color if color == "mediumseagreen" => Color::new("#3cb371"),
            color if color == "mediumslateblue" => Color::new("#7b68ee"),
            color if color == "mediumspringgreen" => Color::new("#00fa9a"),
            color if color == "mediumturquoise" => Color::new("#48d1cc"),
            color if color == "mediumvioletred" => Color::new("#c71585"),
            color if color == "midnightblue" => Color::new("#191970"),
            color if color == "mintcream" => Color::new("#f5fffa"),
            color if color == "mistyrose" => Color::new("#ffe4e1"),
            color if color == "moccasin" => Color::new("#ffe4b5"),
            color if color == "navajowhite" => Color::new("#ffdead"),
            color if color == "oldlace" => Color::new("#fdf5e6"),
            color if color == "olivedrab" => Color::new("#6b8e23"),
            color if color == "orangered" => Color::new("#ff4500"),
            color if color == "orchid" => Color::new("#da70d6"),
            color if color == "palegoldenrod" => Color::new("#eee8aa"),
            color if color == "palegreen" => Color::new("#98fb98"),
            color if color == "paleturquoise" => Color::new("#afeeee"),
            color if color == "palevioletred" => Color::new("#db7093"),
            color if color == "papayawhip" => Color::new("#ffefd5"),
            color if color == "peachpuff" => Color::new("#ffdab9"),
            color if color == "peru" => Color::new("#cd853f"),
            color if color == "pink" => Color::new("#ffc0cb"),
            color if color == "plum" => Color::new("#dda0dd"),
            color if color == "powderblue" => Color::new("#b0e0e6"),
            color if color == "rosybrown" => Color::new("#bc8f8f"),
            color if color == "royalblue" => Color::new("#4169e1"),
            color if color == "saddlebrown" => Color::new("#8b4513"),
            color if color == "salmon" => Color::new("#fa8072"),
            color if color == "sandybrown" => Color::new("#f4a460"),
            color if color == "seagreen" => Color::new("#2e8b57"),
            color if color == "seashell" => Color::new("#fff5ee"),
            color if color == "sienna" => Color::new("#a0522d"),
            color if color == "skyblue" => Color::new("#87ceeb"),
            color if color == "slateblue" => Color::new("#6a5acd"),
            color if color == "slategray" => Color::new("#708090"),
            color if color == "slategrey" => Color::new("#708090"),
            color if color == "snow" => Color::new("#fffafa"),
            color if color == "springgreen" => Color::new("#00ff7f"),
            color if color == "steelblue" => Color::new("#4682b4"),
            color if color == "tan" => Color::new("#d2b48c"),
            color if color == "thistle" => Color::new("#d8bfd8"),
            color if color == "tomato" => Color::new("#ff6347"),
            color if color == "turquoise" => Color::new("#40e0d0"),
            color if color == "violet" => Color::new("#ee82ee"),
            color if color == "wheat" => Color::new("#f5deb3"),
            color if color == "whitesmoke" => Color::new("#f5f5f5"),
            color if color == "yellowgreen" => Color::new("#9acd32"),
            color if color == "rebeccapurple" => Color::new("#663399"),

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

            assert_eq!(
                Color::new(color),
                Color::new(hex),
                "Issue with colors \"{}\"",
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

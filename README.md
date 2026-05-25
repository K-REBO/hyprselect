# wmfocus - Visually focus windows by label

[![CI](https://github.com/K-REBO/wmfocus/workflows/CI/badge.svg)](https://github.com/K-REBO/wmfocus/actions)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/K-REBO/wmfocus/blob/master/LICENSE)
[![Stars](https://img.shields.io/github/stars/K-REBO/wmfocus.svg)](https://github.com/K-REBO/wmfocus/stargazers)

> **Fork of [svenstaro/wmfocus](https://github.com/svenstaro/wmfocus)** with active Wayland (Hyprland) support.

This tool allows you to rapidly choose a specific window directly without having to use the mouse or directional keyboard navigation.

**Supported window managers:**
- **Hyprland** (Wayland) — actively maintained in this fork
- i3
- sway (partial, accepting PRs)

![Screen cast](cast.apng)

Thanks to cairo, it should work on all kinds of screens and automatically display at the correct size according to your DPI.


## Installation

**With Cargo** (clone first):
```bash
git clone https://github.com/K-REBO/wmfocus.git
cd wmfocus
cargo install --features hyprland --path .  # Hyprland
cargo install --features i3 --path .        # i3
```

**With Nix** (flakes):
```bash
# Run directly (Hyprland version, default)
nix run github:K-REBO/wmfocus

# Run i3 version
nix run github:K-REBO/wmfocus#wmfocus-i3

# Install to profile
nix profile install github:K-REBO/wmfocus        # Hyprland
nix profile install github:K-REBO/wmfocus#wmfocus-i3  # i3
```

For NixOS/home-manager, add the overlay:
```nix
{
  inputs.wmfocus.url = "github:K-REBO/wmfocus";

  # In your configuration:
  nixpkgs.overlays = [ inputs.wmfocus.overlays.default ];
  environment.systemPackages = [ pkgs.wmfocus ];  # or pkgs.wmfocus-i3
}
```

## Usage

Draw labels on the upper-left corner of all windows:

    wmfocus

Completely fill out windows and draw the label in the middle (try it with transparency!):

    wmfocus --fill

Use a different font (as provided by fontconfig):

    wmfocus -f "Droid Sans":100

Change up the default colors:

    wmfocus --textcolor red --textcoloralt "#eeeeee" --bgcolor "rgba(50, 50, 200, 0.5)"

wmfocus will make use of a compositor to get real transparency.

## Full help
```
wmfocus 1.5.0

Bido Nakamura <bido@bido.dev>
Forked from svenstaro/wmfocus by Sven-Hendrik Haase <svenstaro@gmail.com>

Visually focus windows by label

USAGE:
    wmfocus [OPTIONS]

OPTIONS:
        --textcolor <TEXT_COLOR>                          Text color (CSS notation) [default: #dddddd]
        --textcoloralt <TEXT_COLOR_ALT>                   Text color alternate (CSS notation) [default: #666666]
        --bgcolor <BG_COLOR>                              Background color (CSS notation) [default: "rgba(30, 30, 30, 0.9)"]
        --textcolorcurrent <TEXT_COLOR_CURRENT>           Text color current window (CSS notation) [default: #333333]
        --textcolorcurrentalt <TEXT_COLOR_CURRENT_ALT>    Text color current window alternate (CSS notation) [default: #999999]
        --bgcolorcurrent <BG_COLOR_CURRENT>               Background color current window (CSS notation) [default: "rgba(200, 200, 200, 0.9)"]
        --halign <HORIZONTAL_ALIGN>                       Horizontal alignment of the box inside the window [default: left] [possible values: left, center, right]
        --valign <VERTICAL_ALIGN>                         Vertical alignment of the box inside the window [default: top] [possible values: top, center, bottom]
        --fill                                            Completely fill out windows
    -c, --chars <HINT_CHARS>                              Define a set of possbile values to use as hint characters [default: sadfjklewcmpgh]
    -e, --exit-keys <EXIT_KEYS>...                        List of keys to exit application, sequences separator is space, key separator is '+', eg Control_L+g
                                                          Shift_L+f
    -f, --font <FONT>                                     Use a specific TrueType font with this format: family:size [default: Mono:72]
    -h, --help                                            Print help information
    -m, --margin <MARGIN>                                 Add an additional margin around the text box (value is a factor of the box size) [default: 0.2]
    -o, --offset <OFFSET>                                 Offset box from edge of window relative to alignment (x,y) [default: 0,0]
    -p, --print-only                                      Print the window id only but don't change focus
    -V, --version                                         Print version information
```

## Troubleshooting

If there's some funky stuff, you can try to track it down by running `wmfocus` with `RUST_LOG=trace`:

    RUST_LOG=trace wmfocus

This will print quite some useful debugging info.


## Compiling

**For Hyprland**: You need to have recent versions of `rust`, `cargo`, `wayland-client`, `libxkbcommon` and `cairo` installed.

    git clone https://github.com/K-REBO/wmfocus.git
    cd wmfocus
    cargo run --features hyprland

**For i3**: You need to have recent versions of `rust`, `cargo`, `xcb-util-keysyms`, `libxkbcommon-x11` and `cairo` installed.

    git clone https://github.com/K-REBO/wmfocus.git
    cd wmfocus
    cargo run --features i3


## Contributing

If you want to implement support for more window managers, have a look at the [i3 implementation](https://github.com/K-REBO/wmfocus/blob/master/src/wm_i3.rs) or the [Hyprland implementation](https://github.com/K-REBO/wmfocus/blob/master/src/wm_hyprland.rs).

This tool is heavily inspired by [i3-easyfocus](https://github.com/cornerman/i3-easyfocus).

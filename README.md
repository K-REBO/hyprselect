# hyprselect - Visually focus windows by label

[![CI](https://github.com/K-REBO/hyprselect/workflows/CI/badge.svg)](https://github.com/K-REBO/hyprselect/actions)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/K-REBO/hyprselect/blob/master/LICENSE)
[![Stars](https://img.shields.io/github/stars/K-REBO/hyprselect.svg)](https://github.com/K-REBO/hyprselect/stargazers)

> **Fork of [svenstaro/wmfocus](https://github.com/svenstaro/wmfocus)** with active Wayland (Hyprland) support.

This tool allows you to rapidly choose a specific window directly without having to use the mouse or directional keyboard navigation.

**Supported window managers:**
- **Hyprland** (Wayland) — actively maintained in this fork
- i3

<video src="https://github.com/user-attachments/assets/9c5aeac1-f342-4704-ba78-18c4ed5cc5be" autoplay loop muted playsinline></video>

Thanks to cairo, it should work on all kinds of screens and automatically display at the correct size according to your DPI.


## Installation

**With Cargo**:
```bash
cargo install hyprselect --features hyprland  # Hyprland
cargo install hyprselect --features i3        # i3
```

**With Nix** (flakes):
```bash
# Run directly (Hyprland version, default)
nix run github:K-REBO/hyprselect

# Run i3 version
nix run github:K-REBO/hyprselect#hyprselect-i3

# Install to profile
nix profile install github:K-REBO/hyprselect        # Hyprland
nix profile install github:K-REBO/hyprselect#hyprselect-i3  # i3
```

For NixOS/home-manager, add the overlay:
```nix
{
  inputs.hyprselect.url = "github:K-REBO/hyprselect";

  # In your configuration:
  nixpkgs.overlays = [ inputs.hyprselect.overlays.default ];
  environment.systemPackages = [ pkgs.hyprselect ];  # or pkgs.hyprselect-i3
}
```

## Usage

Draw labels on the upper-left corner of all windows:

    hyprselect

Completely fill out windows and draw the label in the middle (try it with transparency!):

    hyprselect --fill

Use a different font (as provided by fontconfig):

    hyprselect -f "Droid Sans":100

Change up the default colors:

    hyprselect --textcolor red --textcoloralt "#eeeeee" --bgcolor "rgba(50, 50, 200, 0.5)"

hyprselect will make use of a compositor to get real transparency.

## Full help
```
Visually focus windows by label

Usage: hyprselect [OPTIONS]

Options:
  -f, --font <FONT>
          Use a specific TrueType font with this format: family:size [default: Mono:72]
  -c, --chars <HINT_CHARS>
          Define a set of possbile values to use as hint characters [default: sadfjklewcmpgh]
  -m, --margin <MARGIN>
          Add an additional margin around the text box (value is a factor of the box size) Format:
          single value or left,right,top,bottom [default: 0.2]
  -p, --print-only
          Print the window id only but don't change focus
  -o, --offset <OFFSET>
          Offset box from edge of window relative to alignment (x,y) [default: 0,0]
  -e, --exit-keys <EXIT_KEYS>
          List of keys to exit application, sequences separator is space, key separator is '+', eg
          Control_L+g Shift_L+f
  -s, --swap
          If this flag is set, the currently active window will swap with the selected window
      --textcolor <TEXT_COLOR>
          Text color (CSS notation) [default: #dddddd]
      --textcoloralt <TEXT_COLOR_ALT>
          Text color alternate (CSS notation) [default: #666666]
      --bgcolor <BG_COLOR>
          Background color (CSS notation) [default: "rgba(30, 30, 30, 0.9)"]
      --textcolorcurrent <TEXT_COLOR_CURRENT>
          Text color current window (CSS notation) [default: #333333]
      --textcolorcurrentalt <TEXT_COLOR_CURRENT_ALT>
          Text color current window alternate (CSS notation) [default: #999999]
      --bgcolorcurrent <BG_COLOR_CURRENT>
          Background color current window (CSS notation) [default: "rgba(200, 200, 200, 0.9)"]
      --halign <HORIZONTAL_ALIGN>
          Horizontal alignment of the box inside the window [default: left] [possible values: left,
          center, right]
      --valign <VERTICAL_ALIGN>
          Vertical alignment of the box inside the window [default: top] [possible values: top,
          center, bottom]
      --fill
          Completely fill out windows
  -h, --help
          Print help
  -V, --version
          Print version
```

## Troubleshooting

If there's some funky stuff, you can try to track it down by running `hyprselect` with `RUST_LOG=trace`:

    RUST_LOG=trace hyprselect

This will print quite some useful debugging info.


## Compiling

**For Hyprland**: You need to have recent versions of `rust`, `cargo`, `wayland-client`, `libxkbcommon` and `cairo` installed.

    git clone https://github.com/K-REBO/hyprselect.git
    cd hyprselect
    cargo run --features hyprland

**For i3**: You need to have recent versions of `rust`, `cargo`, `xcb-util-keysyms`, `libxkbcommon-x11` and `cairo` installed.

    git clone https://github.com/K-REBO/hyprselect.git
    cd hyprselect
    cargo run --features i3


## Contributing

If you want to implement support for more window managers, have a look at the [i3 implementation](https://github.com/K-REBO/hyprselect/blob/master/src/wm_i3.rs) or the [Hyprland implementation](https://github.com/K-REBO/hyprselect/blob/master/src/wm_hyprland.rs).

This tool is heavily inspired by [i3-easyfocus](https://github.com/cornerman/i3-easyfocus).

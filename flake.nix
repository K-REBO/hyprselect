{
  description = "Visually focus windows by label";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      pkgsFor = system: nixpkgs.legacyPackages.${system};

      mkWmfocus = { pkgs, features ? [ "hyprland" ] }:
        let
          hasI3 = builtins.elem "i3" features;
          hasWayland = builtins.elem "wayland" features || builtins.elem "hyprland" features;
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "wmfocus";
          version = "1.5.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          buildNoDefaultFeatures = true;
          buildFeatures = features;

          # Tests require all features to compile
          doCheck = false;

          nativeBuildInputs = with pkgs; [
            pkg-config
            cmake
          ];

          buildInputs = with pkgs; [
            cairo
            expat
            fontconfig
            freetype
            pango
            xorg.libX11
            xorg.libxcb
          ] ++ pkgs.lib.optionals hasWayland [
            wayland
            wayland-protocols
            libxkbcommon
          ];

          meta = with pkgs.lib; {
            description = "Visually focus windows by label";
            homepage = "https://github.com/svenstaro/wmfocus";
            license = licenses.mit;
            platforms = platforms.linux;
            mainProgram = "wmfocus";
          };
        };
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = pkgsFor system;
        in
        {
          default = self.packages.${system}.wmfocus-hyprland;

          wmfocus-hyprland = mkWmfocus {
            inherit pkgs;
            features = [ "hyprland" ];
          };

          wmfocus-i3 = mkWmfocus {
            inherit pkgs;
            features = [ "i3" ];
          };
        }
      );

      devShells = forAllSystems (system:
        let
          pkgs = pkgsFor system;
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.wmfocus-hyprland ];

            packages = with pkgs; [
              cargo
              rustc
              rust-analyzer
              clippy
              rustfmt
            ];

            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        }
      );

      overlays.default = final: prev: {
        wmfocus = self.packages.${prev.system}.default;
        wmfocus-i3 = self.packages.${prev.system}.wmfocus-i3;
        wmfocus-hyprland = self.packages.${prev.system}.wmfocus-hyprland;
      };
    };
}

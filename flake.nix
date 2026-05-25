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

      mkHyprselect = { pkgs, features ? [ "hyprland" ] }:
        let
          hasI3 = builtins.elem "i3" features;
          hasWayland = builtins.elem "wayland" features || builtins.elem "hyprland" features;
        in
        pkgs.rustPlatform.buildRustPackage {
          pname = "hyprselect";
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
            homepage = "https://github.com/K-REBO/hyprselect";
            license = licenses.mit;
            platforms = platforms.linux;
            mainProgram = "hyprselect";
          };
        };
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = pkgsFor system;
        in
        {
          default = self.packages.${system}.hyprselect-hyprland;

          hyprselect-hyprland = mkHyprselect {
            inherit pkgs;
            features = [ "hyprland" ];
          };

          hyprselect-i3 = mkHyprselect {
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
            inputsFrom = [ self.packages.${system}.hyprselect-hyprland ];

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
        hyprselect = self.packages.${prev.system}.default;
        hyprselect-i3 = self.packages.${prev.system}.hyprselect-i3;
        hyprselect-hyprland = self.packages.${prev.system}.hyprselect-hyprland;
      };
    };
}

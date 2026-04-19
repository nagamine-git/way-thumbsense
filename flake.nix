{
  description = "ThumbSense for Linux/Wayland — hold a virtual key while touching the trackpad";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      rec {
        packages.way-thumbsense = pkgs.rustPlatform.buildRustPackage {
          pname = "way-thumbsense";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          meta = with pkgs.lib; {
            description = "ThumbSense for Linux/Wayland";
            homepage = "https://github.com/nagamine-git/way-thumbsense";
            license = licenses.mit;
            platforms = platforms.linux;
            mainProgram = "way-thumbsense";
          };
        };

        packages.default = packages.way-thumbsense;

        apps.default = {
          type = "app";
          program = "${packages.way-thumbsense}/bin/way-thumbsense";
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ cargo rustc rustfmt clippy ];
        };
      });
}

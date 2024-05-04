{
  description = "Website";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs@{ self, nixpkgs, rust-overlay, ... }:
    let
      systems = [
        "aarch64-linux"
        "i686-linux"
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;

      overlays = import ./nix/overlays { inherit inputs; };

      pkgsFor = system: import nixpkgs {
        inherit system overlays;
      };
    in
    rec {
      packages = forAllSystems (system: import ./nix/pkgs {
        pkgs = pkgsFor system;
      });

      devShells = forAllSystems (system: rec {
        default = import ./nix/devshell {
          pkgs = pkgsFor system;
        };
      });
    };
}

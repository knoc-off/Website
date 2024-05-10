{ pkgs }:

pkgs.mkShell {
  commonBuildInputs = pkgs.commonBuildInputs;
  nativeBuildInputs = with pkgs; [
    cargo-edit
    cargo-generate
    (rust-bin.stable."1.76.0".default.override {
      extensions = [ "rust-src" ];
      targets = [ "wasm32-unknown-unknown" ];
    })
    rust-analyzer
    sccache
    pkg-config
    trunk
    tailwindcss
    binaryen
    dart-sass
    wasm-bindgen-cli
  ];
  buildInputs = [
    pkgs.super-tiny-icons # i need to figure out how to display icons from otf
  ];
  shellHook = ''
      cd nix/pkgs/portfolio
      rm icons
      ln -s ${pkgs.super-tiny-icons}/ icons

      ln -s ${pkgs.font-awesome}/share/fonts/opentype/ font-awesome
    '';
  RUST_BACKTRACE = 1;
}


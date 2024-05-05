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
    pkgs.super-tiny-icons
  ];
  shellHook = ''
    #mkdir -p icons
    #cp -r ${pkgs.super-tiny-icons}/* icons/
  '';
  RUST_BACKTRACE = 1;
}


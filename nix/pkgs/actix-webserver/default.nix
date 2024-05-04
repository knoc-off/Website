{ pkgs, rustPlatform }:

rustPlatform.buildRustPackage rec {
  pname = "app";
  version = "0.1.0";
  src = ./.;

  nativeBuildInputs = with pkgs; [
    (rust-bin.stable."1.76.0".default.override {
      extensions = [ "rust-src" ];
      targets = [ "wasm32-unknown-unknown" ];
    })
    pkg-config
    trunk
    binaryen
    dart-sass
    tailwindcss
    wasm-bindgen-cli
  ];

  buildInputs = pkgs.commonBuildInputs ++ [
    pkgs.super-tiny-icons
  ];

  cargoLock.lockFile = ./Cargo.lock;

  buildPhase = ''
    runHook preBuild
    mkdir -p $TMPDIR/output
    mkdir -p icons
    cp -r ${pkgs.super-tiny-icons}/* icons/
    trunk build --release --offline --dist $TMPDIR/output --public-url /
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mkdir -p $out/lib
    cp -r $TMPDIR/output/* $out/lib/
    runHook postInstall
  '';
}


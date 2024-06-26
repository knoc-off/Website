{ rustPlatform, pkgs, pkg-config }:
rustPlatform.buildRustPackage
{
  pname = "actix-webserver";
  version = "0.1.0";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  buildInputs = pkgs.commonBuildInputs ++ [
    pkgs.alsa-lib
    pkgs.udev
  ];

  nativeBuildInputs = with pkgs; [
    (rust-bin.stable."1.76.0".default.override {
      extensions = [ "rust-src" ];
    })
    pkg-config
    trunk
    binaryen
    dart-sass
    tailwindcss
    wasm-bindgen-cli
  ];



  postInstall = ''
    mkdir $out/lib
    mkdir $out/static
    # This defines the entry points of the website, and links together
    # the nix code, that builds trunk with the root directory.
    # and links together the rust webserver. so the data-flow is defined in nix

    # This will be all of the resources that the server needs to access to run
    ln -s ${pkgs.custom.portfolio}/lib/ $out/static/portfolio
  '';
}

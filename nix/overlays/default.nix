{ inputs }:
[
  (import ./common-build-inputs.nix)
  #(import ./config.nix)
  (import inputs.rust-overlay)
]


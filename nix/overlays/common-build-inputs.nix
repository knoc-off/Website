final: _prev: {
  commonBuildInputs = with final; [
    openssl.dev
    pkg-config
    zlib.dev
  ] ++ lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
    libiconv
    CoreServices
    SystemConfiguration
  ]);
}


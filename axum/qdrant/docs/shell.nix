let
  # Pin to stable from https://status.nixos.org/
  nixpkgs = import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/596a8e828c5dfa504f91918d0fa4152db3ab5502.tar.gz") {};
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "www_shell";
    buildInputs = with nixpkgs; [
      nodejs
      yarn
    ];
  }

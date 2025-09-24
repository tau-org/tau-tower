{
  description = "Tau webradio server - Nix flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        cranelib = crane.mkLib pkgs;
      in
    {
    packages.default = craneLib.buildPackage {
      src = craneLib.cleanCargoSource ./.;

      nativeBuildInput = with pkgs; [
        pkg-config
        rustPlatform.bindgenHook
      ];

      doCheck = false;
    };
  });

}

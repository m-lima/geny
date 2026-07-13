{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    flake-utils.url = "github:numtide/flake-utils";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
    helper.url = "github:m-lima/nix-template";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      helper,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      (helper.lib.rust.helper inputs system ./. {
        systemLinker = true;
        forceLibraryPath = true;
        buildInputs = pkgs: [
          pkgs.libglvnd
          pkgs.libx11
          pkgs.libxi
        ];
        nativeBuildInputs = pkgs: [
          pkgs.pkg-config
        ];
      }).outputs
    );
}

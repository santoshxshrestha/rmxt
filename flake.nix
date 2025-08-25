{
  description = "A Nix-flake-based Rust development environment";

  inputs = { nixpkgs.url = "github:NixOS/nixpkgs?ref=nixos-unstable"; };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          clippy
          rustfmt
          openssl
          rust-analyzer
          pkg-config
        ];

        shellHook = ''
          export RUST_SRC_PATH=${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}
        '';
      };
    };
}

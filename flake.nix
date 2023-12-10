{
  description = "EECS 240 Projects Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs"; # also valid: "nixpkgs"
    
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils,... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        
        # [HELP] Other variable definitions can go here
        
      in
        {
          # Development environment output
          devShell = pkgs.mkShell {
            # The Nix packages provided in the environment
            packages = with pkgs; [
              # Compiler
              llvmPackages_11.clang

              # Build utils
              bash     # The standard shell
              gnumake  # Make
              mold     # Faster linker
              pkg-config  # Package config (it is what it is)

              # Rust dev setup
              cargo
              rustc

              # Dev utils
              rust-analyzer # A moduler compiler frontend for the Rust language (used in VSCode)
            ];

            shellHook = ''
              export RUST_SRC_PATH="${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
              export MOLD_HOME="${pkgs.mold}"
            '';
          };
        }
    );
}

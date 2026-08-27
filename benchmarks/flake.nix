{
  nixConfig.bash-prompt = "[nix(develop):\\w]$ ";

  inputs = {
    nixpkgs.url = "git+https://gitlab+deploy-token-2663:gldt-iUzzjA9yUqfEzNcUM-5g@gitlab.lrz.de/aemy/tweed/mirrors/nixpkgs-mirror.git?ref=nixos-unstable";
    nixos-unstable.url = "git+https://gitlab+deploy-token-2663:gldt-iUzzjA9yUqfEzNcUM-5g@gitlab.lrz.de/aemy/tweed/mirrors/nixpkgs-mirror.git?ref=nixos-unstable";
    rust-overlay = {
      url = "git+https://gitlab+deploy-token-2661:gldt-fnxNgazf26aSZoB_DZJN@gitlab.lrz.de/aemy/tweed/mirrors/rust-overlay-mirror.git?ref=master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };


  outputs =
    {
      self,
      nixpkgs,
      nixos-unstable,
      rust-overlay,
    }:
    let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
    ];
  in {
    devShells = nixpkgs.lib.genAttrs systems (system:
      let
        overlays = [ (import rust-overlay) ];

        pkgs = import nixpkgs {
          inherit system;
          inherit overlays;
        };

        arm = import nixpkgs {
          localSystem = system;
          crossSystem = {
            config = "arm-none-eabihf";
            abi = "armv7e-m";
            libc = "newlib";
            gcc = {
              float-abi = "hard";
              fpu = "fpv4-sp-d16";
              cpu = "cortex-m4";
            };
          };
        };

        unstable = nixos-unstable.legacyPackages.${system};
      in {
        default = import ./shell.nix {
          inherit pkgs arm unstable;
        };
      });
  };
}

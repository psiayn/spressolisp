{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.spressolisp = naersk-lib.buildPackage {
        pname = "spressolisp";
        root = ./.;
      };
      defaultPackage = packages.spressolisp;

      # `nix run`
      apps.spressolisp = utils.lib.mkApp {
        drv = packages.spressolisp;
      };
      defaultApp = apps.spressolisp;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo rust-analyzer rustfmt ];
      };
    });
}

{
  description = "DevSlides — offline-first code presentation desktop app (a fork of OpenSlides)";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: let
    forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "x86_64-darwin" "aarch64-darwin" ];
  in {
    packages = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devslides = pkgs.callPackage ./nix/package.nix { };
      default = self.packages.${system}.devslides;
    });

    nixosModules.devslides = { pkgs, ... }: {
      environment.systemPackages = [ self.packages.${pkgs.system}.devslides ];
    };

    darwinModules.devslides = { pkgs, ... }: {
      environment.systemPackages = [ self.packages.${pkgs.system}.devslides ];
    };
  };
}

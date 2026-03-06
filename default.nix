{
  nixpkgs ? <nixpkgs>,
  pimalaya ? import (fetchTarball "https://github.com/pimalaya/nix/archive/master.tar.gz"),
  ...
}@args:

pimalaya.mkDefault (
  {
    src = ./.;
    version = "0.1.0";
    mkPackage = (
      {
        lib,
        pkgs,
        buildPackages,
        rustPlatform,
        defaultFeatures,
        features,
      }:
      rustPlatform.buildRustPackage {
        pname = "himalaya-tui";
        version = "0.1.0";
        src = ./.;

        buildNoDefaultFeatures = !defaultFeatures;
        buildFeatures = lib.splitString "," features;

        cargoLock = {
          lockFile = ./Cargo.lock;
          allowBuiltinFetchGit = true;
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        meta = with lib; {
          description = "TUI to manage emails";
          homepage = "https://pimalaya.org";
          license = licenses.mit;
          maintainers = with maintainers; [ ];
        };
      }
    );
  }
  // removeAttrs args [ "pimalaya" ]
)

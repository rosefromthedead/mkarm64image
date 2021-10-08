{
  inputs.flake-utils.url = "github:numtide/flake-utils";
  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      mkarm64image = pkgs.rustPlatform.buildRustPackage {
        pname = "mkarm64image";
        version = "0.1.0";
        src = ./.;

        cargoSha256 = "0kri0vv2m09rnwywh1dr2bhg1z5bkpxrvcswlawkyb19dqfh46xm";
        verifyDeps = true;
      };
    in
      rec {
        packages.mkarm64image = mkarm64image;
        defaultPackage = packages.mkarm64image;
      }
  );
}

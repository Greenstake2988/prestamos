{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          ruby_3_4
          bundler
          nodejs
          sqlite
          postgresql
          libpq
          libyaml
          libffi
          openssl
          zlib
          pkg-config
          gcc
          gnumake
          git
        ];

        shellHook = ''
          export GEM_HOME=$PWD/.gems
          export BUNDLE_PATH=$GEM_HOME
          export PATH=$GEM_HOME/bin:$PATH
        '';
      };
    };
}

let
  rust-overlay = builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/refs/tags/snapshot/2025-01-11.tar.gz";
    sha256 = "0p8qjk100jics1y4zqffkwy1crwz78ia9ilypaasfv94qm9jdpwa";
  };

  pkgs = import <nixpkgs> {
    overlays = [ (import rust-overlay) ];
  };

  # Impossible to add a sha256 due to the design of the script
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;

in
pkgs.mkShell rec {
  packages = [ toolchain ];

  nativeBuildInputs = [ pkgs.pkg-config ];

  buildInputs = with pkgs; [
    rustup

    xorg.libX11
    xorg.libXcursor
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
  '';
}

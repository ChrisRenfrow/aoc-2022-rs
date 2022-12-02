{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  nativeBuildInputs = [ pkg-config openssl.dev ];
  propagatedBuildInputs = [ ];
  buildInputs = [ ];
  shellHook = "";
  LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl.dev ];
}

# { pkgs ? import <nixpkgs> {} }:

{ pkgs ? import <nixpkgs> {  }
}:

pkgs.mkShell {

  
  # Packages needed at runtime by the software you are developing/building
  buildInputs = with pkgs; [
    # docker
    bluez
    dbus # If your software interacts with D-Bus
    # Other runtime dependencies
  ];

  # Build tools needed for compilation, including pkg-config
  nativeBuildInputs = with pkgs; [
    alsa-lib
    pkg-config
    # Other build tools like compilers, make, etc.
  ];
}
# RUST_LOG=trace

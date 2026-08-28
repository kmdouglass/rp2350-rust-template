let

  nixpkgs = fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/c27cdad491a991b11ed731760aa2ef8db0cb0410.tar.gz";
    sha256 = "1r58xn9xdka8bw710i431srl3dmy7dyrhd32rjv709f2mkb6m1ix";
  };

  rust-overlay = fetchTarball{
    url = "https://github.com/oxalica/rust-overlay/archive/dc2fd1acc537f3583744e1373597a5731ff7a6e3.tar.gz";
    sha256 = "1afpvg7m4jm7nf5algsd6mnfqww37rgvxlib4jfff81qgnmayx4d";
  };

  pkgs = import nixpkgs { config = {}; overlays = [ (import rust-overlay)]; };
in

pkgs.mkShellNoCC {
  name = "RP2350 Trigger Delay";
  
  packages = with pkgs; [
    probe-rs-tools
    (rust-bin.stable.latest.default.override {
      extensions = [
        "rust-src"
        "clippy"
        "rust-analyzer"
        "rust-fmt"
        "llvm-tools"
      ];
      targets = [
        "thumbv8m.main-none-eabihf"
      ];
    })
  ];
}

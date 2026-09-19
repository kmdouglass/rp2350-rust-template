let

  nixpkgs = fetchTarball {
    url = "https://github.com/NixOS/nixpkgs/archive/e554fab72f81915600f3f449b786fd9af40439a5.tar.gz";
    sha256 = "08qq3a6ry3sjm916cgwgd5421fddpk5jic8j4ssll9c8zmxm9a34";
  };

  rust-overlay = fetchTarball{
    url = "https://github.com/oxalica/rust-overlay/archive/26a71e661c47bd21a05d06fec749f3f7c75e9d12.tar.gz";
    sha256 = "06qfbyigdlazd1hpad515w5gdgjha7p5fic75c306sfqnq15yla1";
  };

  pkgs = import nixpkgs { config = {}; overlays = [ (import rust-overlay)]; };
in

pkgs.mkShellNoCC {
  name = "RP2350 Rust Template";
  
  packages = with pkgs; [
    probe-rs-tools
    (rust-bin.stable.latest.default.override {
      extensions = [
        "rust-src"
        "clippy"
        "rust-analyzer"
        "rustfmt"
        "llvm-tools"
      ];
      targets = [
        "thumbv8m.main-none-eabihf"
      ];
    })
  ];
}

{
  inputs = {
    geng.url = "github:geng-engine/geng";
  };
  outputs = { self, geng }: geng.makeFlakeOutputs (system:
    {
      src = geng.lib.${system}.filter {
        root = ./.;
        include = [
          "src"
          "assets"
          "Cargo.lock"
          "Cargo.toml"
        ];
      };
    });
}

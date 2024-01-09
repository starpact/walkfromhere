# WFH(walk from here)

`fd` but outputs files under the directory of the given file first.

## As a nix package

```nix
rustPlatform.buildRustPackage rec {
  pname = "wfh";
  version = "<commit SHA>";
  src = fetchFromGitHub {
    owner = "starpact";
    repo = "walkfromhere";
    rev = version;
    hash = "";
  };
  cargoHash = "";
}
```


## With `fzf-lua`

```lua
vim.keymap.set("n", "<leader>f", function()
  local buf_name = vim.api.nvim_buf_get_name(0)
  if buf_name == "" then
    fzf.files()
    return
  end

  fzf.files({
    cmd = "wfh " .. buf_name,
    fzf_opts = { ["--tiebreak"] = "index" },
  })
end)
```

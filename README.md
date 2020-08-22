# Linkforeman

Simple application to manage your config symlinks.
Create and fill out a `.linkforeman.toml` file like the example, and run `linkforeman status` to check their status.
The source paths are relative to the working directory, and the destination paths are relative to the user home directory.

## Example configuration file

```toml
[links]
bash = { src = "bash.sh", dest = ".bashrc" }
cargo = { src = "cargo.toml", dest = ".cargo/config" }
fish = { src = "fish", dest = ".config/fish" }
gdb = { src = "gdb", dest = ".gdbinit" }
git = { src = "git", dest = ".gitconfig" }
icie = { src = "icie", dest = ".config/icie" }
latex = { src = "latex/pustaczek.cls", dest = "/usr/local/share/texmf/tex/latex/local/pustaczek.cls" }
npm = { src = "npm", dest = ".npmrc" }
neovim = { src = "neovim.vim", dest = ".config/nvim/init.vim" }
rustfmt = { src = "rustfmt.toml", dest = ".rustfmt.toml" }
vscode = { src = "vscode.json", dest = ".config/Code/User/settings.json" }
yay = { src = "yay.sh", dest = ".config/yay/makepkg.conf" }
zathura = { src = "zathura", dest = ".config/zathura/zathurarc" }
```

# Linkforeman [![](https://img.shields.io/github/workflow/status/unneon/linkforeman/CI?logo=github-actions&logoColor=white)](https://github.com/unneon/linkforeman/actions) [![](https://img.shields.io/github/license/unneon/linkforeman?color=success&logo=github)](https://github.com/unneon/linkforeman)

Simple application to manage your config symlinks.
Create and fill out a `.linkforeman.toml` file like the example, and run `linkforeman status` to check their status.
The source paths are relative to the working directory, and the destination paths are relative to the user home directory.

## Example configuration file

```toml
[links]
# This is a symlink to the $PWD/bash.sh file at ~/.bashrc.
bash = { src = "bash.sh", dest = ".bashrc" }
# This is a symlink to the $PWD/icie/ directory at ~/.config/icie/.
icie = { src = "icie", dest = ".config/icie" }
# This is a symlink to the $PWD/nfs file at /etc/exports.
nfs = { src = "nfs", dest = "/etc/exports" }
# This generates symlinks to every $PWD/fish/* file at analogous ~/.config/fish/*.
fish = { src = "fish", dest = ".config/fish", recursive = true }
```

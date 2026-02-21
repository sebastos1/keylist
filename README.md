= Keylist

Keymappings from a yaml file:
```yml
config:
  font: JetBrains Mono Nerd Font
  # font_size: 16
  # theme: light
  mod: win
  columns: 2

keys:
  Modifier: [mod]
  General:
    Change window focus: [mod, [h, j, k, l]]
    Close a window: [mod, q]
    Move window: [mod, w]
    Move window with mouse: [mod, mouse_left]
    Resize with right click: [mod, mouse_right]
    Change workspace: [mod, [1,2,3]]
    Send window to workspace: [mod, shift, [1,2,3]]
    Toggle floating/tiling: [mod, shift, space]
    Toggle fullscreen: [mod, f]
  Launchers and menus:
    Open terminal: [mod, enter]
    Open the launcher: [mod, d]
```

== build
```sh
nix develop
cargo run -- -c binds.yaml
```




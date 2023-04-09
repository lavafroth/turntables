# archoxide
An (WIP) attempt to rewrite the archlinux installer in pure Rust

# Getting Started

Clone this repository and use cargo to run the project.

```
git clone git@github.com:lavafroth/archoxide
cd archoxide
cargo run
```

Use the up and down arrow keys to navigate the entries of a table or a list.
Use the left and right arrow keys to enter and leave a submenu respectively.

If no choices for a menu are selected, it will be marked as None. Enter the submenu
by pressing the right arrow key, use the up / down arrows to select an entry. Once
on the parent table by pressing left, you will see the selected value instead of None.

To quit the program, press q.

# TODO:

- [ ] Search functionality with the forward slash key
- [ ] Actually interacting with the system
- [ ] Macros to make the construction of recursive tables and lists look simpler
- [ ] `TerminalList`, a list structure with no lists as children
- [ ] Text input widget

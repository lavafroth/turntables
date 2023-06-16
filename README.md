# turntables (┛◉言◉)┛彡┻━┻

A simple, minimalist framework for recursive TUI tables 

### Getting Started

Clone this repository and use cargo to run the project.

```
git clone https://github.com/lavafroth/turntables.git
cd turntables
cargo run
```

Use the up and down arrow keys to navigate the entries of a table or a list.
Use the left and right arrow keys to enter and leave a submenu respectively.

If no choices for a menu are selected, it will be marked as None. Enter the submenu
by pressing the right arrow key, select an entry. Back in the main menu by
pressing left, the selected value will be shown instead of None.

To quit the program, press q.

### TODO:

- [ ] Search functionality with the forward slash key
- [ ] Actually interacting with the system
- [ ] Macros to make the construction of recursive tables and lists look simpler
- [ ] Text input widget

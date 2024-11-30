# rustc8
A Chip 8 emulator written in Rust for learning purpose. 
This emulator has been tested with various Chip-8 games, you can download these games from:
https://www.zophar.net/pdroms/chip8/chip-8-games-pack.html

## Installation
Ensure you have Rust and Cargo installed. Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/jasonshyang/rustc8.git
cd rustc8
cargo build --release
```

## Usage
To run a Chip-8 ROM:

```sh
cargo run --release -- <path_to_rom>
```

## Controls
The Chip-8 uses a hexadecimal keypad with the following layout:

```sh
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```

In this emulator, the keys are mapped to your keyboard as:

```sh
Keyboard Key -> Chip-8 Key
1            -> 1
2            -> 2
3            -> 3
4            -> C
Q            -> 4
W            -> 5
E            -> 6
R            -> D
A            -> 7
S            -> 8
D            -> 9
F            -> E
Z            -> A
X            -> 0
C            -> B
V            -> F
```

## Reference
http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#dispcoords
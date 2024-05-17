# The Chess Project
This is my take at implementing a chess game.<br>

***

My intention for this project is to create a good resource for beginners to chess programming, while learning myself the concepts needed for this.<br>

I have actually finished a chess game program before, in the ```niblit/chess``` repo, but honestly the code quality is not great, there are no comments or docs, and the UI code is awful, so I am creating this second version from the ground up to make it cleaner and (hopefully) understandable for everyone.<br>

## How to run it
- [Install rust](https://www.rust-lang.org/tools/install "rust-lang.org")
- Clone this repo
  - ```git clone https://github.com/niblit/TheChessProject```
- Run the project
  - ```cargo run```

## Project structure
The project is divided in crates, managed by cargo workspaces, I do it this way to have a clear distinction between UI and Game Logic code (and their dependencies).<br>

***

Efectively, the project contains a folder for each intention:
- ```logic```: contains everything needed to keep track of a complete chess game, **this is the core part of the project**.
- ```ui```: **TODO!** contains everything needed to play the game on your screen.
- ```tui```: **TODO!** contains everything needed to play the game on your terminal.
- ```engine```: **TODO!** contains everything needed to find the best move at a given position.

## Game representation
To keep track of a chess game, you need a lot of components:<br>

- Team representation

    > Location: ```logic/src/components/team.rs```<br>

    Using an enum containing black and white colors, this will be used for piece representation as well as keeping track of the player's turn.

- Piece type

    > Location: ```logic/src/components/piece_type.rs```<br>

    Using an enum for all 6 types of pieces:
    - King
    - Queen
    - Rook
    - Bishop
    - Knight
    - Pawn

- Piece representation:

    > Location: ```logic/src/components/piece.rs```<br>

    Using an struct containing both team and piece type enums, this is the complete piece representation as position will be stored implicitly in the board.

- Game outcomes:

    > Location: ```logic/src/components/outcome.rs```<br>

    This is, to the best of my knowledge, all posible ways a chess game can end, I am using an enum to represent it; the three non-drawing outcomes hold the player that won the game:
    - With a winning side
      - Checkmate
      - Resignation
      - Timeout
    - Results in draw
      - Stalemate
      - Insufficient material
      - Fifty-move Rule
      - Threefold repetition
      - By agreement

- Board representation:

    **TODO:** using a two dimensional array of ```Option<Piece>``` where ```None``` is an empty square and ```Some(Piece)``` is, well, a piece in the board; this is not the most efficient way to represent it, but I will keep it like this because it's easier to understand and fairly straight forward to implement.

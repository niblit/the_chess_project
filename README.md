# The Chess Project
This is my take at implementing a chess game.<br>

***

My intention for this project is to create a good resource for beginners to chess programming, while learning myself the concepts needed for this.<br>

I have actually finished a chess game program before, in the ```niblit/chess``` repo, but honestly the code quality is not great, there are no comments or docs, and the UI code is awful, so I am creating this second version from the ground up to make it cleaner and (hopefully) understandable for everyone.<br>

***

## Game representation
To keep track of a chess game, you need a lot of components:<br>

- Team representation

    > Location: ```logic/src/components/team.rs```<br>

    Using an enum, containig black and white colors, this will be used for piece representation as well as keeping track the player's turn.

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

    Using an struct, containing both team and piece type enums, this is the complete piece representation as position will be stored implicitly in the board.

- Game outcomes:

    > Location: ```logic/src/components/outcome.rs```<br>

    This is, to the best of my knowledge, all posible ways a chess game can end, I am using an enum to represent it, The three non-drawing outcomes hold the player that won the game:
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

    **TODO:** using a two dimensional array of ```Option<Piece>``` where ```None``` is an empty square and ```Some(Piece)``` is, well, a piece in the board, this is not the most efficient way to represent it, but I will keep it this way because is easier to understand and fairly straight forward to implement.

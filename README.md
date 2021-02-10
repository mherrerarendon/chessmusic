# Welcome to Chess Music!
This is a WIP rust sample project that produces music based on moves from a chess game. 

## How it works
There are two major modules in this project, chess and music. 
### Chess Module
The chess module parses chess games in PGN format. Once the moves are parsed, we are then able to get the "cell history" of each piece in the game. Because of the nature of the PGN format, getting "cell history" is not a trivial task. When the piece being moved is a rook, knight, or bishop, PGN only specifies which of the two pieces (Queen or King side) is being moved if the move is ambiguous (for example, if both knights are able to move to the cell, PGN would disambiguate which of the two knights is moved). This makes it difficult to know which piece is being moved in the case where PGN does not disambiguate. The solution is to determine possible valid moves per piece, which I only had to do for rooks, knights, and bishops, but for fun I implemented it for all pieces. 

### Music Module
Each piece is assigned an initial starting pitch. Then, the music module uses the "cell history" provided by the chess module, and assigns new pitches based on how each piece moves from cell to cell. For example, if the white A (file in chess board) pawn is assigned a starting pitch of A, and the pawn advances to the next cell, the new pitch assigned to this pawn would be a B.
that gets pgn formatted games from lichess, parses the moves, and then produces midi data based on the moves.
<br/>
Unit tests have been added, but haven't had a chance to setup code coverage yet. 

## How to build
Clone this repository and cd into it's root directory.
Run `cargo build` on a terminal to build, or `cargo test` to run unit tests.

## How to use
The plan is to take a Lichess game ID as a command line argument, but for now the game ID is hardcoded. So if you run the executable with no arguments you'll hear a game between my dad and me.
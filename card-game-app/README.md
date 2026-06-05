# Terminal Spider Solitaire

A terminal-based Spider Solitaire game built in Python using the standard library `curses` module, with zero external dependencies.

## Key Features
- **Responsive Terminal Design**: Supports 1-suit (Spades), 2-suit (Spades & Hearts), or 4-suit (standard) gameplay, rendering beautifully on any standard terminal.
- **Visual Card Highlighting**: Distinctive Red/White colors for card suits and full multi-card selection highlights.
- **Intuitive Keyboard Controls**: Smooth cursor-based navigation across tableau columns and vertical selection.
- **Full Undo/Redo Support**: Save states stored in a history stack so you can undo any mistake.
- **Non-interactive Smoke Test & Verification Suite**: Run automatic unit tests and logic validation completely headlessly.

## How to Play

Run the game using Python:

```bash
python3 card-game-app/main.py
```

Choose suits count (1, 2, or 4):
```bash
python3 card-game-app/main.py --suits 2
```

### Controls:
- **Left / Right Arrows**: Move between columns.
- **Up / Down Arrows**: Navigate up/down within faceup cards of the current column to select where to split/move a sequence.
- **Space / Enter**: Select the highlighted sequence / Place the selected sequence onto the target column.
- **0 - 9 Keys**: Jump directly to a column or attempt to place selected cards onto that column.
- **U / u**: Undo the last move.
- **D / d**: Deal 10 cards from the stock (one to each column). Standard rule: cannot deal if there are empty columns on the board.
- **R / r**: Restart the game.
- **Q / q**: Quit the game.
- **Esc (Escape)**: Cancel the current selection.

## Running Tests

To run the unit tests:
```bash
PYTHONPATH=card-game-app python3 -m unittest card-game-app/test_game.py
```

To run the non-interactive smoke test suite:
```bash
python3 card-game-app/main.py --smoke
```

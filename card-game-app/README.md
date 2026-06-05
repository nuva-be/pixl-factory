# Terminal Spider Solitaire

A terminal-based Spider Solitaire game built in Python using the standard library `curses` module, with zero external dependencies.

## Key Features
- **Responsive Terminal Design**: Supports 1-suit (Spades), 2-suit (Spades & Hearts), or 4-suit (standard) configurations, rendering beautifully on any standard terminal.
- **Visual Card Highlighting**: Distinctive Red/White colors for card suits and full multi-card selection highlights.
- **Intuitive Keyboard Controls**: Smooth cursor-based navigation across tableau columns and vertical selection.
- **Full Undo Support**: Save states stored in an undo stack so you can roll back any move.
- **Non-interactive Smoke Test & Verification Suite**: Run automatic unit tests and logic validation completely headlessly.

## Project Structure

```text
card-game-app/
├── pyproject.toml
├── README.md
├── main.py
├── src/
│   └── card_game_tui/
│       ├── __init__.py
│       ├── game.py
│       └── ui.py
└── tests/
    ├── __init__.py
    └── test_game.py
```

## How to Play

Run the game using Python:

```bash
python3 main.py
```

Choose suits count (1, 2, or 4):
```bash
python3 main.py --suits 2
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
python3 -m pytest tests/ -v
```

To run the non-interactive smoke test suite:
```bash
python3 main.py --smoke
```

To run the full suite (testing, syntax compilation, and smoke-testing):
```bash
python3 -m pytest tests/ -v && python3 -m py_compile main.py src/card_game_tui/*.py && python3 main.py --smoke
```

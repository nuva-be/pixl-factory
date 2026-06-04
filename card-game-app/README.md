# Terminal Klondike Solitaire (TUI)

A complete terminal-based Klondike Solitaire game in Python using the `curses` standard library.

## Features
- **Accurate Rules**: Draw-1 variation of Klondike Solitaire with stock recycling, tableau stack moves, and foundation stacking.
- **TUI Controls**: Intuitive grid navigation using arrows/WASD keys and simple selection mechanics.
- **Undo System**: Robust move history that lets you undo moves seamlessly with the 'U' key.
- **Auto-Play/Helper Actions**: Press 'A' to automatically play eligible cards from the waste or tableaus straight to the foundations.
- **Hints**: Stuck? Press 'H' to get a hint for a valid move currently available on the board.
- **Responsive Layout**: Adapts gracefully to window resizing, checking for minimum grid dimensions.

## Requirements
- Python 3.8+
- Standard Terminal with support for standard character colors and UTF-8 encoding (to render ♥, ♦, ♣, ♠).

## How to Play

### Installation & Run
To launch the interactive TUI game, run:
```bash
python3 main.py
```

### Controls
- **Arrow Keys / WASD**: Navigate the active cursor across the grid.
- **Space / Enter**: Select a card, stack, or pile, and confirm moves.
- **Esc / C**: Cancel active selection.
- **U**: Undo last action.
- **A**: Auto-play available cards to foundations.
- **H**: Request a valid move hint.
- **R**: Restart/re-shuffle.
- **Q**: Quit the game.

### Game Rules (Klondike)
- **Tableaus (T1 - T7)**: Stack cards downwards in descending ranks with alternating colors (e.g., Red Queen on Black King).
- **Foundations (F1 - F4)**: Build up each suit from Ace (1) to King (13).
- **Stock & Waste**: Draw cards from stock to waste. Recycle the waste pile when stock is depleted.
- **Winning**: Move all 52 cards to the foundations.

## Tests & Smoke Mode
To run the automated tests:
```bash
python3 -m pytest tests/ -v
```

To run the non-interactive smoke verification mode which renders text snapshots:
```bash
python3 main.py --smoke
```

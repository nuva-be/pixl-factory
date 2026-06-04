# FreeCell Solitaire TUI

An elegant, terminal-based FreeCell Solitaire game built in Python using the standard library `curses` module.

## Features

- **Standard FreeCell Rules**: 4 Free Cells, 4 Foundations, 8 Tableaus.
- **Auto-Solve Support**: Automatically moves safe cards (e.g. Aces, 2s, and cards whose lower-rank opposite color counter-parts are already solved) to the foundations.
- **Unlimited Undo**: Press `U` to rollback any moves.
- **Vibrant UI**: Color-coded cards (Red/Black), visual cursor navigation, selection highlighting, and clear status messages.
- **Help Panel**: Interactive help overlays rule definitions and keybindings.
- **Deterministic Seed Support**: Allows reproducible card shuffles for testing.
- **Automated Smoke Test**: Runs fully headless and verifies core game logic under `--smoke` mode.

## Installation & Running

### Requirements

- Python 3.10 or newer

### Setup

Install the package in developer mode (including dev dependencies for testing):

```bash
cd card-game-app
python3 -m pip install -e ".[dev]"
```

### Running the Game

To launch the interactive terminal interface:

```bash
python3 main.py
```

To run the headless smoke test:

```bash
python3 main.py --smoke
```

## How to Play

### Rules

1. Build **Tableaus** (columns 1-8) downwards in alternating colors (e.g., Red 9 on Black 10).
2. Build **Foundations** (top-right slots) upwards from Ace to King matching the respective suit.
3. Use the **Free Cells** (top-left slots) to temporarily hold any single card.
4. Moving a descending, alternating-color sequence of cards between tableaus is allowed, provided there are enough empty Free Cells and Tableaus to facilitate the transition.

### Controls

- **Navigation**: Arrow keys, `WASD`, or `HJKL`.
- **Select / Move**: `Space` or `Enter`.
  - Press on a source pile to select its bottom card (or sequence).
  - Press on a destination pile to execute the move.
  - Press on the same pile again to cancel the selection.
- **Undo**: `U`
- **Restart/New Game**: `N`
- **Help Menu**: `H`
- **Quit**: `Q` or `ESC`

## Running Tests

To execute the unit tests via `pytest`:

```bash
pytest tests/ -v
```

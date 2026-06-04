# Terminal FreeCell Solitaire in Python

A fully-featured, terminal-based FreeCell Solitaire game featuring an interactive curses user interface, full move validation, automatic safe card collection, and multi-step Undo.

## Features

- **Standard FreeCell Rules**: Fully validated moves, supporting cascade-to-cascade, freecell-to-cascade, cascade-to-foundation, and sequence moves.
- **Dynamic Sequence Moves**: Automatically calculates the maximum allowable sequence move size based on empty Free Cells and Cascades: $M = (F + 1) \times 2^E$.
- **Interactive Curses UI**: High-contrast color-coded cards (Red/Black), selected source highlighting, movement indicators, elapsed game timer, moves counter, and intuitive keyboard shortcuts.
- **Auto-Collect**: Scans and safely moves cards that are no longer needed for building sequences on the cascades to the foundations.
- **Full Undo/Redo (via Undo History)**: Revert any move (including its following auto-collects) instantly.
- **Deterministic Play (Seeds)**: Play random deals or specified numeric seeds for competitive play.
- **Automated Smoke Mode**: Verify game logic and rendering via a non-interactive `--smoke` check which performs a standard sequence of moves and outputs high-quality text snapshots of the board state.

## Project Structure

```text
card-game-app/
├── pyproject.toml              # Build system, metadata & pytest config
├── main.py                     # CLI entry point (handles interactive vs smoke mode)
├── README.md                   # Project documentation
├── src/
│   └── card_game_tui/
│       ├── __init__.py         # Package exports
│       ├── engine.py           # Core game state & move validations
│       └── ui.py               # Standard-library curses rendering & loops
└── tests/
    ├── __init__.py
    └── test_engine.py          # Comprehensive test cases for FreeCell rules
```

## Setup & Running

This game is self-contained and requires only a Python 3 environment.

### Prerequisites

- **Python**: `>= 3.8`

### Install Dependencies (For testing)

```bash
cd card-game-app
pip install -e .
```

### Run the Interactive Game

Launch the interactive curses terminal TUI:

```bash
python3 main.py
```

### Keyboard Shortcuts (Controls)

When running the interactive UI:
- **Move Cards**: Enter the **Source key**, followed by the **Destination key**:
  - **Free Cells**: `Q`, `W`, `E`, `R` (1st to 4th cell)
  - **Foundations**: `A`, `S`, `D`, `F` (Spades, Hearts, Diamonds, Clubs)
  - **Cascades**: `1` to `8` (columns 1 to 8)
- **Special Actions**:
  - `U`: **Undo** last move
  - `C` or `Space`: Manually trigger **Auto-Collect**
  - `R`: **Restart** current game (same layout and seed)
  - `N`: Start **New Game** with a random seed
  - `Q` or `Esc`: **Quit** the game

## Running Tests and Validation

### 1. Run Unit Tests (using Pytest)

```bash
python3 -m pytest tests/ -v
```

### 2. Verify Compilation (py_compile)

Check for any Python syntax or importing errors:

```bash
python3 -m py_compile main.py src/card_game_tui/*.py
```

### 3. Run Automated Smoke Test

Verify correctness and print text-based snapshot renderings without opening curses:

```bash
python3 main.py --smoke
```

# Terminal Klondike Solitaire Spec

This document details the specification and implementation plan for a terminal-based Klondike Solitaire game in Python 3.11+ using the standard `curses` library.

The application is structured to decouple core game rules and state from the presentation layer. This allows full testability of game mechanics without initializing a terminal.

---

## 1. Directory Structure

All files are located in `solitaire-app/`:

```
solitaire-app/
├── main.py        # Entry point: handles CLI args (including --smoke) and starts TUI
├── game.py        # Core Klondike Solitaire game model and rule engine (pure Python)
├── ui.py          # curses-based terminal user interface and input handler
└── test_game.py   # Automated unit tests for game logic
```

---

## 2. Core Game Engine (`game.py`)

The game rules are modeled strictly around standard **Draw-One Klondike Solitaire**.

### 2.1 Domain Entities

```python
from dataclasses import dataclass
from typing import List, Optional, Tuple

@dataclass
class Card:
    suit: str       # 'H' (Hearts), 'D' (Diamonds), 'C' (Clubs), 'S' (Spades)
    rank: int       # 1 (Ace) to 13 (King)
    is_face_up: bool = False

    @property
    def is_red(self) -> bool:
        return self.suit in ('H', 'D')

    @property
    def label(self) -> str:
        # e.g., "A", "2"..."10", "J", "Q", "K"
        ranks = {1: "A", 11: "J", 12: "Q", 13: "K"}
        return ranks.get(self.rank, str(self.rank))
```

### 2.2 Game State Model

`GameState` encapsulates all piles and tracks move history for **unlimited Undo** functionality:

- **Stock (`List[Card]`)**: Face-down draw pile.
- **Waste (`List[Card]`)**: Face-up drawn cards.
- **Foundations (`List[List[Card]]`)**: 4 piles, initially empty.
- **Tableau (`List[List[Card]]`)**: 7 piles. Tableau $i$ has $i$ cards, with top card face-up.
- **History (`List[dict]`)**: Deep copies or delta representations of prior states to support Undo.

### 2.3 Core Mechanics & Rules

- **Deal**: Shuffle a 52-card deck. Deal cards to Tableau columns (Col 1 has 1, Col 2 has 2... Col 7 has 7). Reveal the top card of each. Remaining 24 cards go to the Stock.
- **Draw**: Pop 1 card from Stock and append to Waste (face-up). If Stock is empty, recycle Waste by reversing it back to Stock (so that the original draw order is maintained).
- **Tableau-to-Tableau Moves**:
  - A card or a valid face-up build (stack of decreasing/alternating color cards) can be moved from column $A$ to column $B$.
  - Target column top card must be opposite color and exactly 1 rank higher.
  - If the target column is empty, only a King (rank 13) can be placed there.
- **Tableau-to-Foundation Moves**:
  - Move the top card of a Tableau column to a Foundation.
  - If the Foundation is empty, only an Ace (rank 1) of that suit is allowed.
  - If not empty, the card must be of the same suit and exactly 1 rank higher than the current top card of that Foundation.
- **Waste-to-Tableau/Foundation**:
  - Similar rules applied to the top card of the Waste pile.
- **Foundation-to-Tableau**:
  - Allows pulling a card back down from a Foundation pile to a Tableau column (following Tableau placement rules).
- **Auto-Flip**:
  - If a move exposes a face-down card at the top of a Tableau column, it must be automatically flipped face-up.
- **Win Condition**:
  - All 4 Foundation piles contain 13 cards (ending with Kings).

---

## 3. Terminal TUI Layer (`ui.py`)

The UI layer runs in `curses` using a grid-based navigation scheme.

### 3.1 Layout Design

The screen is divided into two main zones:

```
  [Stock]   [Waste]               [F1]   [F2]   [F3]   [F4]
   [ ]       [9♦]                 [A♥]   [ ]    [ ]    [ ]
  
  
  [Col 1]   [Col 2]   [Col 3]   [Col 4]   [Col 5]   [Col 6]   [Col 7]
   [10♣]     [ ]       [ ]       [ ]       [ ]       [ ]       [ ]
             [J♥]      [ ]       [ ]       [ ]       [ ]       [ ]
                       [Q♠]      [ ]       [ ]       [ ]       [ ]
```

### 3.2 Keyboard Navigation & Controls

We implement a keyboard-driven cursor navigation scheme:

1. **Cursor (Highlighted Element)**:
   - Use **Arrow keys** or Vim keys (`h`, `j`, `k`, `l`) to move the cursor.
   - Piles are arranged on a virtual grid:
     - Top row: Stock, Waste, Foundation 1, Foundation 2, Foundation 3, Foundation 4.
     - Bottom row: Tableau columns 1 to 7.
   - When cursor is on a Tableau column, pressing **Up** or **Down** moves the cursor within the face-up cards of that column. This allows selecting a specific card in a stack to move a partial build!

2. **Selecting and Moving**:
   - **Space** or **Enter**:
     - If the cursor is on the **Stock** pile: trigger a Draw operation.
     - If no card/pile is currently selected: select the current pile (and card index if in Tableau) as the **source**. The selection is visually highlighted.
     - If a **source** is already selected: move the selected card/stack from the source pile to the currently highlighted pile (the **target**). Then clear selection.
   - **Escape** or `c`: Cancel current selection.

3. **Global Shortcuts**:
   - `u` or `U`: Undo last move.
   - `r` or `R`: Restart game (re-shuffles and deals).
   - `q` or `Q`: Quit game.

### 3.3 Text & Color Configuration

- Red suits (Hearts/Diamonds) are rendered with red foreground text on appropriate backgrounds.
- Black suits (Clubs/Spades) are rendered with black or standard terminal color text.
- Face-down cards are rendered as `[ ]` or `[#]`.
- Face-up cards are rendered as `[10♦]`, `[A♥]`, `[K♠]`, `[Q♣]`, etc., using Unicode suit symbols or ASCII fallbacks (`H`, `D`, `C`, `S`).

---

## 4. Verification and Testing

### 4.1 Unit Testing (`test_game.py`)
- Standard Python unit tests using `unittest`.
- Covers deck setup, drawing, valid/invalid moves for all directions, auto-flip, and victory validation.
- Runs purely in CLI without loading `curses`.

### 4.2 Smoke Mode
- Run `python3 main.py --smoke`.
- The entry point parses `--smoke`, verifies all imports, initializes a mock terminal/session state, runs tests, and immediately exits with exit code `0`.
- Ensures zero-dependency execution environment checks.

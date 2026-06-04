# Spider Solitaire Terminal Game Implementation Plan

This plan outlines the architecture, rules, UI design, and testing strategy for a terminal-based Spider Solitaire game built with Python and the standard `curses` library. All source files will be located in the `card-game-app/` directory.

---

## 1. Game Rules & Logic (Spider Solitaire)

### Card & Deck Representation
- **Standard Spider Solitaire** uses **2 decks (104 cards)**.
- **Difficulty / Suit Configurations**:
  - **1 Suit (Easy)**: All cards are Spades (♠). (104 Spades)
  - **2 Suits (Medium)**: Spades (♠) and Hearts (♥). (52 Spades, 52 Hearts)
  - **4 Suits (Hard)**: Spades (♠), Hearts (♥), Diamonds (♦), and Clubs (♣). (26 of each)
- **Ranks**: King (K), Queen (Q), Jack (J), 10, 9, 8, 7, 6, 5, 4, 3, 2, Ace (A).

### Initial Deal / Tableau Setup
- **10 Tableau Columns**:
  - Columns 1-4: 6 cards each (5 face-down, 1 face-up at the bottom).
  - Columns 5-10: 5 cards each (4 face-down, 1 face-up at the bottom).
  - Total dealt initially: 54 cards.
- **Stock Pile**:
  - Remaining 50 cards are kept in the stock.
  - Dealt in 5 rounds of 10 cards each (1 card to each column).
  - **Constraint**: Dealing from the stock is only allowed if **no column is empty** (standard rule, though some variants allow dealing with empty columns; we will enforce standard rules or make it configurable).

### Card Movement Rules
- **Moving a Card or Sequence**:
  - Any single face-up card can be moved to another column if the destination card's rank is exactly **one higher** than the card being moved. Suit does not matter for single card moves. (e.g., Any Jack can be placed on any Queen).
  - A sequence of cards can be moved *together* only if:
    1. They are in descending rank order (e.g., J, 10, 9, 8).
    2. They are of the **same suit** (e.g., all Spades).
  - Any face-up card or valid sequence can be moved to an **empty column**.
- **Revealing Cards**:
  - If a move leaves a facedown card at the bottom of a column, that card is automatically flipped face-up.

### Clearing Sequences (Win Condition)
- When a complete sequence of King down to Ace (K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A) of the **same suit** is formed in a column, it is automatically removed from the Tableau and placed in the Completed pile.
- **Game Win**: When all 8 completed sequences (104 cards) are removed.
- **Game Loss**: No more valid moves, the stock is empty, and the board is in a locked/unplayable state. (Usually, the user decides to resign, but we can detect gridlock if needed).

---

## 2. Core Data Structures (`card-game-app/engine.py`)

We will design a clean, object-oriented state engine decouple-able from `curses` to facilitate unit testing and the `--smoke` non-interactive test run.

### `Card`
```python
class Card:
    def __init__(self, rank: int, suit: str, face_up: bool = False):
        self.rank = rank      # 1 (Ace) to 13 (King)
        self.suit = suit      # 'S' (Spades), 'H' (Hearts), 'D' (Diamonds), 'C' (Clubs)
        self.face_up = face_up
```

### `GameState`
- **`tableau`**: `List[List[Card]]` - 10 columns.
- **`stock`**: `List[Card]` - Decks/remaining cards.
- **`completed_sequences`**: `int` - Count of removed sequences (0 to 8).
- **`history`**: `List[Memento]` - For Undo functionality.
- **`score`**: `int` - Starts at 500. Each move subtracts 1 point. Completing a sequence adds 100 points.

### Key Operations
- `deal_initial()`: Shuffles and populates the tableau and stock.
- `deal_from_stock()`: Deals 1 card to each column.
- `can_move(from_col, card_idx, to_col)`: Validates if a move is legal.
- `move_cards(from_col, card_idx, to_col)`: Executes the move, flips newly exposed bottom cards, and automatically extracts completed sequences.
- `undo()`: Reverts the last state.
- `check_win()`: Returns `True` if `completed_sequences == 8`.

---

## 3. Terminal Rendering via Curses (`card-game-app/ui.py`)

Using the standard-library `curses` module, we will implement a full-screen, responsive interface.

### Layout Design
```
   [SPIDER SOLITAIRE]                       Score: 495   Moves: 5   Suits: 1-Suit (S)
   ==================================================================================
   Stock: [ [50] ]                      Completed: [K♠] [K♠] [ ] [ ] [ ] [ ] [ ] [ ]
   
   Col 1    Col 2    Col 3    Col 4    Col 5    Col 6    Col 7    Col 8    Col 9    Col 10
   -----    -----    -----    -----    -----    -----    -----    -----    -----    ------
   [ ]      [ ]      [ ]      [ ]      [ ]      [ ]      [ ]      [ ]      [ ]      [ ]
   [ ]      [ ]      [ ]      [ ]      [ ]      [ ]      [ ]      [ ]      [ ]      [ ]
   [ ]      [ ]      [ ]      [ ]      10♠      [ ]      [ ]      [ ]      [ ]      [ ]
   J♠       9♥       [ ]      [ ]               [ ]      [ ]      [ ]      [ ]      [ ]
   10♠      8♦       K♣       [ ]                        [ ]      [ ]      [ ]      [ ]
            7♦       Q♣                                  [ ]      [ ]      [ ]      [ ]
                                                         5♠       2♦       [ ]      [ ]
                                                                  A♦

   ==================================================================================
   Controls: [Arrow keys / Tab] Move cursor  [Space/Enter] Select card/column
             [S] Deal Stock  [U] Undo  [R] New Game  [Q] Quit
```

### Visual Representation of Cards
- Face-down card: `[░░░]` or blue block.
- Face-up card: Rank + Suit symbol. Examples: `A♠`, `10♥`, `Q♦`, `K♣`.
- Color schemes:
  - Spades/Clubs: White or default color.
  - Hearts/Diamonds: Red text (`curses.color_pair` with red foreground).
  - Selected card/sequence: Highlighted background (Reverse video or yellow background).

### Cursor & Selection Mechanics
- **Grid-based selection / Keyboard cursor**:
  - The player moves a cursor (highlighted cell or arrow pointer) across columns.
  - Pressing `SPACE` or `ENTER` on a column selects the deepest movable sequence.
  - Moving the cursor to another column and pressing `SPACE`/`ENTER` attempts the move.
- Alternative: Keyboard column shortcut keys (e.g., Press `1` through `0` to select source column, then press destination column). We will provide **both** cursor-based navigations and quick hotkeys for smooth UX.

---

## 4. Input Handling & Actions

| Input Key | Action |
| --- | --- |
| `LEFT` / `RIGHT` or `H` / `L` | Navigate left/right across columns |
| `UP` / `DOWN` or `K` / `J` | Navigate up/down within a column to select the starting card of a sequence |
| `SPACE` / `ENTER` | Select starting card of sequence / Drop sequence onto target column |
| `S` | Deal a round from stock |
| `U` | Undo last move |
| `R` | Restart / New Game (prompts for difficulty: 1, 2, or 4 suits) |
| `Q` / `ESC` | Exit game |

---

## 5. Non-Interactive Demo Verification (`--smoke`)

To satisfy the verification requirements without prompting for curses terminal initialization, `python3 main.py --smoke` will run a programmatic simulation of the solitaire game engine:
1. Initialize a 1-suit Spider solitaire game.
2. Verify the card count in columns (54) and stock (50).
3. Find a legal move in the initial dealt state, execute it, and verify that columns and score updated.
4. Deal from stock and verify stock size decreases by 10 and columns increase.
5. Perform an undo and verify correctness.
6. Print a JSON report of the execution status and exit with code `0`.

---

## 6. Testing Strategy

### Unit Tests (`card-game-app/test_engine.py`)
We will write lightweight and automated unit tests for:
- Card model initialization and representation.
- Complete deck shuffling and dealing proportions.
- Move validation rules (successes and various invalid move rejections).
- Automatic extraction and clearing of complete K-to-A sequences.
- Stock deals and its pre-requisites (no empty columns).
- Undo/redo correctness.

We can execute unit tests using standard library `unittest` or `pytest`:
`python3 -m unittest card-game-app/test_engine.py`

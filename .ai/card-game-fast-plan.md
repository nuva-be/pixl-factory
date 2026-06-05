# Implementation Plan - Terminal Spider Solitaire

A terminal-based Spider Solitaire game built in Python using the standard library `curses` module, with a decoupled architecture to allow both interactive gameplay and headless smoke-testing.

## 1. Game Rules & Data Structures

We implement standard Spider Solitaire.
- **Decks**: 2 decks (104 cards total).
- **Suits**: Support 1-suit (Spades), 2-suit (Spades, Hearts), or 4-suit (standard) configurations. 1-suit is the default and recommended for terminal play.
- **Tableau**: 10 columns.
  - Setup: First 4 columns get 6 cards (5 facedown, 1 faceup). Next 6 columns get 5 cards (4 facedown, 1 faceup).
  - The remaining 50 cards form the Stock.
- **Deals**: Dealing from Stock puts 1 card faceup on each of the 10 columns. Standard rule: Stock cannot be dealt if there are any empty columns.
- **Move Rules**:
  - A sequence of faceup cards can be moved if they are of the *same suit* and in *decreasing numerical order* (e.g., 9♠, 8♠, 7♠).
  - A sequence/card can be placed onto any faceup card of any suit that is exactly one rank higher (e.g., placing a 7♠ on an 8♥), or onto an empty column.
- **Clearing Runs**:
  - If a full same-suit sequence from King down to Ace (K, Q, J, 10, 9, 8, 7, 6, 5, 4, 3, 2, A) is formed in a column, it is cleared from the board and placed into the completed foundations.
- **Win Condition**: All 8 runs are cleared (104 cards).

### Data Models (`card-game-app/game.py`)
- `Card`:
  - `suit`: Str (e.g. `'♠'`, `'♥'`, `'♦'`, `'♣'`)
  - `rank`: Int (1 for Ace, 11 for Jack, 12 for Queen, 13 for King)
  - `face_up`: Bool
- `SpiderGame`:
  - `tableau`: List of 10 lists of `Card`
  - `stock`: List of `Card`
  - `completed_runs`: Int (0 to 8)
  - `undo_stack`: List of game state snapshots (memento pattern using deepcopy)
  - `suits_count`: Int (1, 2, or 4)

---

## 2. Terminal Rendering Approach (`curses`)

Using Python's standard-library `curses` module. To make it highly visually clear:
- **Card Styling**:
  - Hearts/Diamonds in Red.
  - Spades/Clubs in default/white.
  - Facedown cards represented with a distinct background/pattern like `[░░░]` or `[###]`.
  - Selected cards highlighted (reverse video/bold/yellow).
- **Layout & Positioning**:
  - We divide the terminal into columns. An 80-character terminal easily accommodates 10 columns of width 6 with 1-char gaps:
    `Column X = col_idx * 7 + 2`
  - Stacking cards: To fit long columns vertically, we stack cards. Only the top-most part of a card in a stack is rendered (e.g., `| 9♠|`), and the bottom card is drawn fully (e.g. `[ 9♠]`). This takes only 1 line per card plus 1 line for the bottom card!
- **Header/Footer**:
  - Header: Shows stock count, completed runs count, and current game mode (e.g., "1-Suit").
  - Footer: Interactive guide ("Arrows: Move, Enter: Select/Drop, U: Undo, D: Deal, R: Restart, Q: Quit").
  - Status/Error line for messages like "Invalid move!" or "Cannot deal with empty columns!".

---

## 3. Input Handling & Move/Action Validation

We use a simple state machine for the UI:
1. **IDLE State**:
   - Left/Right Arrows: Move column cursor (0-9).
   - Up/Down Arrows: Navigate *up* and *down* within the face-up cards of the current column to select where to split/move the sequence.
   - Enter/Space: Validate if the highlighted card and all cards below it form a valid same-suit decreasing sequence. If yes, transition to **SELECTED State** and save the selection.
   - `u` / `U`: Undo.
   - `d` / `D`: Deal stock.
   - `r` / `R`: Restart game.
   - `q` / `Q`: Quit.
2. **SELECTED State**:
   - Left/Right Arrows: Move destination column cursor (0-9).
   - Enter/Space: Attempt to move selected cards to the target column.
     - Validate destination card rank (must be selected card rank + 1, or column must be empty).
     - If valid: perform move, flip new top card if needed, check for completed run, push to undo stack, return to **IDLE State**.
     - If invalid: show error message, stay in SELECTED state (or escape).
   - Escape: Cancel selection, return to **IDLE State**.

---

## 4. Win/Loss Detection

- **Win**: Triggered when `completed_runs == 8`. A victory screen is shown.
- **Loss/Stuck**: There is no hard loss state in solitaire, but we can display a status message if no valid moves are possible on the board and the stock is empty.

---

## 5. UI Layout Diagram

```text
======================= SPIDER SOLITAIRE =======================
 Stock: [|||||] (50 cards left)                  Runs Completed: 0/8
================================================================
  Col 0   Col 1   Col 2   Col 3   Col 4   Col 5   Col 6   Col 7 ...
  |###|   |###|   |###|   |###|   |###|   |###|   |###|   |###|
  |###|   | 9♠|   | K♦|   |###|   | 5♣|   |###|   |###|   |###|
  | 8♥|   [ 8♠]   [ Q♦]   | 4♠|   [ 4♣]   | J♥|   | Q♠|   | 2♦|
  [ 7♥]                   [ 3♠]           [10♥]   [ J♠]   [ A♦]
                          [ 2♠]
                          [ A♠]
----------------------------------------------------------------
[Status: Selected 4 cards from Col 3. Choose target column...]
[Controls: Enter/Space: Place | Esc: Cancel]
```

---

## 6. Test Strategy

1. **Unit Tests** (`card-game-app/test_game.py`):
   - Test Card, Deck, and initial Tableau setup.
   - Test sequence validation (is sequence valid? is move valid?).
   - Test stock dealing and empty column constraints.
   - Test complete run detection and clearing.
   - Test undo functionality.
2. **Non-interactive Smoke Test** (`python3 main.py --smoke`):
   - Direct headless simulation of starting a game, performing a valid move, dealing a hand, and triggering undo, without invoking `curses`.
   - Returns exit code 0 on success.

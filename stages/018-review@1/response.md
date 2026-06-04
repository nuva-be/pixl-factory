I have successfully reviewed the complete FreeCell card game app against the technical specification in `.ai/card-game-spec.md`.

Here is the confirmation of all key items:
1. **Application Location**: Located in `card-game-app/`.
2. **Environment & Framework**: Built for Python 3.11+ using the standard library `curses` module for double-buffered visual rendering.
3. **Game Rules & Mechanics**: Fully implemented in a clean, decoupled pure-Python domain model (`card_game_tui.domain`).
   - Standard 4 Free Cells, 4 Foundations, and 8 Tableau columns layout dealing 52 cards deterministically from a given seed.
   - Alternating color, descending rank movement rules for tableaus (including empty column acceptance).
   - Sequence/multi-card movement sequence bounds checking with the exact capacity formula `(1 + Empty Free Cells) * 2 ^ (Empty Tableau Columns)` (correctly excluding the target column when empty).
   - Iterative auto-homing with safe criteria (opposite color suits must have all lower cards in foundation).
   - Unlimited undo/redo stack that seamlessly reverses and reapplies moves along with nested auto-homing actions.
4. **Keyboard Controls**: Fully documented in `card-game-app/README.md`. Features navigation via Arrows/WASD/HJKL, select/drop via Space/Enter, sequence move count adjustments via `+/-` (or `[`/`]`), Escape to cancel selection, `u`/`y` for Undo/Redo, restart, new game, and confirmable quit.
5. **Testing Suite**: All 24 tests run via `pytest` pass flawlessly.
6. **Smoke Mode**: Running `python3 main.py --smoke` executes a headless game initialization and validates basic operations without invoking a terminal window, exiting cleanly with code `0`.

The formal review has been saved to `.ai/card-game-review.md` and `status.json` has been updated to reflect the successful outcome. The app is complete and fully demo-ready!
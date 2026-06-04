I have successfully created the detailed specification and fully implemented the terminal-based Klondike Solitaire game in Python!

### Summary of Completed Deliverables:
1. **Detailed Specification** (`.ai/solitaire-spec.md`):
   - Fully documents directory structures, core logic rules, rendering coordinates, virtual-grid cursor navigation, keyboard control schemes, and test environments.
2. **Core Solitaire Game Engine** (`solitaire-app/game.py`):
   - Implements standard draw-one Klondike rules including 7 tableaus, 4 foundations, stock, waste, auto-reveals, and unlimited undo history tracking.
   - De-coupled from `curses` allowing robust independent testing.
3. **curses-based TUI Layer** (`solitaire-app/ui.py`):
   - A grid-based navigation scheme supporting arrow/Vim keys for visual navigation.
   - Supports card stacking, color coding for Red/Black cards, and visual selections.
   - Includes highly intuitive single-key **Auto-Move** (`a` key) to send valid cards directly to foundations.
4. **Interactive Entry Point and Smoke Mode** (`solitaire-app/main.py`):
   - Supports regular interactive play via `python3 main.py`.
   - Supports non-interactive smoke testing via `python3 main.py --smoke`, allowing tests to verify imports, initialization, and all rules without opening an interactive terminal screen.
5. **Rules Verification Suite** (`solitaire-app/test_game.py`):
   - 7 automated unit tests checking deal mechanics, drawing/recycling, move constraints, auto-reveals, undo history, and winning conditions.
6. **Task Status** (`status.json`):
   - Recorded `outcome=succeeded` at the workspace root.
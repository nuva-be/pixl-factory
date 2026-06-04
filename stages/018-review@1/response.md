I have thoroughly reviewed the terminal-based Klondike Solitaire app against the specifications in `.ai/solitaire-spec.md`, run the test suite, and verified that everything works perfectly.

### Key Confirmations
1. **App Location**: All application code, assets, and tests reside cleanly within `solitaire-app/`.
2. **Platform & Requirements**: Built with **Python 3.11+** utilizing the standard library's `curses` module for the terminal user interface.
3. **Klondike Rules Engine**: Correctly implements Draw-One Klondike rules (accurate deal, waste pile recycling with preserved original order, valid tableau-to-tableau/tableau-to-foundation/waste-to-any moves, foundation-to-tableau down-moves, automatic card flipping on tableau exposure, and victory detection).
4. **Keyboard Controls**: Features grid navigation via standard **Arrow keys** or Vim keys (`h`/`j`/`k`/`l`). Provides intra-tableau navigation for selecting partial stacks, selection highlight/cancel, auto-moving (`a`/`A`), unlimited undos (`u`/`U`), restart (`r`/`R`), in-game help (`?`), and quit (`q`/`Q`).
5. **Robust Test Suite**: All 20 tests pass successfully with `pytest`.
6. **Smoke Mode**: Running `python3 main.py --smoke` passes flawlessly in a non-interactive terminal environment.

### Deliverables Saved
*   **Review Report**: Written to `.ai/solitaire-review.md`.
*   **Execution Status**: Written to `status.json` at the workspace root, with `{"outcome": "succeeded"}` to indicate completeness and demo readiness.
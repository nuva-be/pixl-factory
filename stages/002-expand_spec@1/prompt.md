Goal: Build a terminal-based Klondike solitaire game in Python with a curses TUI


Goal: $goal

Create a detailed implementation spec for a Python Klondike solitaire TUI.

Hard requirements:
- Create the application under solitaire-app/.
- Use Python 3.11+ and the standard-library curses module for the playable UI.
- Implement draw-one Klondike with 7 tableau piles, 4 foundations, stock, and waste.
- Support keyboard-only play.
- Keep game rules testable without curses.
- Include a smoke mode so `python3 main.py --smoke` starts enough of the app to prove imports and setup without requiring an interactive terminal.

Write the spec to .ai/solitaire-spec.md.
Write status.json at workspace root: outcome=succeeded if the spec is complete, outcome=failed with failure_reason otherwise.
Goal: Quickly build a terminal-based Klondike solitaire game in Python with a curses TUI

## Completed stages
- **plan_app**: succeeded
  - Model: gemini-3.5-flash, 61.9k tokens in / 4.6k out
  - Files: /home/daytona/workspace/fabro/.ai/solitaire-fast-plan.md, /home/daytona/workspace/fabro/status.json


Read .ai/solitaire-fast-plan.md.

Build the complete app under solitaire-app/ in one focused pass:
- pyproject.toml
- main.py
- src/solitaire_tui/ package
- tests/ package
- README.md

Implement:
- Card, deck, pile, and GameState types
- Initial Klondike deal
- Move validation and execution
- Stock/waste draw and recycle
- Undo
- Win detection
- Curses UI with board rendering, keyboard navigation, help, new game, and quit
- --smoke mode that imports the app, creates a game, renders a text snapshot or summary, and exits without curses interaction

Run:
cd solitaire-app && python3 -m pytest tests/ -v && python3 -m py_compile main.py src/solitaire_tui/*.py && python3 main.py --smoke

Write status.json at workspace root: outcome=succeeded if the app builds, tests pass, and smoke mode works, outcome=failed with failure_reason otherwise.
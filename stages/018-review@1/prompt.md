Goal: Build a terminal-based FreeCell solitaire game in Python

## Completed stages
- **expand_spec**: succeeded
  - Model: gemini-3.5-flash, 85.9k tokens in / 8.3k out
  - Files: /home/daytona/workspace/fabro/.ai/card-game-spec.md, /home/daytona/workspace/fabro/status.json
- **impl_setup**: succeeded
  - Model: gemini-3.5-flash, 144.1k tokens in / 6.8k out
  - Files: /home/daytona/workspace/fabro/card-game-app/README.md, /home/daytona/workspace/fabro/card-game-app/main.py, /home/daytona/workspace/fabro/card-game-app/pyproject.toml, /home/daytona/workspace/fabro/card-game-app/src/card_game_tui/__init__.py, /home/daytona/workspace/fabro/card-game-app/src/card_game_tui/domain.py, /home/daytona/workspace/fabro/card-game-app/src/card_game_tui/tui.py, /home/daytona/workspace/fabro/card-game-app/tests/__init__.py, /home/daytona/workspace/fabro/card-game-app/tests/test_domain.py, /home/daytona/workspace/fabro/status.json
- **verify_setup**: succeeded
  - Model: gemini-3.5-flash, 185.9k tokens in / 2.2k out
  - Files: /home/daytona/workspace/fabro/.ai/verify_setup.md, /home/daytona/workspace/fabro/status.json
- **check_setup**: succeeded
- **impl_data**: succeeded
  - Model: gemini-3.5-flash, 179.9k tokens in / 32.0k out
  - Files: /home/daytona/workspace/fabro/card-game-app/src/card_game_tui/domain.py, /home/daytona/workspace/fabro/card-game-app/tests/test_domain.py, /home/daytona/workspace/fabro/status.json
- **verify_data**: succeeded
  - Model: gemini-3.5-flash, 140.2k tokens in / 3.2k out
  - Files: /home/daytona/workspace/fabro/.ai/verify_data.md, /home/daytona/workspace/fabro/status.json
- **check_data**: succeeded
- **impl_logic**: succeeded
  - Model: gemini-3.5-flash, 177.0k tokens in / 8.2k out
  - Files: /home/daytona/workspace/fabro/card-game-app/src/card_game_tui/domain.py, /home/daytona/workspace/fabro/card-game-app/tests/test_domain.py, /home/daytona/workspace/fabro/status.json
- **verify_logic**: succeeded
  - Model: gemini-3.5-flash, 151.0k tokens in / 10.0k out
  - Files: /home/daytona/workspace/fabro/.ai/verify_logic.md, /home/daytona/workspace/fabro/status.json
- **check_logic**: succeeded
- **impl_ui**: succeeded
  - Model: gemini-3.5-flash, 263.6k tokens in / 27.0k out
  - Files: /home/daytona/workspace/fabro/card-game-app/main.py, /home/daytona/workspace/fabro/card-game-app/src/card_game_tui/render.py, /home/daytona/workspace/fabro/card-game-app/src/card_game_tui/tui.py, /home/daytona/workspace/fabro/card-game-app/tests/test_render.py, /home/daytona/workspace/fabro/card-game-app/tests/test_tui.py, /home/daytona/workspace/fabro/status.json
- **verify_ui**: succeeded
  - Model: gemini-3.5-flash, 144.1k tokens in / 4.0k out
  - Files: /home/daytona/workspace/fabro/.ai/verify_ui.md, /home/daytona/workspace/fabro/card-game-app/README.md, /home/daytona/workspace/fabro/status.json
- **check_ui**: succeeded
- **impl_integration**: succeeded
  - Model: gemini-3.5-flash, 222.9k tokens in / 4.5k out
  - Files: /home/daytona/workspace/fabro/status.json
- **verify_integration**: succeeded
  - Model: gemini-3.5-flash, 131.8k tokens in / 2.1k out
  - Files: /home/daytona/workspace/fabro/.ai/verify_integration.md, /home/daytona/workspace/fabro/status.json
- **check_integration**: succeeded


Review the complete card game app against .ai/card-game-spec.md.

Confirm:
- The app is in card-game-app/
- It is Python 3.11+ and uses curses for the TUI
- The requested game rules are implemented correctly
- Keyboard controls are usable and documented
- Tests pass
- Smoke mode works without an interactive terminal

Run:
cd card-game-app && python3 -m pytest tests/ -v && python3 main.py --smoke

Write review to .ai/card-game-review.md.
Write status.json at workspace root: outcome=succeeded if the app is complete and demo-ready, outcome=failed with specific missing or broken items.
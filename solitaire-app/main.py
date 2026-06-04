import sys
import argparse
import unittest

def run_smoke_test() -> None:
    """Proves imports work, instantiates core classes, and runs self-tests."""
    print("Running Solitaire smoke tests...")
    
    # 1. Test Imports
    try:
        from game import Card, GameState
        from ui import SolitaireTUI
        print("✓ Successfully imported core modules (game, ui).")
    except Exception as e:
        print(f"✗ Failed to import core modules: {e}")
        sys.exit(1)

    # 2. Test instantiation of core logic
    try:
        game = GameState(seed=123)
        print(f"✓ GameState instantiated. Stock size: {len(game.stock)} cards.")
        
        tui = SolitaireTUI(game)
        print("✓ SolitaireTUI instantiated.")
    except Exception as e:
        print(f"✗ Failed to instantiate game components: {e}")
        sys.exit(1)

    # 3. Run full unit tests to confirm rule-engine validity
    print("Running automated unit tests...")
    loader = unittest.TestLoader()
    # Discover and run tests in the solitaire-app folder
    from test_game import TestSolitaireGame
    suite = loader.loadTestsFromTestCase(TestSolitaireGame)
    runner = unittest.TextTestRunner(verbosity=1)
    result = runner.run(suite)
    
    if result.wasSuccessful():
        print("✓ All automated rules unit tests passed successfully!")
    else:
        print("✗ Automated unit tests failed!")
        sys.exit(1)

    print("Smoke mode passed successfully.")
    sys.exit(0)

def main() -> None:
    parser = argparse.ArgumentParser(description="Terminal Klondike Solitaire")
    parser.add_argument(
        "--smoke",
        action="store_true",
        help="Run non-interactive smoke tests to verify imports, setup, and game rules."
    )
    args = parser.parse_args()

    if args.smoke:
        run_smoke_test()

    # Normal execution starts curses-based TUI
    import curses
    from game import GameState
    from ui import SolitaireTUI

    game = GameState()
    tui = SolitaireTUI(game)
    
    try:
        curses.wrapper(tui.run)
    except Exception as e:
        print(f"Error running solitaire TUI: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()

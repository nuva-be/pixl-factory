#!/usr/bin/env python3
import sys
import argparse
from pathlib import Path

# Add src/ directory to the library path
src_path = Path(__file__).parent / "src"
sys.path.insert(0, str(src_path))

from card_game_tui.game import SpiderGame, Card

def run_smoke_test():
    """Runs a non-interactive verification suite to confirm game rules and states."""
    print("====================================================")
    print("Running Spider Solitaire Smoke Test / Verification...")
    print("====================================================")

    # 1. Initialize Game
    print("Initializing 1-Suit Spider game...")
    game = SpiderGame(suits_count=1)
    
    # 2. Check Tableau and Stock Counts
    assert len(game.stock) == 50, f"Expected 50 stock cards, got {len(game.stock)}"
    tableau_cards = sum(len(col) for col in game.tableau)
    assert tableau_cards == 54, f"Expected 54 cards in tableau, got {tableau_cards}"
    print("[PASSED] Game initialized correctly with 54 cards in tableau and 50 in stock.")

    # 3. Simulate and verify sequence validation
    print("Verifying sequence validation rules...")
    c1 = Card(5, '♠', True)
    c2 = Card(4, '♠', True)
    c3 = Card(3, '♠', True)
    c_down = Card(2, '♠', False)

    assert game.is_valid_sequence([c1, c2, c3]) == True, "Expected sequential same-suit face-up cards to be valid"
    assert game.is_valid_sequence([c1, c3]) == False, "Expected non-sequential cards to be invalid"
    assert game.is_valid_sequence([c1, c2, c_down]) == False, "Expected sequence with facedown card to be invalid"
    print("[PASSED] Sequence validation rules are correct.")

    # 4. Simulate a forced move and undo
    print("Simulating a controlled card move and undo...")
    # Clean up column 0 and 1 for controlled test
    game.tableau[0] = [Card(10, '♠', True)]
    game.tableau[1] = [Card(9, '♠', True)]

    # 9♠ can move to 10♠
    assert game.can_move(1, 0, 0) == True, "Should be able to move 9♠ to 10♠"
    assert game.can_move(0, 0, 1) == False, "Should not be able to move 10♠ to 9♠"

    # Perform move
    move_success = game.move_cards(1, 0, 0)
    assert move_success == True, "Move action should succeed"
    assert len(game.tableau[1]) == 0, "Source column should now be empty"
    assert len(game.tableau[0]) == 2, "Destination column should have 2 cards"
    assert game.tableau[0][1].rank == 9, "Top card of destination column should be 9"

    # Undo move
    undo_success = game.undo()
    assert undo_success == True, "Undo should succeed"
    assert len(game.tableau[1]) == 1, "Source column should be restored to 1 card"
    assert len(game.tableau[0]) == 1, "Destination column should be restored to 1 card"
    assert game.tableau[1][0].rank == 9, "Restored card should be 9"
    print("[PASSED] Card move and undo simulation successful.")

    # 5. Simulate Deal from Stock with Empty Columns
    print("Testing stock dealing empty-column constraint...")
    game.tableau[0] = [] # empty column 0
    assert game.can_deal() == False, "Should not be allowed to deal stock when there is an empty column"
    assert game.deal_stock() == False, "Deal action must fail when empty column exists"

    game.tableau[0] = [Card(5, '♠', True)] # Put a card back
    assert game.can_deal() == True, "Should be allowed to deal stock when no empty columns exist"
    deal_success = game.deal_stock()
    assert deal_success == True, "Deal action should succeed"
    assert len(game.stock) == 40, f"Expected 40 stock cards remaining, got {len(game.stock)}"
    print("[PASSED] Stock dealing empty-column constraints successfully validated.")

    # 6. Simulate completed run detection
    print("Verifying run-clearing logic...")
    col = [Card(5, '♠', False)]
    for r in range(13, 0, -1):
        col.append(Card(r, '♠', True))
    game.tableau[0] = col
    
    assert game.completed_runs == 0, "Completed runs should be 0 before check"
    cleared = game.check_and_clear_run(0)
    assert cleared == True, "Complete K-A run should be cleared successfully"
    assert game.completed_runs == 1, f"Expected 1 completed run, got {game.completed_runs}"
    assert len(game.tableau[0]) == 1, f"Expected only facedown card left, got {len(game.tableau[0])}"
    assert game.tableau[0][0].face_up == True, "Remaining facedown card should be flipped face-up"
    print("[PASSED] Run-clearing logic successfully validated.")

    # 7. Print a simple terminal layout simulation for smoke-test
    print("\nSimulated Terminal Layout Render:")
    print("---------------------------------")
    cols_render = []
    for c_idx in range(10):
        col_cards = game.tableau[c_idx]
        cards_str = ", ".join(repr(card) for card in col_cards)
        print(f"Col {c_idx}: {cards_str}")
    print(f"Stock size: {len(game.stock)}")
    print(f"Completed runs: {game.completed_runs}/8")
    print("---------------------------------")

    print("\n====================================================")
    print("ALL SMOKE TESTS PASSED!")
    print("====================================================")
    sys.exit(0)

def main():
    parser = argparse.ArgumentParser(description="Terminal Spider Solitaire Game in Python")
    parser.add_argument("--smoke", action="store_true", help="Run non-interactive smoke test verification and exit")
    parser.add_argument("--suits", type=int, choices=[1, 2, 4], default=1, help="Number of suits to use (1, 2, or 4). Default: 1")
    args = parser.parse_args()

    if args.smoke:
        run_smoke_test()
    else:
        # Import UI here to avoid curses import issues if running in non-terminal environments
        try:
            from card_game_tui.ui import SpiderSolitaireUI
        except ImportError as e:
            print(f"Error: Could not import curses UI. Make sure you are in a terminal environment. Details: {e}", file=sys.stderr)
            sys.exit(1)
        
        ui = SpiderSolitaireUI(suits_count=args.suits)
        ui.run()

if __name__ == "__main__":
    main()

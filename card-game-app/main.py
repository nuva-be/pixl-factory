import argparse
import sys
import os

# Adjust path to import package correctly if not installed
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

from card_game_tui.engine import FreeCellGame, Card, SUITS, SUIT_COLORS, RANK_NAMES
from card_game_tui.ui import run_curses_ui

def render_game_text(game):
    """
    Renders a text snapshot of the current FreeCellGame board state.
    """
    lines = []
    lines.append("=" * 60)
    lines.append(f"Moves: {game.move_count}")
    
    # Render Free Cells and Foundations
    fc_strs = []
    for c in game.free_cells:
        fc_strs.append(f"[{c}]" if c else "[   ]")
    
    fnd_strs = []
    for i, suit in enumerate(game.foundation_suits):
        pile = game.foundations[i]
        fnd_strs.append(f"[{pile[-1]}]" if pile else f"[ {SUITS[suit]} ]")
        
    lines.append(f"Free Cells:  {' '.join(fc_strs)}    Foundations: {' '.join(fnd_strs)}")
    lines.append("-" * 60)
    
    # Render Cascades
    max_height = max(len(col) for col in game.cascades)
    lines.append("Cascades:")
    for row_idx in range(max_height):
        row_strs = []
        for col_idx in range(8):
            col = game.cascades[col_idx]
            if row_idx < len(col):
                card_str = str(col[row_idx])
                if len(card_str) == 2:
                    card_str = " " + card_str
                row_strs.append(f"[{card_str}]")
            else:
                row_strs.append("     ")
        lines.append("  " + " ".join(row_strs))
    lines.append("=" * 60)
    return "\n".join(lines)


def run_smoke_test():
    """
    Non-interactive smoke test.
    Loads a deterministic game (seed=42), performs a sequence of valid moves,
    verifies logic, undoes, and verifies state correctness.
    """
    print("Running non-interactive FreeCell smoke test...")
    
    # 1. Initialize game with a seed
    game = FreeCellGame(seed=42)
    print(f"Game initialized with seed 42. Moves: {game.move_count}")
    
    # Check initial counts
    total_cards = sum(len(col) for col in game.cascades)
    assert total_cards == 52, f"Expected 52 cards, got {total_cards}"
    assert len(game.cascades[0]) == 7, "Cascade 0 should have 7 cards"
    assert len(game.cascades[4]) == 6, "Cascade 4 should have 6 cards"
    print("Initial card counts and distribution verified successfully.")
    
    print("\nINITIAL GAME STATE:")
    print(render_game_text(game))

    # 2. Perform a valid move (bottom card of Cascade 0 to Free Cell 0)
    card_0_bottom = game.cascades[0][-1]
    print(f"Moving bottom card of Cascade 0 ({card_0_bottom}) to Free Cell 0.")
    success, msg = game.validate_and_move('cascade', 0, 'freecell', 0)
    assert success, f"Expected move to succeed, but failed: {msg}"
    assert game.free_cells[0] == card_0_bottom, "Card was not placed in Free Cell 0"
    assert len(game.cascades[0]) == 6, f"Expected Cascade 0 to have 6 cards, got {len(game.cascades[0])}"
    assert game.move_count == 1, f"Expected move count to be 1, got {game.move_count}"
    print("First move completed and verified successfully.")
    
    print("\nGAME STATE AFTER MOVE 1:")
    print(render_game_text(game))

    # 3. Perform another valid move (bottom card of Cascade 1 to Free Cell 1)
    card_1_bottom = game.cascades[1][-1]
    print(f"Moving bottom card of Cascade 1 ({card_1_bottom}) to Free Cell 1.")
    success, msg = game.validate_and_move('cascade', 1, 'freecell', 1)
    assert success, f"Expected move to succeed, but failed: {msg}"
    assert game.free_cells[1] == card_1_bottom, "Card was not placed in Free Cell 1"
    assert len(game.cascades[1]) == 6, f"Expected Cascade 1 to have 6 cards, got {len(game.cascades[1])}"
    assert game.move_count == 2, f"Expected move count to be 2, got {game.move_count}"
    print("Second move completed and verified successfully.")

    print("\nGAME STATE AFTER MOVE 2:")
    print(render_game_text(game))

    # 4. Perform an invalid move (moving to occupied free cell 0)
    print("Testing invalid move: Cascade 2 bottom to occupied Free Cell 0.")
    success, msg = game.validate_and_move('cascade', 2, 'freecell', 0)
    assert not success, "Expected move to fail, but it succeeded!"
    print(f"Invalid move correctly rejected. Error message: '{msg}'")

    # 5. Verify Undo functionality
    print("Undoing second move...")
    undo_success = game.undo()
    assert undo_success, "Undo failed"
    assert game.free_cells[1] is None, "Expected Free Cell 1 to be empty after undo"
    assert len(game.cascades[1]) == 7, "Expected Cascade 1 to restore its card"
    assert game.cascades[1][-1] == card_1_bottom, "Expected original card to return to bottom of Cascade 1"
    assert game.move_count == 1, f"Expected move count to revert to 1, got {game.move_count}"
    print("Undo functionality verified successfully.")

    print("\nGAME STATE AFTER UNDO:")
    print(render_game_text(game))

    # 6. Verify safe auto-collect (Aces are always auto-collected)
    print("Verifying auto-collect of Aces...")
    game = FreeCellGame(seed=42)
    ace_spades = Card('S', 1)
    game.cascades[0].append(ace_spades)
    print("Placed Ace of Spades at the bottom of Cascade 0 manually:")
    print(render_game_text(game))
    
    print("Triggering auto_collect...")
    game.auto_collect()
    
    # Spades foundation is index 0. It should now contain the Ace of Spades.
    assert len(game.foundations[0]) >= 1, "Expected Ace of Spades to be auto-collected to foundation 0"
    assert game.foundations[0][0] == ace_spades, "Foundation 0's first card should be Ace of Spades"
    print("Auto-collect logic verified successfully.")

    print("\nGAME STATE AFTER AUTO-COLLECT:")
    print(render_game_text(game))

    print("\nSmoke test PASSED successfully!")
    sys.exit(0)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Terminal-based FreeCell solitaire game in Python.")
    parser.add_argument('--smoke', action='store_true', help="Run non-interactive smoke test.")
    args = parser.parse_args()

    if args.smoke:
        run_smoke_test()
    else:
        run_curses_ui()

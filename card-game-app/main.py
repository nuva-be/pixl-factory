import sys
import os
import argparse

# Add src to python path to ensure package import works
sys.path.insert(0, os.path.join(os.path.dirname(__file__), 'src'))

from card_game_tui.game import GameState, Card
from card_game_tui.tui import start_ui

def render_text_snapshot(state: GameState):
    """Renders a text snapshot of the current Solitaire board."""
    lines = []
    lines.append("\n" + "=" * 50)
    lines.append("           KLONDIKE SOLITAIRE SNAPSHOT")
    lines.append("=" * 50)
    
    stock_card = f"[ █ {len(state.stock)} ]" if state.stock else "[  X  ]"
    waste_card = state.waste[-1].to_string() if state.waste else "[Empty]"
    
    foundations_str = []
    for i, f in enumerate(state.foundations):
        suit_sym = Card.SUITS[state.foundation_suits[i]]
        top_c = f[-1].to_string() if f else f"[  {suit_sym}  ]"
        foundations_str.append(top_c)
        
    lines.append(f"Stock: {stock_card:<12} Waste: {waste_card:<12} Foundations: {' '.join(foundations_str)}")
    lines.append("-" * 50)
    lines.append("Tableaus:")
    
    max_len = max(len(t) for t in state.tableaus)
    if max_len == 0:
        lines.append("  [ All tableaus empty! ]")
    else:
        for row_idx in range(max_len):
            row_items = []
            for t_idx in range(7):
                t = state.tableaus[t_idx]
                if row_idx < len(t):
                    row_items.append(f"{t[row_idx].to_string():<7}")
                else:
                    row_items.append("       ")
            lines.append("  " + " ".join(row_items))
            
    lines.append("=" * 50)
    lines.append(f"Status Message: {state.message}")
    lines.append("=" * 50 + "\n")
    return "\n".join(lines)

def run_smoke_test():
    """Runs a non-interactive simulation of the game, verifies correctness, and prints snapshots."""
    print("Initializing Solitaire smoke test...")
    
    # Initialize game with a fixed seed for reproducible results
    state = GameState(seed=4242)
    
    # Assert initial setup size
    assert len(state.stock) == 24, f"Expected 24 stock cards, got {len(state.stock)}"
    assert len(state.waste) == 0, "Expected empty waste pile initially"
    assert sum(len(t) for t in state.tableaus) == 28, "Expected 28 cards in tableaus"
    
    # Verify top card of each tableau is face up
    for i in range(7):
        assert state.tableaus[i][-1].face_up, f"Top card of Tableau {i} should be face up"
        
    print("\n[SMOKE TEST] Initial game state layout:")
    print(render_text_snapshot(state))
    
    # Action 1: Draw card from stock
    print("Action: Drawing card from stock...")
    success = state.draw_card()
    assert success, "Failed to draw card from stock"
    assert len(state.stock) == 23, f"Expected 23 stock cards, got {len(state.stock)}"
    assert len(state.waste) == 1, f"Expected 1 card in waste, got {len(state.waste)}"
    assert state.waste[-1].face_up, "Drawn card should be face up"
    
    print(render_text_snapshot(state))
    
    # Action 2: Perform Undo
    print("Action: Undoing last draw...")
    undo_success = state.undo()
    assert undo_success, "Failed to undo last draw action"
    assert len(state.stock) == 24, f"Expected stock back to 24, got {len(state.stock)}"
    assert len(state.waste) == 0, f"Expected waste back to 0, got {len(state.waste)}"
    
    print(render_text_snapshot(state))
    
    # Action 3: Request Hint
    print("Action: Requesting hint...")
    state.request_hint()
    print(f"Resulting message: {state.message}")
    assert "Hint:" in state.message or "No valid moves" in state.message, "Failed to fetch hint"
    
    # Action 4: Auto-play to foundations (should do nothing/succeed on initial board depending on seed)
    print("Action: Trying auto-play to foundations...")
    moves_count = state.auto_play_to_foundations()
    print(f"Auto-play completed with {moves_count} moves.")
    
    print(render_text_snapshot(state))
    
    print("Solitaire smoke test passed successfully!")
    sys.exit(0)

def main():
    parser = argparse.ArgumentParser(description="Terminal Klondike Solitaire Game")
    parser.add_argument("--smoke", action="store_true", help="Run non-interactive smoke test with text snapshots and exit")
    args = parser.parse_args()
    
    if args.smoke:
        run_smoke_test()
    else:
        # Start interactive UI
        start_ui()

if __name__ == "__main__":
    main()

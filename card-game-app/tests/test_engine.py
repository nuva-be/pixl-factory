import unittest
from card_game_tui.engine import Card, FreeCellGame

class TestFreeCellEngine(unittest.TestCase):
    def test_card_creation(self):
        card = Card('H', 1)
        self.assertEqual(card.suit, 'H')
        self.assertEqual(card.rank, 1)
        self.assertEqual(card.color, 'red')
        self.assertEqual(str(card), "A♥")
        self.assertEqual(repr(card), "A♥")

    def test_card_equality(self):
        c1 = Card('S', 10)
        c2 = Card('S', 10)
        c3 = Card('H', 10)
        self.assertEqual(c1, c2)
        self.assertNotEqual(c1, c3)
        self.assertNotEqual(c1, "10♠")

    def test_invalid_card(self):
        with self.assertRaises(ValueError):
            Card('X', 5)
        with self.assertRaises(ValueError):
            Card('H', 14)

    def test_deal(self):
        game = FreeCellGame(seed=42)
        # Check that 52 cards were dealt correctly
        total_cards = sum(len(c) for c in game.cascades)
        self.assertEqual(total_cards, 52)
        self.assertEqual(len(game.cascades[0]), 7)
        self.assertEqual(len(game.cascades[4]), 6)
        # Check empty states
        self.assertTrue(all(cell is None for cell in game.free_cells))
        self.assertTrue(all(len(fnd) == 0 for fnd in game.foundations))

    def test_get_bottom_sequence(self):
        # Create custom cascades to test sequence identification
        c1 = [
            Card('H', 13), # K♥
            Card('S', 12), # Q♠
            Card('H', 11), # J♥
            Card('C', 10), # 10♣
        ]
        seq = FreeCellGame.get_bottom_sequence(c1)
        self.assertEqual(len(seq), 4)
        self.assertEqual(seq[0].rank, 13)

        # Break sequence in middle
        c2 = [
            Card('H', 13), # K♥
            Card('H', 12), # Q♥ (same color, breaks sequence)
            Card('S', 11), # J♠
            Card('H', 10), # 10♥
        ]
        seq = FreeCellGame.get_bottom_sequence(c2)
        self.assertEqual(len(seq), 3)
        self.assertEqual(seq[0].rank, 12)

    def test_get_max_move_size(self):
        game = FreeCellGame(seed=42)
        # F=4 empty free cells, E=0 empty cascades
        # Max move size = (4 + 1) * 2^0 = 5
        self.assertEqual(game.get_max_move_size(), 5)

        # Make one free cell occupied
        game.free_cells[0] = Card('S', 1)
        # F=3, E=0 -> (3 + 1) * 2^0 = 4
        self.assertEqual(game.get_max_move_size(), 4)

        # Make one cascade empty
        game.cascades[7] = []
        # F=3, E=1 (excluding src and dest cascade indices)
        # (3 + 1) * 2^1 = 8
        self.assertEqual(game.get_max_move_size(exclude_src_idx=0, exclude_dest_idx=1), 8)

    def test_validate_and_move_to_free_cell(self):
        game = FreeCellGame(seed=42)
        # Try moving bottom card of first cascade to first free cell
        bottom_card = game.cascades[0][-1]
        success, msg = game.validate_and_move('cascade', 0, 'freecell', 0)
        self.assertTrue(success)
        self.assertEqual(game.free_cells[0], bottom_card)
        self.assertEqual(len(game.cascades[0]), 6)

        # Test moving to already occupied free cell
        success, msg = game.validate_and_move('cascade', 1, 'freecell', 0)
        self.assertFalse(success)

    def test_validate_and_move_to_foundation(self):
        game = FreeCellGame(seed=42)
        # Clear cascades to avoid any auto-collect side effects during manual testing
        game.cascades = [[] for _ in range(8)]
        
        # Manually inject an Ace of Spades (S, 1) to make it ready for foundation
        # Foundations: 0=Spades, 1=Hearts, 2=Diamonds, 3=Clubs
        ace_spades = Card('S', 1)
        game.cascades[0].append(ace_spades)
        
        # Try moving to foundation
        success, msg = game.validate_and_move('cascade', 0, 'foundation', 0)
        self.assertTrue(success)
        self.assertEqual(game.foundations[0][-1], ace_spades)

        # Test valid consecutive foundation moves
        two_spades = Card('S', 2)
        game.cascades[1].append(two_spades)
        # Moving to Spades (0) should work because Ace of Spades is already there
        success, msg = game.validate_and_move('cascade', 1, 'foundation', 0)
        self.assertTrue(success)
        self.assertEqual(game.foundations[0][-1], two_spades)

        # Moving random non-consecutive card should fail
        king_hearts = Card('H', 13)
        game.cascades[2].append(king_hearts)
        success, msg = game.validate_and_move('cascade', 2, 'foundation', 1)
        self.assertFalse(success) # Hearts is empty, needs Ace

    def test_cascade_to_cascade_single(self):
        game = FreeCellGame(seed=42)
        # Set up a clean valid move
        # Put 10 of Clubs on bottom of cascade 0
        # Put 9 of Hearts on bottom of cascade 1
        game.cascades[0] = [Card('C', 10)]
        game.cascades[1] = [Card('H', 9)]

        success, msg = game.validate_and_move('cascade', 1, 'cascade', 0)
        self.assertTrue(success)
        self.assertEqual(len(game.cascades[0]), 2)
        self.assertEqual(game.cascades[0][-1], Card('H', 9))
        self.assertEqual(len(game.cascades[1]), 0)

    def test_undo(self):
        game = FreeCellGame(seed=42)
        original_cascade_len = len(game.cascades[0])
        success, msg = game.validate_and_move('cascade', 0, 'freecell', 0)
        self.assertTrue(success)
        
        self.assertEqual(len(game.cascades[0]), original_cascade_len - 1)
        self.assertIsNotNone(game.free_cells[0])

        undo_success = game.undo()
        self.assertTrue(undo_success)
        self.assertEqual(len(game.cascades[0]), original_cascade_len)
        self.assertIsNone(game.free_cells[0])

    def test_check_win(self):
        game = FreeCellGame(seed=42)
        self.assertFalse(game.check_win())

        # Simulate a win state
        for idx, suit in enumerate(game.foundation_suits):
            game.foundations[idx] = [Card(suit, rank) for rank in range(1, 14)]
        self.assertTrue(game.check_win())

if __name__ == '__main__':
    unittest.main()

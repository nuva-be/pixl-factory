import unittest
from card_game_tui.game import Card, SpiderGame

class TestSpiderGame(unittest.TestCase):
    def test_card_init(self):
        c = Card(1, '♠', False)
        self.assertEqual(c.rank, 1)
        self.assertEqual(c.suit, '♠')
        self.assertFalse(c.face_up)
        self.assertEqual(repr(c), "[###]")

        c.face_up = True
        self.assertEqual(repr(c), "[ A♠]")

        c2 = Card(10, '♥', True)
        self.assertEqual(repr(c2), "[10♥]")

    def test_game_setup_1_suit(self):
        game = SpiderGame(suits_count=1)
        # 104 cards total. 54 in tableau, 50 in stock
        self.assertEqual(len(game.stock), 50)
        total_tableau = sum(len(col) for col in game.tableau)
        self.assertEqual(total_tableau, 54)

        # Check column card counts
        for i in range(4):
            self.assertEqual(len(game.tableau[i]), 6)
            self.assertTrue(game.tableau[i][-1].face_up)
            # others are face-down
            for card in game.tableau[i][:-1]:
                self.assertFalse(card.face_up)

        for i in range(4, 10):
            self.assertEqual(len(game.tableau[i]), 5)
            self.assertTrue(game.tableau[i][-1].face_up)

    def test_game_setup_2_suit(self):
        game = SpiderGame(suits_count=2)
        all_cards = list(game.stock)
        for col in game.tableau:
            all_cards.extend(col)
        
        suits = {c.suit for c in all_cards}
        self.assertEqual(suits, {'♠', '♥'})

    def test_is_valid_sequence(self):
        game = SpiderGame(suits_count=1)
        # Empty list is not valid
        self.assertFalse(game.is_valid_sequence([]))

        # Single card face up is valid
        c1 = Card(5, '♠', True)
        self.assertTrue(game.is_valid_sequence([c1]))

        # Face down card is not valid
        c_down = Card(5, '♠', False)
        self.assertFalse(game.is_valid_sequence([c_down]))

        # Valid sequence: 5, 4, 3 same suit
        c2 = Card(4, '♠', True)
        c3 = Card(3, '♠', True)
        self.assertTrue(game.is_valid_sequence([c1, c2, c3]))

        # Different suit not valid
        c2_h = Card(4, '♥', True)
        self.assertFalse(game.is_valid_sequence([c1, c2_h, c3]))

        # Non-sequential not valid
        self.assertFalse(game.is_valid_sequence([c1, c3]))

    def test_move_validation_and_action(self):
        game = SpiderGame(suits_count=1)
        # Force a controlled state
        game.tableau[0] = [Card(10, '♠', True)]
        game.tableau[1] = [Card(9, '♠', True)]

        # Can move 9 to 10
        self.assertTrue(game.can_move(1, 0, 0))
        # Cannot move 10 to 9
        self.assertFalse(game.can_move(0, 0, 1))

        # Execute move
        success = game.move_cards(1, 0, 0)
        self.assertTrue(success)
        self.assertEqual(len(game.tableau[1]), 0)
        self.assertEqual(len(game.tableau[0]), 2)
        self.assertEqual(game.tableau[0][0].rank, 10)
        self.assertEqual(game.tableau[0][1].rank, 9)

        # Undo the move
        self.assertTrue(game.undo())
        self.assertEqual(len(game.tableau[1]), 1)
        self.assertEqual(len(game.tableau[0]), 1)
        self.assertEqual(game.tableau[1][0].rank, 9)
        self.assertEqual(game.tableau[0][0].rank, 10)

    def test_stock_deal_and_empty_column_constraint(self):
        game = SpiderGame(suits_count=1)
        original_stock_count = len(game.stock)
        self.assertTrue(game.can_deal())
        
        # Empty a column
        game.tableau[0] = []
        # Cannot deal if any column is empty
        self.assertFalse(game.can_deal())
        self.assertFalse(game.deal_stock())

        # Put a card back
        game.tableau[0] = [Card(5, '♠', True)]
        self.assertTrue(game.can_deal())
        self.assertTrue(game.deal_stock())
        self.assertEqual(len(game.stock), original_stock_count - 10)

    def test_check_and_clear_run(self):
        game = SpiderGame(suits_count=1)
        # Setup a column with facedown card, then K down to A
        col = [Card(5, '♠', False)]
        for r in range(13, 0, -1):
            col.append(Card(r, '♠', True))
        game.tableau[0] = col

        # Initially 0 completed runs
        self.assertEqual(game.completed_runs, 0)

        # Trigger check/clear
        cleared = game.check_and_clear_run(0)
        self.assertTrue(cleared)
        self.assertEqual(game.completed_runs, 1)
        
        # The facedown card should be flipped face-up now
        self.assertEqual(len(game.tableau[0]), 1)
        self.assertTrue(game.tableau[0][0].face_up)

if __name__ == '__main__':
    unittest.main()

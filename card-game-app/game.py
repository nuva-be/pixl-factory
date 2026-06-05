import random
import copy

class Card:
    def __init__(self, rank, suit, face_up=False):
        self.rank = rank  # 1 (Ace) to 13 (King)
        self.suit = suit  # '♠', '♥', '♦', '♣'
        self.face_up = face_up

    def __repr__(self):
        if not self.face_up:
            return "[###]"
        rank_str = {
            1: "A",
            11: "J",
            12: "Q",
            13: "K"
        }.get(self.rank, str(self.rank))
        # Ensure 2-char representation for alignment
        if len(rank_str) == 1:
            rank_str = " " + rank_str
        return f"[{rank_str}{self.suit}]"

    def clone(self):
        return Card(self.rank, self.suit, self.face_up)


class SpiderGame:
    def __init__(self, suits_count=1):
        if suits_count not in (1, 2, 4):
            raise ValueError("Suits count must be 1, 2, or 4")
        self.suits_count = suits_count
        self.tableau = [[] for _ in range(10)]
        self.stock = []
        self.completed_runs = 0
        self.undo_stack = []
        self.reset_game()

    def reset_game(self):
        self.tableau = [[] for _ in range(10)]
        self.completed_runs = 0
        self.undo_stack = []

        # Generate 104 cards based on suit count
        cards = []
        if self.suits_count == 1:
            # 8 sets of Spades (13 cards each)
            for _ in range(8):
                for rank in range(1, 14):
                    cards.append(Card(rank, '♠'))
        elif self.suits_count == 2:
            # 4 sets of Spades, 4 sets of Hearts
            for _ in range(4):
                for rank in range(1, 14):
                    cards.append(Card(rank, '♠'))
                    cards.append(Card(rank, '♥'))
        else:
            # 4 suits: 2 sets of each suit (Spades, Hearts, Diamonds, Clubs)
            for _ in range(2):
                for suit in ['♠', '♥', '♦', '♣']:
                    for rank in range(1, 14):
                        cards.append(Card(rank, suit))

        random.shuffle(cards)

        # Deal to tableau:
        # First 4 columns get 6 cards (total 24)
        # Next 6 columns get 5 cards (total 30)
        # Remaining 50 form the stock.
        for col in range(10):
            num_cards = 6 if col < 4 else 5
            for _ in range(num_cards):
                self.tableau[col].append(cards.pop())
            # Turn top card face-up
            if self.tableau[col]:
                self.tableau[col][-1].face_up = True

        self.stock = cards

    def save_state(self):
        """Returns a snapshot of the game state for undo."""
        return {
            'tableau': [[c.clone() for c in col] for col in self.tableau],
            'stock': [c.clone() for c in self.stock],
            'completed_runs': self.completed_runs
        }

    def push_undo(self, snapshot):
        self.undo_stack.append(snapshot)

    def undo(self):
        if not self.undo_stack:
            return False
        state = self.undo_stack.pop()
        self.tableau = state['tableau']
        self.stock = state['stock']
        self.completed_runs = state['completed_runs']
        return True

    def is_valid_sequence(self, cards):
        """Checks if a subset of cards forms a valid same-suit decreasing sequence."""
        if not cards:
            return False
        if not all(c.face_up for c in cards):
            return False
        suit = cards[0].suit
        for i in range(len(cards) - 1):
            if cards[i].suit != suit:
                return False
            if cards[i].rank != cards[i+1].rank + 1:
                return False
        return True

    def get_movable_sequence_start_indices(self, col_idx):
        """Returns list of valid start indices for sequences in the column."""
        col = self.tableau[col_idx]
        if not col:
            return []
        
        valid_indices = []
        for i in range(len(col)):
            if col[i].face_up:
                if self.is_valid_sequence(col[i:]):
                    valid_indices.append(i)
        return valid_indices

    def can_move(self, from_col, card_idx, to_col):
        """Checks if moving the sequence starting at card_idx from from_col to to_col is valid."""
        if from_col < 0 or from_col >= 10 or to_col < 0 or to_col >= 10:
            return False
        if from_col == to_col:
            return False

        col_from = self.tableau[from_col]
        col_to = self.tableau[to_col]

        if not col_from or card_idx < 0 or card_idx >= len(col_from):
            return False

        moving_cards = col_from[card_idx:]
        if not self.is_valid_sequence(moving_cards):
            return False

        # If target column is empty, any sequence is allowed
        if not col_to:
            return True

        # Target column's top card must be face_up
        target_card = col_to[-1]
        if not target_card.face_up:
            return False

        # Target card rank must be exactly moving_sequence_start_rank + 1
        # Note: Suit doesn't have to match for placing, but has to match for moving together.
        if target_card.rank == moving_cards[0].rank + 1:
            return True

        return False

    def move_cards(self, from_col, card_idx, to_col):
        """Moves cards if valid, handles auto-flip and clearing complete runs."""
        if not self.can_move(from_col, card_idx, to_col):
            return False

        # Save state before modification
        snapshot = self.save_state()

        col_from = self.tableau[from_col]
        col_to = self.tableau[to_col]

        moving_cards = col_from[card_idx:]
        self.tableau[from_col] = col_from[:card_idx]
        self.tableau[to_col].extend(moving_cards)

        # Auto-flip newly exposed card
        if self.tableau[from_col] and not self.tableau[from_col][-1].face_up:
            self.tableau[from_col][-1].face_up = True

        # Check for completed run in target column
        self.check_and_clear_run(to_col)

        # Push state to undo stack
        self.push_undo(snapshot)
        return True

    def check_and_clear_run(self, col_idx):
        """Checks if a completed K to A same-suit sequence is at the top of col_idx, and clears it."""
        col = self.tableau[col_idx]
        if len(col) < 13:
            return False

        # Look at last 13 cards
        potential_run = col[-13:]
        if not self.is_valid_sequence(potential_run):
            return False

        # Verify it goes from King (13) down to Ace (1)
        if potential_run[0].rank == 13 and potential_run[-1].rank == 1:
            # We have a complete run! Clear it.
            self.tableau[col_idx] = col[:-13]
            self.completed_runs += 1

            # Auto-flip new top card
            if self.tableau[col_idx] and not self.tableau[col_idx][-1].face_up:
                self.tableau[col_idx][-1].face_up = True
            return True

        return False

    def can_deal(self):
        """Stock can be dealt if stock is not empty and no columns are empty (standard rule)."""
        if not self.stock:
            return False
        # Check for empty columns
        for col in self.tableau:
            if not col:
                return False
        return True

    def deal_stock(self):
        """Deals 10 cards from stock to tableau columns, one each."""
        if not self.can_deal():
            return False

        snapshot = self.save_state()

        for col in range(10):
            card = self.stock.pop()
            card.face_up = True
            self.tableau[col].append(card)
            # Dealing could theoretically complete a run in one of the columns
            self.check_and_clear_run(col)

        self.push_undo(snapshot)
        return True

    def is_won(self):
        return self.completed_runs == 8

    def has_moves_left(self):
        """Check if any valid moves are possible on the current board (or stock is available)."""
        if self.stock:
            return True

        # Check all possible moves between columns
        for from_col in range(10):
            valid_indices = self.get_movable_sequence_start_indices(from_col)
            for card_idx in valid_indices:
                for to_col in range(10):
                    if self.can_move(from_col, card_idx, to_col):
                        return True
        return False

from card_game_tui.engine import GameState, Move, validate_move, Card, Rank, Suit

from card_game_tui.engine import GameState, Move, validate_move, Card, Rank, Suit, get_max_movable_cards

def test_validate_move_initial_illegal():
    state = GameState()
    state.deal(seed=42)
    # Moving card from empty FreeCell is illegal
    move = Move('F', 0, 'C', 0, 1)
    valid, reason = validate_move(state, move)
    assert not valid
    assert "Source is empty or invalid" in reason

def test_get_max_movable_cards():
    state = GameState()
    # Initial state: 4 free cells empty, 0 empty columns
    state.free_cells = [None] * 4
    state.tableau = [[Card(Rank.ACE, Suit.SPADES)] for _ in range(8)]
    assert get_max_movable_cards(state, target_is_empty_col=False) == 5

    # 1 free cell occupied, 0 empty columns -> F = 3, T = 0
    state.free_cells[0] = Card(Rank.KING, Suit.HEARTS)
    assert get_max_movable_cards(state, target_is_empty_col=False) == 4

    # F = 3, 2 empty columns -> T = 2.
    # If target is NOT empty, max = (1 + 3) * 2^2 = 16
    state.tableau[0] = []
    state.tableau[1] = []
    assert get_max_movable_cards(state, target_is_empty_col=False) == 16
    # If target is empty, T is effectively reduced by 1 -> max = (1 + 3) * 2^1 = 8
    assert get_max_movable_cards(state, target_is_empty_col=True) == 8

def test_validate_move_sequence():
    state = GameState()
    state.free_cells = [None] * 4
    # Columns:
    # C0: [K♥, Q♠, J♥] -> valid sequence
    # C1: [10♣]
    state.tableau = [[] for _ in range(8)]
    state.tableau[0] = [
        Card(Rank.KING, Suit.HEARTS),
        Card(Rank.QUEEN, Suit.SPADES),
        Card(Rank.JACK, Suit.HEARTS)
    ]
    state.tableau[1] = [Card(Rank.TEN, Suit.CLUBS)]

    # Move sequence J♥ (len 1) to 10♣ is invalid because J cannot go on 10
    move_invalid = Move('C', 0, 'C', 1, 1)
    valid, reason = validate_move(state, move_invalid)
    assert not valid

    # Move sequence J♥ (len 1) is valid to move to empty C2
    move_valid_single = Move('C', 0, 'C', 2, 1)
    valid, reason = validate_move(state, move_valid_single)
    assert valid

    # Let's check moving a sequence [Q♠, J♥] (len 2) onto an empty column
    move_seq = Move('C', 0, 'C', 2, 2)
    valid, reason = validate_move(state, move_seq)
    assert valid

    # If we fill all free cells and make column moves restricted:
    # F = 0, T = 6 (6 empty columns, but moving to C2 reduces effective empty cols to 5)
    # Let's make all other columns occupied so T = 0.
    state.free_cells = [Card(Rank.TWO, Suit.DIAMONDS)] * 4
    # Set other columns occupied
    for i in range(2, 8):
        state.tableau[i] = [Card(Rank.ACE, Suit.DIAMONDS)]
    
    # But C2 will be our target column, set it to King of Diamonds (K♦)
    # The sequence starting card is Queen of Spades (Q♠). Q♠ can go on K♦.
    state.tableau[2] = [Card(Rank.KING, Suit.DIAMONDS)]
    
    # Now F = 0, T = 0 -> max movable is (1+0)*2^0 = 1.
    # Moving [Q♠, J♥] (len 2) to C2 (K♦) should fail due to capacity.
    move_too_long = Move('C', 0, 'C', 2, 2)
    valid, reason = validate_move(state, move_too_long)
    assert not valid
    assert "Insufficient empty FreeCells" in reason

def test_validate_move_to_freecell():
    state = GameState()
    state.free_cells = [None] * 4
    state.tableau[0] = [Card(Rank.ACE, Suit.SPADES)]
    
    # Valid move of single card to empty FreeCell
    move_valid = Move('C', 0, 'F', 0, 1)
    valid, reason = validate_move(state, move_valid)
    assert valid

    # Cannot move a sequence to a FreeCell
    state.tableau[0] = [Card(Rank.TWO, Suit.HEARTS), Card(Rank.ACE, Suit.SPADES)]
    move_seq = Move('C', 0, 'F', 1, 2)
    valid, reason = validate_move(state, move_seq)
    assert not valid
    assert "Cannot move a sequence to a FreeCell" in reason

    # Cannot move to an occupied FreeCell
    state.free_cells[2] = Card(Rank.KING, Suit.CLUBS)
    move_occ = Move('C', 0, 'F', 2, 1)
    valid, reason = validate_move(state, move_occ)
    assert not valid
    assert "Target FreeCell is occupied" in reason

def test_validate_move_to_foundation():
    state = GameState()
    state.free_cells = [None] * 4
    state.foundations = {suit: [] for suit in Suit}
    state.tableau[0] = [Card(Rank.ACE, Suit.SPADES)]
    state.tableau[1] = [Card(Rank.TWO, Suit.SPADES)]

    # Moving Ace of Spades to empty Foundation is valid
    move_ace = Move('C', 0, 'A', 0, 1)
    valid, reason = validate_move(state, move_ace)
    assert valid

    # Moving Two of Spades to empty Foundation is invalid
    move_two_invalid = Move('C', 1, 'A', 0, 1)
    valid, reason = validate_move(state, move_two_invalid)
    assert not valid
    assert "Foundations must start with an Ace" in reason

    # Place Ace of Spades in foundation first
    state.foundations[Suit.SPADES] = [Card(Rank.ACE, Suit.SPADES)]
    # Now, moving Two of Spades to foundation is valid
    move_two_valid = Move('C', 1, 'A', 0, 1)
    valid, reason = validate_move(state, move_two_valid)
    assert valid

    # Moving a non-consecutive card (e.g. Four of Spades) is invalid
    state.tableau[2] = [Card(Rank.FOUR, Suit.SPADES)]
    move_four_invalid = Move('C', 2, 'A', 0, 1)
    valid, reason = validate_move(state, move_four_invalid)
    assert not valid
    assert "Must be next rank up" in reason

    # Cannot move a sequence to a Foundation
    state.tableau[3] = [Card(Rank.THREE, Suit.HEARTS), Card(Rank.TWO, Suit.SPADES)]
    move_seq = Move('C', 3, 'A', 0, 2)
    valid, reason = validate_move(state, move_seq)
    assert not valid
    assert "Cannot move a sequence to a Foundation" in reason

def test_validate_move_sequence_invalid_alternating_color():
    state = GameState()
    state.free_cells = [None] * 4
    state.tableau[0] = [Card(Rank.JACK, Suit.HEARTS), Card(Rank.TEN, Suit.DIAMONDS)]
    
    # Hearts and Diamonds are both RED. Sequence J♥, 10♦ is invalid!
    move = Move('C', 0, 'C', 1, 2)
    valid, reason = validate_move(state, move)
    assert not valid
    assert "alternating color descending sequence" in reason

def test_validate_move_sequence_invalid_ranks():
    state = GameState()
    state.free_cells = [None] * 4
    state.tableau[0] = [Card(Rank.JACK, Suit.HEARTS), Card(Rank.NINE, Suit.SPADES)]
    
    # Jack (11) and Nine (9) is invalid sequence (should be Jack and Ten)
    move = Move('C', 0, 'C', 1, 2)
    valid, reason = validate_move(state, move)
    assert not valid
    assert "alternating color descending sequence" in reason



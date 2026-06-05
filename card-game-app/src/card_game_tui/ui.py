import curses
import sys
from card_game_tui.game import SpiderGame, Card

class SpiderSolitaireUI:
    def __init__(self, suits_count=1):
        self.game = SpiderGame(suits_count=suits_count)
        self.cursor_col = 0
        self.cursor_card_idx = 0
        self.selected_col = None
        self.selected_card_idx = None
        self.status_msg = "Welcome to Spider Solitaire! Use arrow keys to navigate."
        self.status_color = 4  # Cyan

        self.adjust_cursor_after_col_change()

    def adjust_cursor_after_col_change(self):
        col = self.game.tableau[self.cursor_col]
        if not col:
            self.cursor_card_idx = 0
        else:
            self.cursor_card_idx = len(col) - 1

    def run(self):
        # Use curses wrapper to handle init and cleanup automatically
        curses.wrapper(self.main_loop)

    def main_loop(self, stdscr):
        # Setup colors
        curses.use_default_colors()
        curses.init_pair(1, curses.COLOR_RED, -1)     # Red for Hearts/Diamonds
        curses.init_pair(2, curses.COLOR_WHITE, -1)   # White/Default for Spades/Clubs
        curses.init_pair(3, curses.COLOR_YELLOW, curses.COLOR_BLUE) # Highlighted/Selected card
        curses.init_pair(4, curses.COLOR_CYAN, -1)    # Labels/Borders/Status
        curses.init_pair(5, curses.COLOR_GREEN, -1)   # Success/Completed
        curses.init_pair(6, curses.COLOR_BLACK, curses.COLOR_WHITE) # Reverse video fallback or dark gray

        curses.curs_set(0) # Hide blinking cursor
        stdscr.keypad(True)
        stdscr.clear()

        while True:
            # Check terminal size
            height, width = stdscr.getmaxyx()
            if height < 24 or width < 80:
                stdscr.clear()
                stdscr.addstr(0, 0, "Terminal too small!", curses.color_pair(1) | curses.A_BOLD)
                stdscr.addstr(1, 0, f"Current: {width}x{height}. Required: >= 80x24.", curses.color_pair(4))
                stdscr.addstr(2, 0, "Please resize your terminal window to continue...", curses.color_pair(4))
                stdscr.refresh()
                
                # Wait for resize
                ch = stdscr.getch()
                if ch in (ord('q'), ord('Q')):
                    break
                continue

            self.render_screen(stdscr)
            
            try:
                ch = stdscr.getch()
            except KeyboardInterrupt:
                break

            if ch in (ord('q'), ord('Q')):
                break
            
            self.handle_input(ch)

    def render_screen(self, stdscr):
        stdscr.erase()
        height, width = stdscr.getmaxyx()

        # 1. Header Area (rows 0-2)
        stdscr.addstr(0, 0, "=" * (width - 1), curses.color_pair(4))
        title = " SPIDER SOLITAIRE "
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.color_pair(4) | curses.A_BOLD)

        mode_str = f"Mode: {self.game.suits_count}-Suit"
        stock_str = f"Stock: {'[|||||]' if self.game.stock else '[EMPTY]'} ({len(self.game.stock)} cards)"
        runs_str = f"Completed: {self.game.completed_runs}/8"
        
        stdscr.addstr(1, 2, stock_str, curses.color_pair(4))
        stdscr.addstr(1, 30, runs_str, curses.color_pair(5) if self.game.completed_runs > 0 else curses.color_pair(2))
        stdscr.addstr(1, width - len(mode_str) - 3, mode_str, curses.color_pair(4))
        
        stdscr.addstr(2, 0, "=" * (width - 1), curses.color_pair(4))

        # 2. Tableau Columns (rows 4 to height-6)
        col_width = 7
        col_gap = 1
        
        # Draw Column headers and cards
        for col_idx in range(10):
            x = col_idx * (col_width + col_gap) + 1
            
            # Header label
            header_attr = curses.color_pair(4)
            if col_idx == self.cursor_col:
                header_attr |= curses.A_REVERSE | curses.A_BOLD
            stdscr.addstr(4, x, f" Col {col_idx} ", header_attr)

            col_cards = self.game.tableau[col_idx]
            
            if not col_cards:
                # Render empty column slot
                if col_idx == self.cursor_col:
                    stdscr.addstr(6, x + 1, "[   ]", curses.color_pair(3))
                else:
                    stdscr.addstr(6, x + 1, "[ - ]", curses.color_pair(4))
                continue

            for card_idx, card in enumerate(col_cards):
                y = 6 + card_idx
                if y >= height - 6:
                    # Prevent writing off screen if too many cards
                    # Just draw a indicator that there are more cards
                    stdscr.addstr(height - 6, x + 2, "...", curses.color_pair(1) | curses.A_BOLD)
                    break

                # Prepare card string representation
                is_bottom = (card_idx == len(col_cards) - 1)
                
                # Check formatting
                if not card.face_up:
                    card_str = " |###| " if not is_bottom else " [###] "
                    attr = curses.color_pair(2) # Default / facedown
                else:
                    rank_str = {1: "A", 11: "J", 12: "Q", 13: "K"}.get(card.rank, str(card.rank))
                    if len(rank_str) == 1:
                        rank_str = " " + rank_str
                    
                    if is_bottom:
                        card_str = f" [{rank_str}{card.suit}] "
                    else:
                        card_str = f" |{rank_str}{card.suit}| "

                    # Suit colors
                    if card.suit in ('♥', '♦'):
                        attr = curses.color_pair(1)  # Red
                    else:
                        attr = curses.color_pair(2)  # White/Spade/Club

                # Highlight rules:
                # If we are in SELECTED state:
                # - If this column is the selected column, and we are at or below selected_card_idx: highlight!
                # If we are in IDLE state:
                # - If this column is the current column, and we are at the cursor_card_idx: highlight!
                is_highlighted = False
                if self.selected_col is not None:
                    if col_idx == self.selected_col and card_idx >= self.selected_card_idx:
                        is_highlighted = True
                else:
                    if col_idx == self.cursor_col and card_idx == self.cursor_card_idx:
                        is_highlighted = True

                if is_highlighted:
                    attr = curses.color_pair(3) | curses.A_BOLD

                stdscr.addstr(y, x, card_str, attr)

        # 3. Status Line (height - 4)
        stdscr.addstr(height - 4, 0, "-" * (width - 1), curses.color_pair(4))
        stdscr.addstr(height - 3, 2, self.status_msg[:width-5], curses.color_pair(self.status_color) | curses.A_BOLD)

        # 4. Footer controls (height - 2 and height - 1)
        help_line1 = "Arrows: Navigate | Space/Enter: Select Card | 0-9: Jump Col | D: Deal Stock | U: Undo"
        help_line2 = "R: Restart | Q: Quit | Esc: Cancel Selection"
        if self.selected_col is not None:
            help_line1 = "Arrows: Choose Target Column | Space/Enter: Drop Cards | Esc: Cancel Selection"
        
        stdscr.addstr(height - 2, 2, help_line1[:width-5], curses.color_pair(4))
        stdscr.addstr(height - 1, 2, help_line2[:width-5], curses.color_pair(4))

        stdscr.refresh()

    def handle_input(self, ch):
        # Escape key handling
        if ch == 27:
            if self.selected_col is not None:
                self.selected_col = None
                self.selected_card_idx = None
                self.status_msg = "Selection cancelled."
                self.status_color = 4
            return

        # Undo
        if ch in (ord('u'), ord('U')):
            if self.selected_col is not None:
                self.selected_col = None
                self.selected_card_idx = None
            if self.game.undo():
                self.status_msg = "Undo successful."
                self.status_color = 5
                self.adjust_cursor_after_col_change()
            else:
                self.status_msg = "Nothing to undo."
                self.status_color = 1
            return

        # Restart
        if ch in (ord('r'), ord('R')):
            self.game.reset_game()
            self.cursor_col = 0
            self.selected_col = None
            self.selected_card_idx = None
            self.status_msg = "Game restarted."
            self.status_color = 4
            self.adjust_cursor_after_col_change()
            return

        # Deal Stock
        if ch in (ord('d'), ord('D')):
            if self.selected_col is not None:
                # Cancel selection on deal
                self.selected_col = None
                self.selected_card_idx = None
            
            if self.game.can_deal():
                self.game.deal_stock()
                self.status_msg = "Dealt 10 cards from stock."
                self.status_color = 5
                self.adjust_cursor_after_col_change()
            else:
                if not self.game.stock:
                    self.status_msg = "Cannot deal: Stock is empty!"
                else:
                    self.status_msg = "Cannot deal: All columns must have at least 1 card!"
                self.status_color = 1
            return

        # Column shortcuts (0-9)
        if ord('0') <= ch <= ord('9'):
            target_col = ch - ord('0')
            if self.selected_col is not None:
                # Attempt to move to this column
                self.attempt_move_to(target_col)
            else:
                self.cursor_col = target_col
                self.adjust_cursor_after_col_change()
                self.status_msg = f"Jumped to Column {target_col}"
                self.status_color = 4
            return

        # State-dependent input handling
        if self.selected_col is None:
            self.handle_idle_input(ch)
        else:
            self.handle_selected_input(ch)

    def handle_idle_input(self, ch):
        col = self.game.tableau[self.cursor_col]

        if ch == curses.KEY_LEFT:
            self.cursor_col = (self.cursor_col - 1) % 10
            self.adjust_cursor_after_col_change()
        elif ch == curses.KEY_RIGHT:
            self.cursor_col = (self.cursor_col + 1) % 10
            self.adjust_cursor_after_col_change()
        elif ch == curses.KEY_UP:
            # Move cursor up within face-up cards of the current column
            if col:
                valid_start_indices = self.game.get_movable_sequence_start_indices(self.cursor_col)
                if valid_start_indices:
                    # Find if we can move up
                    current_pos = valid_start_indices.index(self.cursor_card_idx) if self.cursor_card_idx in valid_start_indices else -1
                    if current_pos > 0:
                        self.cursor_card_idx = valid_start_indices[current_pos - 1]
                    else:
                        # Fallback/stay at the highest valid sequence start
                        self.cursor_card_idx = valid_start_indices[0]
        elif ch == curses.KEY_DOWN:
            if col:
                valid_start_indices = self.game.get_movable_sequence_start_indices(self.cursor_col)
                if valid_start_indices:
                    current_pos = valid_start_indices.index(self.cursor_card_idx) if self.cursor_card_idx in valid_start_indices else -1
                    if current_pos != -1 and current_pos < len(valid_start_indices) - 1:
                        self.cursor_card_idx = valid_start_indices[current_pos + 1]
                    else:
                        self.cursor_card_idx = len(col) - 1
        elif ch in (10, 13, ord(' '), curses.KEY_ENTER):
            # Select sequence
            if not col:
                self.status_msg = "Cannot select from an empty column!"
                self.status_color = 1
                return

            valid_indices = self.game.get_movable_sequence_start_indices(self.cursor_col)
            if self.cursor_card_idx not in valid_indices:
                self.status_msg = "Invalid selection: Cards must be same suit and decreasing!"
                self.status_color = 1
                return

            # Success, select!
            self.selected_col = self.cursor_col
            self.selected_card_idx = self.cursor_card_idx
            self.status_msg = f"Selected cards from Col {self.selected_col}. Choose target column."
            self.status_color = 3

    def handle_selected_input(self, ch):
        if ch == curses.KEY_LEFT:
            self.cursor_col = (self.cursor_col - 1) % 10
        elif ch == curses.KEY_RIGHT:
            self.cursor_col = (self.cursor_col + 1) % 10
        elif ch in (10, 13, ord(' '), curses.KEY_ENTER):
            self.attempt_move_to(self.cursor_col)

    def attempt_move_to(self, target_col):
        if target_col == self.selected_col:
            # Drop in the same column = cancel selection
            self.selected_col = None
            self.selected_card_idx = None
            self.status_msg = "Selection cancelled."
            self.status_color = 4
            return

        # Attempt move
        if self.game.move_cards(self.selected_col, self.selected_card_idx, target_col):
            # Success!
            self.status_msg = f"Moved sequence to Column {target_col}."
            self.status_color = 5
            
            # Check for win!
            if self.game.is_won():
                self.status_msg = "CONGRATULATIONS! You won Spider Solitaire!"
                self.status_color = 5
            elif not self.game.has_moves_left():
                self.status_msg = "No moves left! Press U to undo, D to deal, or R to restart."
                self.status_color = 1

            self.cursor_col = target_col
            self.selected_col = None
            self.selected_card_idx = None
            self.adjust_cursor_after_col_change()
        else:
            # Failure
            self.status_msg = "Invalid move! Cards must go onto rank + 1 or empty column."
            self.status_color = 1

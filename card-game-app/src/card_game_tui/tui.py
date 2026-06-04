import sys
import curses
import locale
from .game import GameState, Card

# Set locale to support unicode suit symbols (♥, ♦, ♣, ♠, █)
try:
    locale.setlocale(locale.LC_ALL, '')
except Exception:
    pass  # Fallback to default locale if not supported

def draw_game(stdscr, state: GameState):
    stdscr.erase()
    
    # Get terminal dimensions
    max_y, max_x = stdscr.getmaxyx()
    
    if max_y < 24 or max_x < 80:
        try:
            stdscr.addstr(0, 0, "Terminal too small!", curses.color_pair(1) | curses.A_BOLD)
            stdscr.addstr(1, 0, f"Current: {max_x}x{max_y}. Required: 80x24 minimum.", curses.A_BOLD)
            stdscr.addstr(2, 0, "Please resize your terminal window to continue...", curses.A_DIM)
        except Exception:
            pass  # Fail-safe in case of tiny screens where writing causes crashes
        stdscr.refresh()
        return

    # Draw Header
    title = "♠ ♥ ♣ ♦  KLONDIKE SOLITAIRE  ♦ ♣ ♥ ♠"
    stdscr.addstr(0, (80 - len(title)) // 2, title, curses.color_pair(4) | curses.A_BOLD)
    stdscr.addstr(1, 0, "─" * 80, curses.color_pair(3))

    # Column configuration
    col_width = 10
    start_x = 4

    # --- Draw Row 0 (Stock, Waste, Spacer, Foundations) ---
    
    # Stock (Col 0)
    stock_x = start_x + 0 * col_width
    stdscr.addstr(2, stock_x, "[ Stock ]", curses.color_pair(3))
    stock_str = f"[ █ {len(state.stock):2d}]" if state.stock else "[  X  ]"
    is_cursor = (state.cursor_row == 0 and state.cursor_col == 0)
    is_selected = (state.selected is not None and state.selected[0] == 0 and state.selected[1] == 0)
    
    attr = curses.color_pair(3)
    if is_cursor:
        attr |= curses.A_REVERSE
    if is_selected:
        attr |= curses.A_BLINK
    stdscr.addstr(3, stock_x + 1, stock_str, attr)

    # Waste (Col 1)
    waste_x = start_x + 1 * col_width
    stdscr.addstr(2, waste_x, "[ Waste ]", curses.color_pair(3))
    is_cursor = (state.cursor_row == 0 and state.cursor_col == 1)
    is_selected = (state.selected is not None and state.selected[0] == 0 and state.selected[1] == 1)
    
    if state.waste:
        top_card = state.waste[-1]
        card_str = f"[ {top_card.rank_symbol:>2}{top_card.suit_symbol} ]"
        color_pair = 1 if top_card.color == 'red' else 2
        attr = curses.color_pair(color_pair)
        if is_cursor:
            attr |= curses.A_REVERSE
        if is_selected:
            attr |= curses.A_UNDERLINE
        stdscr.addstr(3, waste_x + 1, card_str, attr)
    else:
        attr = curses.color_pair(3)
        if is_cursor:
            attr |= curses.A_REVERSE
        stdscr.addstr(3, waste_x + 1, "[ Empty ]", attr)

    # Spacer (Col 2)
    spacer_x = start_x + 2 * col_width
    stdscr.addstr(2, spacer_x, "         ")
    stdscr.addstr(3, spacer_x, "         ")

    # Foundations (Col 3 to 6)
    for i in range(4):
        f_suit = state.foundation_suits[i]
        f_symbol = Card.SUITS[f_suit]
        f_x = start_x + (3 + i) * col_width
        stdscr.addstr(2, f_x, f"[  F{i+1} {f_symbol} ]", curses.color_pair(3))
        
        is_cursor = (state.cursor_row == 0 and state.cursor_col == 3 + i)
        is_selected = (state.selected is not None and state.selected[0] == 0 and state.selected[1] == 3 + i)
        
        f_pile = state.foundations[i]
        if f_pile:
            top_card = f_pile[-1]
            card_str = f"[ {top_card.rank_symbol:>2}{top_card.suit_symbol} ]"
            color_pair = 1 if top_card.color == 'red' else 2
            attr = curses.color_pair(color_pair)
            if is_cursor:
                attr |= curses.A_REVERSE
            if is_selected:
                attr |= curses.A_UNDERLINE
            stdscr.addstr(3, f_x + 1, card_str, attr)
        else:
            color_pair = 1 if f_suit in ('H', 'D') else 2
            attr = curses.color_pair(color_pair)
            if is_cursor:
                attr |= curses.A_REVERSE
            stdscr.addstr(3, f_x + 1, f"[  {f_symbol}  ]", attr)

    # --- Draw Row 1 (Tableaus 0 to 6) ---
    
    stdscr.addstr(5, 0, "─" * 80, curses.color_pair(3))
    
    for col in range(7):
        t_x = start_x + col * col_width
        stdscr.addstr(6, t_x, f"[ T{col+1} ]", curses.color_pair(3))
        
        pile = state.tableaus[col]
        if not pile:
            # Draw empty placeholder
            is_cursor = (state.cursor_row == 1 and state.cursor_col == col)
            attr = curses.color_pair(3)
            if is_cursor:
                attr |= curses.A_REVERSE
            stdscr.addstr(7, t_x + 1, "[ Empty ]", attr)
        else:
            for idx, card in enumerate(pile):
                is_cursor = (state.cursor_row == 1 and state.cursor_col == col and state.cursor_card_idx == idx)
                is_selected = (state.selected is not None and state.selected[0] == 1 and state.selected[1] == col and idx >= state.selected[2])
                
                card_y = 7 + idx
                # If too deep for screen, cap rendering and show indicators
                if card_y >= max_y - 4:
                    stdscr.addstr(max_y - 4, t_x + 1, "[  ...  ]", curses.color_pair(3))
                    break
                
                if card.face_up:
                    card_str = f"[ {card.rank_symbol:>2}{card.suit_symbol} ]"
                    color_pair = 1 if card.color == 'red' else 2
                    attr = curses.color_pair(color_pair)
                else:
                    card_str = "[  █  ]"
                    attr = curses.color_pair(3)
                
                if is_selected:
                    attr |= curses.A_UNDERLINE
                if is_cursor:
                    attr |= curses.A_REVERSE
                    
                stdscr.addstr(card_y, t_x + 1, card_str, attr)

    # --- Draw Footer / Information ---
    
    # Win State banner
    if state.check_win():
        win_msg = "★★★ CONGRATULATIONS! YOU WON THE GAME! Press 'R' to play again. ★★★"
        stdscr.addstr(max_y - 3, (80 - len(win_msg)) // 2, win_msg, curses.color_pair(4) | curses.A_BOLD | curses.A_BLINK)
    else:
        # Action/Message line
        stdscr.addstr(max_y - 3, 2, "Status: " + state.message[:75], curses.color_pair(4) | curses.A_BOLD)

    # Keyboard helper
    legend = "[Arrows/WASD] Move [Space/Enter] Select/Move [Esc/C] Clear [U] Undo [A] Auto [H] Hint [R] Rest [Q] Quit"
    stdscr.addstr(max_y - 2, (80 - len(legend)) // 2, legend, curses.A_DIM)
    
    stdscr.refresh()

def main_loop(stdscr):
    # Initialize Curses Colors
    curses.use_default_colors()
    curses.init_pair(1, curses.COLOR_RED, -1)     # Red suits
    curses.init_pair(2, curses.COLOR_WHITE, -1)   # Black suits
    curses.init_pair(3, curses.COLOR_CYAN, -1)    # Blue/Cyan borders & back
    curses.init_pair(4, curses.COLOR_YELLOW, -1)  # Yellow cursor / highlights
    
    # Hide standard cursor block
    try:
        curses.curs_set(0)
    except Exception:
        pass
        
    state = GameState()
    
    while True:
        draw_game(stdscr, state)
        
        try:
            ch = stdscr.getch()
        except KeyboardInterrupt:
            break
            
        if ch == -1:
            continue
            
        # Standardize inputs (support arrow keys, WASD, and letters)
        if ch == ord('q') or ch == ord('Q'):
            break
        elif ch == ord('r') or ch == ord('R'):
            state.reset()
        elif ch == ord('u') or ch == ord('U'):
            state.undo()
        elif ch == ord('a') or ch == ord('A'):
            state.auto_play_to_foundations()
        elif ch == ord('h') or ch == ord('H'):
            state.request_hint()
        elif ch in (curses.KEY_LEFT, ord('a'), ord('A')):
            state.move_cursor('left')
        elif ch in (curses.KEY_RIGHT, ord('d'), ord('D')):
            state.move_cursor('right')
        elif ch in (curses.KEY_UP, ord('w'), ord('W')):
            state.move_cursor('up')
        elif ch in (curses.KEY_DOWN, ord('s'), ord('S')):
            state.move_cursor('down')
        elif ch in (ord(' '), ord('\n'), curses.KEY_ENTER):
            state.select_or_move()
        elif ch in (27, ord('c'), ord('C')):  # 27 is Esc
            state.selected = None
            state.message = "Selection canceled."

def start_ui():
    curses.wrapper(main_loop)

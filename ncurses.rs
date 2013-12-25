#[crate_id="ncurses#5.7"];
use std::libc::{c_char, c_int, c_short, c_uchar, c_uint, c_void,FILE};

pub type chtype = c_uint;
pub type attr_t = c_int;
pub type NCURSES_ATTR_T = attr_t;

type attr_t_p = *attr_t;
type short_p = *c_short;
type void_p = *c_void;
type char_p = *c_char;
type chtype_p = *chtype;
type WINDOW_p = *WINDOW;
type SCREEN_p = *SCREEN;
type FILE_p = *FILE;

type va_list = *u8;

struct WINDOW;
struct SCREEN;

#[link(name = "ncurses")]
extern {
    pub static curscr: WINDOW_p;
    pub static newscr: WINDOW_p;
    pub static stdscr: WINDOW_p;
    pub static ttytype: *c_char;
    pub static COLORS : c_int;
    pub static COLOR_PAIRS : c_int;
    pub static COLS : c_int;
    pub static ESCDELAY : c_int;
    pub static LINES : c_int;
    pub static TABSIZE : c_int;

    pub fn addch (_:chtype) -> c_int;
    pub fn addchnstr (_:*chtype, _:c_int) -> c_int;
    pub fn addchstr (_:*chtype) -> c_int;
    pub fn addnstr (_:char_p, _:c_int) -> c_int;
    pub fn addstr (_:char_p) -> c_int;
    pub fn attroff (_:NCURSES_ATTR_T) -> c_int;
    pub fn attron (_:NCURSES_ATTR_T) -> c_int;
    pub fn attrset (_:NCURSES_ATTR_T) -> c_int;
    pub fn attr_get (_:attr_t_p, _:short_p, _:void_p) -> c_int;
    pub fn attr_off (_:attr_t, _:void_p) -> c_int;
    pub fn attr_on (_:attr_t, _:void_p) -> c_int;
    pub fn attr_set (_:attr_t, _:c_short, _:void_p) -> c_int;
    pub fn baudrate () -> c_int;
    pub fn beep  () -> c_int;
    pub fn bkgd (_:chtype) -> c_int;
    pub fn bkgdset (_:chtype);
    pub fn border (_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
    pub fn box_ (_:WINDOW_p, _:chtype, _:chtype) -> c_int;
    pub fn can_change_color () -> bool;
    pub fn cbreak () -> c_int;
    pub fn chgat (_:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    pub fn clear () -> c_int;
    pub fn clearok (_:WINDOW_p,_:bool) -> c_int;
    pub fn clrtobot () -> c_int;
    pub fn clrtoeol () -> c_int;
    pub fn color_content (_:c_short,_:short_p,_:short_p,_:short_p) -> c_int;
    pub fn color_set (_:c_short,_:void_p) -> c_int;
    pub fn COLOR_PAIR (_:c_int) -> c_int;
    pub fn copywin (_:WINDOW_p,_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    pub fn curs_set (_:c_int) -> c_int;
    pub fn def_prog_mode () -> c_int;
    pub fn def_shell_mode () -> c_int;
    pub fn delay_output (_:c_int) -> c_int;
    pub fn delch () -> c_int;
    pub fn delscreen (_:SCREEN_p);
    pub fn delwin (_:WINDOW_p) -> c_int;
    pub fn deleteln () -> c_int;
    pub fn derwin (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int) -> *WINDOW;
    pub fn doupdate () -> c_int;
    pub fn dupwin (_:WINDOW_p) -> *WINDOW;
    pub fn echo () -> c_int;
    pub fn echochar (_:chtype) -> c_int;
    pub fn erase () -> c_int;
    pub fn endwin () -> c_int;
    pub fn erasechar () -> c_char;
    pub fn filter ();
    pub fn flash () -> c_int;
    pub fn flushinp () -> c_int;
    pub fn getbkgd (_:WINDOW_p) -> chtype;
    pub fn getch () -> c_int;
    pub fn getnstr (_:char_p, _:c_int) -> c_int;
    pub fn getstr (_:char_p) -> c_int;
    pub fn getwin (_:FILE_p) -> *WINDOW;
    pub fn halfdelay (_:c_int) -> c_int;
    pub fn has_colors () -> bool;
    pub fn has_ic () -> bool;
    pub fn has_il () -> bool;
    pub fn hline (_:chtype, _:c_int) -> c_int;
    pub fn idcok (_:WINDOW_p, _:bool);
    pub fn idlok (_:WINDOW_p, _:bool) -> c_int;
    pub fn immedok (_:WINDOW_p, _:bool);
    pub fn inch () -> chtype;
    pub fn inchnstr (_:chtype_p, _:c_int) -> c_int;
    pub fn inchstr (_:chtype_p) -> c_int;
    pub fn initscr () -> *WINDOW;
    pub fn init_color (_:c_short,_:c_short,_:c_short,_:c_short) -> c_int;
    pub fn init_pair (_:c_short,_:c_short,_:c_short) -> c_int;
    pub fn innstr (_:char_p, _:c_int) -> c_int;
    pub fn insch (_:chtype) -> c_int;
    pub fn insdelln (_:c_int) -> c_int;
    pub fn insertln () -> c_int;
    pub fn insnstr (_:char_p, _:c_int) -> c_int;
    pub fn insstr (_:char_p) -> c_int;
    pub fn instr (_:char_p) -> c_int;
    pub fn intrflush (_:WINDOW_p,_:bool) -> c_int;
    pub fn isendwin () -> bool;
    pub fn is_linetouched (_:WINDOW_p,_:c_int) -> bool;
    pub fn is_wintouched (_:WINDOW_p) -> bool;
    pub fn keyname (_:c_int) -> *c_char;
    pub fn keypad (_:WINDOW_p, _:bool) -> c_int;
    pub fn killchar () -> char;
    pub fn leaveok (_:WINDOW_p,_:bool) -> c_int;
    pub fn longname () -> *c_char;
    pub fn meta (_:WINDOW_p,_:bool) -> c_int;
    pub fn move (_:c_int, _:c_int) -> c_int;
    pub fn mvaddch (_:c_int, _:c_int, _:chtype) -> c_int;
    pub fn mvaddchnstr (_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    pub fn mvaddchstr (_:c_int, _:c_int, _:chtype_p) -> c_int;
    pub fn mvaddnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    pub fn mvaddstr (_:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvchgat (_:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    pub fn mvcur (_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    pub fn mvdelch (_:c_int, _:c_int) -> c_int;
    pub fn mvderwin (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    pub fn mvgetch (_:c_int, _:c_int) -> c_int;
    pub fn mvgetnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    pub fn mvgetstr (_:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvhline (_:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    pub fn mvinch (_:c_int, _:c_int) -> chtype;
    pub fn mvinchnstr (_:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    pub fn mvinchstr (_:c_int, _:c_int, _:chtype_p) -> c_int;
    pub fn mvinnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    pub fn mvinsch (_:c_int, _:c_int, _:chtype) -> c_int;
    pub fn mvinsnstr (_:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    pub fn mvinsstr (_:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvinstr (_:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvprintw (_:c_int, _:c_int, _:char_p) -> c_int;
    // pub fn mvscanw (_:c_int,_:c_int, _:char_p) -> c_int;
    pub fn mvvline (_:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    pub fn mvwaddch (_:WINDOW_p, _:c_int, _:c_int, _:chtype) -> c_int;
    pub fn mvwaddchnstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    pub fn mvwaddchstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p) -> c_int;
    pub fn mvwaddnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    pub fn mvwaddstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvwchgat (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    pub fn mvwdelch (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    pub fn mvwgetch (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    pub fn mvwgetnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    pub fn mvwgetstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvwhline (_:WINDOW_p, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    pub fn mvwin (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    pub fn mvwinch (_:WINDOW_p, _:c_int, _:c_int) -> chtype;
    pub fn mvwinchnstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p, _:c_int) -> c_int;
    pub fn mvwinchstr (_:WINDOW_p, _:c_int, _:c_int, _:chtype_p) -> c_int;
    pub fn mvwinnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    pub fn mvwinsch (_:WINDOW_p, _:c_int, _:c_int, _:chtype) -> c_int;
    pub fn mvwinsnstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p, _:c_int) -> c_int;
    pub fn mvwinsstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvwinstr (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvwprintw (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;

    // pub fn mvwscanw (_:WINDOW_p, _:c_int, _:c_int, _:char_p) -> c_int;
    pub fn mvwvline (_:WINDOW_p, _:c_int, _:c_int, _:chtype, _:c_int) -> c_int;
    pub fn napms (_:c_int) -> c_int;
    pub fn newpad (_:c_int,_:c_int) -> *WINDOW;
    pub fn newterm (_:char_p,_:FILE_p,_:FILE_p) -> *SCREEN;
    pub fn newwin (_:c_int,_:c_int,_:c_int,_:c_int) -> *WINDOW;
    pub fn nl () -> c_int;
    pub fn nocbreak () -> c_int;
    pub fn nodelay (_:WINDOW_p,_:bool) -> c_int;
    pub fn noecho () -> c_int;
    pub fn nonl () -> c_int;
    pub fn noqiflush ();
    pub fn noraw () -> c_int;
    pub fn notimeout (_:WINDOW_p,_:bool) -> c_int;
    pub fn overlay (_:WINDOW_p,_:WINDOW_p) -> c_int;
    pub fn overwrite (_:WINDOW_p,_:WINDOW_p) -> c_int;
    pub fn pair_content (_:c_short,_:short_p,_:short_p) -> c_int;
    pub fn PAIR_NUMBER (_:c_int) -> c_int;
    pub fn pechochar (_:WINDOW_p, _:chtype) -> c_int;
    pub fn pnoutrefresh (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;
    pub fn prefresh (_:WINDOW_p,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int,_:c_int) -> c_int;

    pub fn printw (_:char_p) -> c_int;
    pub fn putwin (_:WINDOW_p, _:FILE_p) -> c_int;
    pub fn qiflush ();
    pub fn raw () -> c_int;
    pub fn redrawwin (_:WINDOW_p) -> c_int;
    pub fn refresh () -> c_int;
    pub fn resetty () -> c_int;
    pub fn reset_prog_mode () -> c_int;
    pub fn reset_shell_mode () -> c_int;
    //pub fn ripoffline (_:c_int, extern unsafe fn f(WINDOW_p, c_int) -> c_int) -> c_int;
    pub fn ripoffline (_:c_int, f:*u8) -> c_int;
    pub fn savetty () -> c_int;
    // pub fn scanw (_:NCURSES_CONST char_p,...) -> c_int;
    pub fn scr_dump (_:char_p) -> c_int;
    pub fn scr_init (_:char_p) -> c_int;
    pub fn scrl (_:c_int) -> c_int;
    pub fn scroll (_:WINDOW_p) -> c_int;
    pub fn scrollok (_:WINDOW_p,_:bool) -> c_int;
    pub fn scr_restore (_:char_p) -> c_int;
    pub fn scr_set (_:char_p) -> c_int;
    pub fn setscrreg (_:c_int,_:c_int) -> c_int;
    pub fn set_term (_:SCREEN_p) -> SCREEN_p;
    pub fn slk_attroff (_:chtype) -> c_int;
    pub fn slk_attr_off (_:attr_t, _:void_p) -> c_int;
    pub fn slk_attron (_:chtype) -> c_int;
    pub fn slk_attr_on (_:attr_t,_:void_p) -> c_int;
    pub fn slk_attrset (_:chtype) -> c_int;
    pub fn slk_attr () -> attr_t;
    pub fn slk_attr_set (_:attr_t,_:c_short,_:void_p) -> c_int;
    pub fn slk_clear () -> c_int;
    pub fn slk_color (_:c_short) -> c_int;
    pub fn slk_init (_:c_int) -> c_int;
    pub fn slk_label (_:c_int) -> char_p; 
    pub fn slk_noutrefresh () -> c_int;
    pub fn slk_refresh () -> c_int;
    pub fn slk_restore () -> c_int;
    pub fn slk_set (_:c_int,_:char_p,_:c_int) -> c_int;
    pub fn slk_touch () -> c_int;
    pub fn standout () -> c_int;
    pub fn standend () -> c_int;
    pub fn start_color () -> c_int;
    pub fn subpad (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW_p;
    pub fn subwin (_:WINDOW_p, _:c_int, _:c_int, _:c_int, _:c_int) -> WINDOW_p;
    pub fn syncok (_:WINDOW_p, _:bool) -> c_int;
    pub fn termattrs () -> chtype;
    pub fn termname () -> char_p;
    pub fn timeout (_:c_int);
    pub fn touchline (_:WINDOW_p, _:c_int, _:c_int) -> c_int;
    pub fn touchwin (_:WINDOW_p) -> c_int;
    pub fn typeahead (_:c_int) -> c_int;
    pub fn ungetch (_:c_int) -> c_int;
    pub fn untouchwin (_:WINDOW_p) -> c_int;
    pub fn use_env (_:bool);
    pub fn vidattr (_:chtype) -> c_int;
    // pub fn vidputs (_:chtype, extern unsafe fn f(c_int) -> c_int) -> c_int;
    pub fn vidputs (_:chtype, f:*u8) -> c_int;
    pub fn vline (_:chtype, _:c_int) -> c_int;
    pub fn vwprintw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    pub fn vw_printw (_:WINDOW_p, _:char_p,_:va_list) -> c_int;
    // pub fn vwscanw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    // pub fn vw_scanw (_:WINDOW_p, _:char_p, _:va_list) -> c_int;
    pub fn waddch (_:WINDOW_p, _:chtype) -> c_int;
    pub fn waddchnstr (_:WINDOW_p,_:chtype_p,_:c_int) -> c_int;
    pub fn waddchstr (_:WINDOW_p,_:chtype_p) -> c_int;
    pub fn waddnstr (_:WINDOW_p,_:char_p,_:c_int) -> c_int;
    pub fn waddstr (_:WINDOW_p,_:char_p) -> c_int;
    pub fn wattron (_:WINDOW_p, _:c_int) -> c_int;
    pub fn wattroff (_:WINDOW_p, _:c_int) -> c_int;
    pub fn wattrset (_:WINDOW_p, _:c_int) -> c_int;
    pub fn wattr_get (_:WINDOW_p, _:attr_t_p, _:short_p, _:void_p) -> c_int;
    pub fn wattr_on (_:WINDOW_p, _:attr_t, _:void_p) -> c_int;
    pub fn wattr_off (_:WINDOW_p, _:attr_t, _:void_p) -> c_int;
    pub fn wattr_set (_:WINDOW_p, _:attr_t, _:c_short, _:void_p) -> c_int;
    pub fn wbkgd (_:WINDOW_p, _:chtype) -> c_int;
    pub fn wbkgdset (_:WINDOW_p,_:chtype);
    pub fn wborder (_:WINDOW_p,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype,_:chtype) -> c_int;
    pub fn wchgat (_:WINDOW_p, _:c_int, _:attr_t, _:c_short, _:void_p) -> c_int;
    pub fn wclear (_:WINDOW_p) -> c_int;
    pub fn wclrtobot (_:WINDOW_p) -> c_int;
    pub fn wclrtoeol (_:WINDOW_p) -> c_int;
    pub fn wcolor_set (_:WINDOW_p,_:c_short,_:void_p) -> c_int;
    pub fn wcursyncup (_:WINDOW_p);
    pub fn wdelch (_:WINDOW_p) -> c_int;
    pub fn wdeleteln (_:WINDOW_p) -> c_int;
    pub fn wechochar (_:WINDOW_p, _:chtype) -> c_int;
    pub fn werase (_:WINDOW_p) -> c_int;
    pub fn wgetch (_:WINDOW_p) -> c_int;
    pub fn wgetnstr (_:WINDOW_p,_:char_p,_:c_int) -> c_int;
    pub fn wgetstr (_:WINDOW_p, _:char_p) -> c_int;
    pub fn whline (_:WINDOW_p, _:chtype, _:c_int) -> c_int;
    pub fn winch (_:WINDOW_p) -> chtype;
    pub fn winchnstr (_:WINDOW_p, _:chtype_p, _:c_int) -> c_int;
    pub fn winchstr (_:WINDOW_p, _:chtype_p) -> c_int;
    pub fn winnstr (_:WINDOW_p, _:char_p, _:c_int) -> c_int;
    pub fn winsch (_:WINDOW_p, _:chtype) -> c_int;
    pub fn winsdelln (_:WINDOW_p,_:c_int) -> c_int;
    pub fn winsertln (_:WINDOW_p) -> c_int;
    pub fn winsnstr (_:WINDOW_p, _:char_p,_:c_int) -> c_int;
    pub fn winsstr (_:WINDOW_p, _:char_p) -> c_int;
    pub fn winstr (_:WINDOW_p, _:char_p) -> c_int;
    pub fn wmove (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    pub fn wnoutrefresh (_:WINDOW_p) -> c_int;
    pub fn wprintw (_:WINDOW_p, _:char_p) -> c_int;
    pub fn wredrawln (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    pub fn wrefresh (_:WINDOW_p) -> c_int;
    // pub fn wscanw (_:WINDOW_p, _:NCURSES_CONST char_p) -> c_int;
    pub fn wscrl (_:WINDOW_p,_:c_int) -> c_int;
    pub fn wsetscrreg (_:WINDOW_p,_:c_int,_:c_int) -> c_int;
    pub fn wstandout (_:WINDOW_p) -> c_int;
    pub fn wstandend (_:WINDOW_p) -> c_int;
    pub fn wsyncdown (_:WINDOW_p);
    pub fn wsyncup (_:WINDOW_p);
    pub fn wtimeout (_:WINDOW_p,_:c_int);
    pub fn wtouchln (_:WINDOW_p,_:c_int,_:c_int,_:c_int) -> c_int;
    pub fn wvline (_:WINDOW_p,_:chtype,_:c_int) -> c_int;

    /*
     * These are also declared in <term.h>:
     */
    pub fn tigetflag (_:char_p) -> c_int;
    pub fn tigetnum (_:char_p) -> c_int;
    pub fn tigetstr (_:char_p) -> *c_char;
    pub fn putp (_:char_p) -> c_int;

    pub fn tparm (_:char_p) -> *c_char;

/*
 * These functions are not in X/Open, but we use them in macro definitions:
 */
    pub fn getattrs (_:WINDOW_p) -> c_int;
    pub fn getcurx (_:WINDOW_p) -> c_int;
    pub fn getcury (_:WINDOW_p) -> c_int;
    pub fn getbegx (_:WINDOW_p) -> c_int;
    pub fn getbegy (_:WINDOW_p) -> c_int;
    pub fn getmaxx (_:WINDOW_p) -> c_int;
    pub fn getmaxy (_:WINDOW_p) -> c_int;
    pub fn getparx (_:WINDOW_p) -> c_int;
    pub fn getpary (_:WINDOW_p) -> c_int;
}

/*
 * These functions are extensions - not in X/Open Curses.
 */
extern {
/*
extern NCURSES_EXPORT(bool) is_term_resized (int, int);
extern NCURSES_EXPORT(char *) keybound (int, int);
extern NCURSES_EXPORT(const char *) curses_version (void);
extern NCURSES_EXPORT(int) assume_default_colors (int, int);
extern NCURSES_EXPORT(int) define_key (const char *, int);
extern NCURSES_EXPORT(int) key_defined (const char *);
extern NCURSES_EXPORT(int) keyok (int, bool);
extern NCURSES_EXPORT(int) resize_term (int, int);
extern NCURSES_EXPORT(int) resizeterm (int, int);
extern NCURSES_EXPORT(int) set_escdelay (int);
extern NCURSES_EXPORT(int) set_tabsize (int);
extern NCURSES_EXPORT(int) use_default_colors (void);
extern NCURSES_EXPORT(int) use_extended_names (bool);
extern NCURSES_EXPORT(int) use_legacy_coding (int);
//typedef int (*NCURSES_WINDOW_CB)(WINDOW *, void *);
//extern NCURSES_EXPORT(int) use_screen (SCREEN *, NCURSES_SCREEN_CB, void *);
//typedef int (*NCURSES_SCREEN_CB)(SCREEN *, void *);
//extern NCURSES_EXPORT(int) use_window (WINDOW *, NCURSES_WINDOW_CB, void *);
extern NCURSES_EXPORT(int) wresize (WINDOW *, int, int);
extern NCURSES_EXPORT(void) nofilter(void);
*/
}

/*
 * These extensions provide access to information stored in the WINDOW even
 * when NCURSES_OPAQUE is set:
 */
extern {
pub fn wgetparent (_:WINDOW_p) -> WINDOW_p;
pub fn is_cleared (_:WINDOW_p) -> bool;
pub fn is_idcok (_:WINDOW_p) -> bool;
pub fn is_idlok (_:WINDOW_p) -> bool;
pub fn is_immedok (_:WINDOW_p) -> bool;
pub fn is_keypad (_:WINDOW_p) -> bool;
pub fn is_leaveok (_:WINDOW_p) -> bool;
pub fn is_nodelay (_:WINDOW_p) -> bool;
pub fn is_notimeout (_:WINDOW_p) -> bool;
pub fn is_scrollok (_:WINDOW_p) -> bool;
pub fn is_syncok (_:WINDOW_p) -> bool;
pub fn wgetscrreg (_:WINDOW_p, _:*c_int, _:*c_int) -> c_int;

}

/* attributes */

static NCURSES_ATTR_SHIFT  : uint =       8;
fn NCURSES_BITS(mask:uint,shift:uint) -> uint { ((mask) << ((shift) + NCURSES_ATTR_SHIFT) as i32) }

pub fn A_NORMAL() -> c_int		{ (1u - 1u) as c_int }
pub fn A_ATTRIBUTES() -> c_int	{ NCURSES_BITS(!(1u - 1u),0)  as c_int }
pub fn A_CHARTEXT() -> c_int	{ (NCURSES_BITS(1u,0) - 1u)  as c_int }
pub fn A_COLOR() -> c_int		{ NCURSES_BITS(((1u) << 8) - 1u,0)  as c_int }
pub fn A_STANDOUT() -> c_int	{ NCURSES_BITS(1u,8) as c_int }
pub fn A_UNDERLINE() -> c_int	{ NCURSES_BITS(1u,9)  as c_int }
pub fn A_REVERSE() -> c_int	{ NCURSES_BITS(1u,10)  as c_int }
pub fn A_BLINK() -> c_int		{ NCURSES_BITS(1u,11)  as c_int }
pub fn A_DIM() -> c_int		{ NCURSES_BITS(1u,12)  as c_int }
pub fn A_BOLD() -> c_int		{ NCURSES_BITS(1u,13)  as c_int }
pub fn A_ALTCHARSET() -> c_int	{ NCURSES_BITS(1u,14)  as c_int }
pub fn A_INVIS() -> c_int		{ NCURSES_BITS(1u,15)  as c_int }
pub fn A_PROTECT() -> c_int	{ NCURSES_BITS(1u,16)  as c_int }
pub fn A_HORIZONTAL() -> c_int	{ NCURSES_BITS(1u,17)  as c_int }
pub fn A_LEFT() -> c_int		{ NCURSES_BITS(1u,18)  as c_int }
pub fn A_LOW() -> c_int		{ NCURSES_BITS(1u,19) as c_int }
pub fn A_RIGHT() -> c_int		{ NCURSES_BITS(1u,20)  as c_int }
pub fn A_TOP() -> c_int		{ NCURSES_BITS(1u,21)  as c_int }
pub fn A_VERTICAL() -> c_int	{ NCURSES_BITS(1u,22)  as c_int }

/*
 * Most of the pseudo functions are macros that either provide compatibility
 * with older versions of curses, or provide inline functionality to improve
 * performance.
 */

/*
 * These pseudo functions are always implemented as macros:
 */

pub unsafe fn getyx(win:WINDOW_p,y: &mut c_int,x: &mut c_int) { *y = getcury(win); *x = getcurx(win); }
pub unsafe fn getbegyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getbegy(win); *x = getbegx(win) }
pub unsafe fn getmaxyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getmaxy(win); *x = getmaxx(win) }
pub unsafe fn getparyx(win:WINDOW_p,y: &mut c_int, x: &mut c_int) { *y = getpary(win); *x = getparx(win) }

pub unsafe fn getsyx(y:&mut c_int, x:&mut c_int) {
    if newscr != (0 as WINDOW_p) {
        if is_leaveok(newscr) {
            *x = -1 as c_int;
            *y = -1 as c_int;
        } else {
            getyx(newscr,(y), (x));
        }
    }
}


pub unsafe fn setsyx(y:&mut c_int,x:&mut c_int) {
    if newscr != (0 as WINDOW_p) {
        if *y == -1 && *x == -1 {
            leaveok(newscr, true);
        } else {
            leaveok(newscr, false);
            wmove(newscr, *y, *x);
        }
    }
}

//#define setterm(term)		setupterm(term, 1, (int *)0)

//#define fixterm()		reset_prog_mode()
//#define resetterm()		reset_shell_mode()
//#define saveterm()		def_prog_mode()
//#define crmode()		cbreak()
//#define nocrmode()		nocbreak()
//#define gettmode()

/* colors */
pub static COLOR_BLACK : c_short =   0;
pub static COLOR_RED : c_short =     1;
pub static COLOR_GREEN : c_short =   2;
pub static COLOR_YELLOW : c_short =  3;
pub static COLOR_BLUE : c_short =    4;
pub static COLOR_MAGENTA : c_short = 5;
pub static COLOR_CYAN : c_short =    6;
pub static COLOR_WHITE : c_short =   7;

/* line graphics */

extern {
    pub static acs_map : chtype_p;
}

unsafe fn NCURSES_ACS(c:char) -> c_uchar {
    *((acs_map as i32 + 4*(c as c_int)) as chtype_p) as c_uchar
}

/* VT100 symbols begin here */
pub unsafe fn ACS_ULCORNER() -> c_uchar { NCURSES_ACS('l') } /* upper left corner */
pub unsafe fn ACS_LLCORNER() -> c_uchar { NCURSES_ACS('m') } /* lower left corner */
pub unsafe fn ACS_URCORNER() -> c_uchar { NCURSES_ACS('k') } /* upper right corner */
pub unsafe fn ACS_LRCORNER() -> c_uchar { NCURSES_ACS('j') } /* lower right corner */
pub unsafe fn ACS_LTEE() -> c_uchar { NCURSES_ACS('t') } /* tee pointing right */
pub unsafe fn ACS_RTEE() -> c_uchar { NCURSES_ACS('u') } /* tee pointing left */
pub unsafe fn ACS_BTEE() -> c_uchar { NCURSES_ACS('v') } /* tee pointing up */
pub unsafe fn ACS_TTEE() -> c_uchar { NCURSES_ACS('w') } /* tee pointing down */
pub unsafe fn ACS_HLINE() -> c_uchar { NCURSES_ACS('q') } /* horizontal line */
pub unsafe fn ACS_VLINE() -> c_uchar { NCURSES_ACS('x') } /* vertical line */
pub unsafe fn ACS_PLUS() -> c_uchar { NCURSES_ACS('n') } /* large plus or crossover */
pub unsafe fn ACS_S1() -> c_uchar	 { NCURSES_ACS('o') } /* scan line 1 */
pub unsafe fn ACS_S9() -> c_uchar	 { NCURSES_ACS('s') } /* scan line 9 */
pub unsafe fn ACS_DIAMOND() -> c_uchar { NCURSES_ACS('`') } /* diamond */
pub unsafe fn ACS_CKBOARD() -> c_uchar { NCURSES_ACS('a') } /* checker board (stipple) */
pub unsafe fn ACS_DEGREE() -> c_uchar { NCURSES_ACS('f') } /* degree symbol */
pub unsafe fn ACS_PLMINUS() -> c_uchar { NCURSES_ACS('g') } /* plus/minus */
pub unsafe fn ACS_BULLET() -> c_uchar { NCURSES_ACS('~') } /* bullet */
/* Teletype 5410v1 symbols begin here */
pub unsafe fn ACS_LARROW() -> c_uchar { NCURSES_ACS(',') } /* arrow pointing left */
pub unsafe fn ACS_RARROW() -> c_uchar { NCURSES_ACS('+') } /* arrow pointing right */
pub unsafe fn ACS_DARROW() -> c_uchar { NCURSES_ACS('.') } /* arrow pointing down */
pub unsafe fn ACS_UARROW() -> c_uchar { NCURSES_ACS('-') } /* arrow pointing up */
pub unsafe fn ACS_BOARD() -> c_uchar { NCURSES_ACS('h') } /* board of squares */
pub unsafe fn ACS_LANTERN() -> c_uchar { NCURSES_ACS('i') } /* lantern symbol */
pub unsafe fn ACS_BLOCK() -> c_uchar { NCURSES_ACS('0') } /* solid square block */
/*
 * These aren't documented, but a lot of System Vs have them anyway
 * (you can spot pprryyzz{{||}} in a lot of AT&T terminfo strings).
 * The ACS_names may not match AT&T's, our source didn't know them.
 */
pub unsafe fn ACS_S3() -> c_uchar	 { NCURSES_ACS('p') } /* scan line 3 */
pub unsafe fn ACS_S7() -> c_uchar	 { NCURSES_ACS('r') } /* scan line 7 */
pub unsafe fn ACS_LEQUAL() -> c_uchar { NCURSES_ACS('y') } /* less/equal */
pub unsafe fn ACS_GEQUAL() -> c_uchar { NCURSES_ACS('z') } /* greater/equal */
pub unsafe fn ACS_PI() -> c_uchar	 { NCURSES_ACS('{') } /* Pi */
pub unsafe fn ACS_NEQUAL() -> c_uchar { NCURSES_ACS('|') } /* not equal */
pub unsafe fn ACS_STERLING() -> c_uchar { NCURSES_ACS('}') } /* UK pound sign */

/*
 * Line drawing ACS names are of the form ACS_trbl, where t is the top, r
 * is the right, b is the bottom, and l is the left.  t, r, b, and l might
 * be B (blank), S (single), D (double), or T (thick).  The subset defined
 * here only uses B and S.
 */
pub unsafe fn ACS_BSSB() -> c_uchar { ACS_ULCORNER() }
pub unsafe fn ACS_SSBB() -> c_uchar { ACS_LLCORNER() }
pub unsafe fn ACS_BBSS() -> c_uchar { ACS_URCORNER() }
pub unsafe fn ACS_SBBS() -> c_uchar { ACS_LRCORNER() }
pub unsafe fn ACS_SBSS() -> c_uchar { ACS_RTEE() }
pub unsafe fn ACS_SSSB() -> c_uchar { ACS_LTEE() }
pub unsafe fn ACS_SSBS() -> c_uchar { ACS_BTEE() }
pub unsafe fn ACS_BSSS() -> c_uchar { ACS_TTEE() }
pub unsafe fn ACS_BSBS() -> c_uchar { ACS_HLINE() }
pub unsafe fn ACS_SBSB() -> c_uchar { ACS_VLINE() }
pub unsafe fn ACS_SSSS() -> c_uchar { ACS_PLUS() }


pub static ERR:c_int =     (-1);

pub static OK:c_int =      (0);

/* values for the _flags member */
pub static _SUBWIN:c_int =         0x01;	/* is this a sub-window? */
pub static _ENDLINE:c_int =       0x02;	/* is the window flush right? */
pub static _FULLWIN:c_int  =      0x04;	/* is the window full-screen? */
pub static _SCROLLWIN:c_int =     0x08;	/* bottom edge is at screen bottom? */
pub static _ISPAD:c_int	  =      0x10;	/* is this window a pad? */
pub static _HASMOVED:c_int    =   0x20;	/* has cursor moved since last refresh? */
pub static _WRAPPED:c_int      =  0x40;	/* cursor was just wrappped */

/*
 * this value is used in the firstchar and lastchar fields to mark
 * unchanged lines
 */
pub static _NOCHANGE:c_int =       -1;

/*
 * this value is used in the oldindex field to mark lines created by insertions
 * and scrolls.
 */
pub static _NEWINDEX:c_int =	-1;

/*
 * Pseudo-character tokens outside ASCII range.  The curses wgetch() function
 * will return any given one of these only if the corresponding k- capability
 * is defined in your terminal's terminfo entry.
 *
 * Some keys (KEY_A1, etc) are arranged like this:
 *	a1     up    a3
 *	left   b2    right
 *	c1     down  c3
 *
 * A few key codes do not depend upon the terminfo entry.
 */
pub static KEY_CODE_YES:c_int =	0x100;		/* A wchar_t contains a key code */
pub static KEY_MIN:c_int =		0x101;		/* Minimum curses key */
pub static KEY_BREAK:c_int =	0x101;		/* Break key (unreliable) */
pub static KEY_SRESET:c_int =	0x158;		/* Soft (partial) reset (unreliable) */
pub static KEY_RESET:c_int =	0x159;		/* Reset or hard reset (unreliable) */
/*
 * These definitions were generated by /tmp/ncurses-27.roots/ncurses-27/ncurses/include/MKkey_defs.sh /tmp/ncurses-27.roots/ncurses-27/ncurses/include/Caps
 */
pub static KEY_DOWN:c_int =	0x102;		/* down-arrow key */
pub static KEY_UP:c_int=		0x103;		/* up-arrow key */
pub static KEY_LEFT:c_int=	0x104;		/* left-arrow key */
pub static KEY_RIGHT:c_int=	0x105;		/* right-arrow key */
pub static KEY_HOME:c_int=	0x106;		/* home key */
pub static KEY_BACKSPACE:c_int=	0x107;		/* backspace key */
pub static KEY_F0:c_int=		0x108;		/* Function keys.  Space for 64 */
pub static KEY_F1:c_int=		0x109;
pub static KEY_F2:c_int=		0x10a;
pub static KEY_F3:c_int=		0x10b;
pub static KEY_F4:c_int=		0x10c;
pub static KEY_F5:c_int=		0x10d;
pub static KEY_F6:c_int=		0x10e;
pub static KEY_F7:c_int=		0x10f;
pub static KEY_F8:c_int=		0x110;
pub static KEY_F9:c_int=		0x111;
pub static KEY_F10:c_int=		0x112;
pub static KEY_F11:c_int=		0x113;
pub static KEY_F12:c_int=		0x114;
pub static KEY_F13:c_int=		0x115;
pub static KEY_F14:c_int=		0x116;
pub static KEY_F15:c_int=		0x117;
//static KEY_F(n)	(KEY_F0+(n))	/* Value of function key n */

pub static KEY_DL:c_int=		0x148;		/* delete-line key */
pub static KEY_IL:c_int=		0x149;		/* insert-line key */
pub static KEY_DC:c_int=		0x150;		/* delete-character key */
pub static KEY_IC:c_int=		0x151;		/* insert-character key */
pub static KEY_EIC:c_int=		0x152;		/* sent by rmir or smir in insert mode */
pub static KEY_CLEAR:c_int=	0x14d;		/* clear-screen or erase key */
pub static KEY_EOS:c_int=		0x14e;		/* clear-to-end-of-screen key */
pub static KEY_EOL:c_int=		0x14f;		/* clear-to-end-of-line key */
pub static KEY_SF:c_int=		0x150;		/* scroll-forward key */
pub static KEY_SR:c_int=		0x151;		/* scroll-backward key */
pub static KEY_NPAGE:c_int=	0x152;		/* next-page key */
pub static KEY_PPAGE:c_int=	0x153;		/* previous-page key */
pub static KEY_STAB:c_int=	0x154;		/* set-tab key */
pub static KEY_CTAB:c_int=	0x155;		/* clear-tab key */
pub static KEY_CATAB:c_int=	0x156;		/* clear-all-tabs key */
pub static KEY_ENTER:c_int=	0x157;		/* enter/send key */
pub static KEY_PRINT:c_int=	0x15a;		/* print key */
pub static KEY_LL:c_int=		0x15b;		/* lower-left key (home down) */
pub static KEY_A1:c_int=		0x15c;		/* upper left of keypad */
pub static KEY_A3:c_int=		0x15d;		/* upper right of keypad */
pub static KEY_B2:c_int=		0x15e;		/* center of keypad */
pub static KEY_C1:c_int=		0x15f;		/* lower left of keypad */
pub static KEY_C3:c_int=		0x160;		/* lower right of keypad */
pub static KEY_BTAB:c_int=	0x161;		/* back-tab key */
pub static KEY_BEG:c_int=		0x162;		/* begin key */
pub static KEY_CANCEL:c_int=	0x163;		/* cancel key */
pub static KEY_CLOSE:c_int=	0x164;		/* close key */
pub static KEY_COMMAND:c_int=	0x165;		/* command key */
pub static KEY_COPY:c_int=	0x166;		/* copy key */
pub static KEY_CREATE:c_int=	0x167;		/* create key */
pub static KEY_END:c_int=		0x168;		/* end key */
pub static KEY_EXIT:c_int=	0x169;		/* exit key */
pub static KEY_FIND:c_int=	0x16a;		/* find key */
pub static KEY_HELP:c_int=	0x16b;		/* help key */
pub static KEY_MARK:c_int=	0x16c;		/* mark key */
pub static KEY_MESSAGE:c_int=	0x16d;		/* message key */
pub static KEY_MOVE:c_int=	0x16e;		/* move key */
pub static KEY_NEXT:c_int=	0x16f;		/* next key */
pub static KEY_OPEN:c_int=	0x170;		/* open key */
pub static KEY_OPTIONS:c_int=	0x171;		/* options key */
pub static KEY_PREVIOUS:c_int=	0x172;		/* previous key */
pub static KEY_REDO:c_int=	0x173;		/* redo key */
pub static KEY_REFERENCE:c_int=	0x174;		/* reference key */
pub static KEY_REFRESH:c_int=	0x175;		/* refresh key */
pub static KEY_REPLACE:c_int=	0x176;		/* replace key */
pub static KEY_RESTART:c_int=	0x177;		/* restart key */
pub static KEY_RESUME:c_int=	0x178;		/* resume key */
pub static KEY_SAVE:c_int=	0x179;		/* save key */
pub static KEY_SBEG:c_int=	0x17a;		/* shifted begin key */
pub static KEY_SCANCEL:c_int=	0x17b;		/* shifted cancel key */
pub static KEY_SCOMMAND:c_int=	0x17c;		/* shifted command key */
pub static KEY_SCOPY:c_int=	0x17d;		/* shifted copy key */
pub static KEY_SCREATE:c_int=	0x17e;		/* shifted create key */
pub static KEY_SDC	:c_int=	0x17f;		/* shifted delete-character key */
pub static KEY_SDL	:c_int=	0x180;		/* shifted delete-line key */
pub static KEY_SELECT:c_int=	0x181;		/* select key */
pub static KEY_SEND:c_int=	0x182;		/* shifted end key */
pub static KEY_SEOL:c_int=	0x183;		/* shifted clear-to-end-of-line key */
pub static KEY_SEXIT:c_int=	0x184;		/* shifted exit key */
pub static KEY_SFIND:c_int=	0x185;		/* shifted find key */
pub static KEY_SHELP:c_int=	0x186;		/* shifted help key */
pub static KEY_SHOME:c_int=	0x187;		/* shifted home key */
pub static KEY_SIC	:c_int=	0x188;		/* shifted insert-character key */
pub static KEY_SLEFT:c_int=	0x189;		/* shifted left-arrow key */
pub static KEY_SMESSAGE:c_int=	0x18a;		/* shifted message key */
pub static KEY_SMOVE:c_int=	0x18b;		/* shifted move key */
pub static KEY_SNEXT:c_int=	0x18c;		/* shifted next key */
pub static KEY_SOPTIONS:c_int=	0x18d;		/* shifted options key */
pub static KEY_SPREVIOUS:c_int=	0x18e;		/* shifted previous key */
pub static KEY_SPRINT:c_int=	0x18f;		/* shifted print key */
pub static KEY_SREDO:c_int=	0x190;		/* shifted redo key */
pub static KEY_SREPLACE:c_int=	0x191;		/* shifted replace key */
pub static KEY_SRIGHT:c_int=	0x192;		/* shifted right-arrow key */
pub static KEY_SRSUME:c_int=	0x193;		/* shifted resume key */
pub static KEY_SSAVE:c_int=	0x194;		/* shifted save key */
pub static KEY_SSUSPEND:c_int=	0x195;		/* shifted suspend key */
pub static KEY_SUNDO:c_int=	0x196;		/* shifted undo key */
pub static KEY_SUSPEND:c_int=	0x197;		/* suspend key */
pub static KEY_UNDO:c_int=	0x198;		/* undo key */
pub static KEY_MOUSE:c_int=	0x199;		/* Mouse event has occurred */
pub static KEY_RESIZE:c_int=	0x19a;		/* Terminal resize event */
pub static KEY_EVENT:c_int=	0x19b;		/* We were interrupted by an event */

pub static KEY_MAX	:c_int=	0x1ff;		/* Maximum key value is 0633 */

// FSK skipping functions involving cchar_t which has "interesting"
// structure and which are all guarded by #ifdef
// _XOPEN_SOURCE_EXTENDED

// FSK skipping mouse support material
/* $Id: curses.tail,v 1.16 2008/07/05 20:20:38 tom Exp $ */

extern mod ncurses;

fn main() {
    use ncurses::{initscr,stdscr,raw,keypad,noecho,printw,getch,refresh,endwin};
    use ncurses::{attron,attroff,A_BOLD};
    use ncurses::KEY_F1;
    use std::libc;

    unsafe {
        // raw(), cbreak(): turn off terminal buffering.  raw() passes
        // ctrl chars (^Z, ^C) through; cbreak() leaves interpretation
        // to terminal driver.
        // echo(), noecho(): control echoing of chars to terminal.
        // keypad(): enable reading of fcn keys, arrow keys, etc.
        // halfdelay(): This is some mode where input functions will only
        // wait for a timeout and then return ERR

        let ch:libc::c_int;
        initscr();                    /* Start curses mode      */
        raw();
        keypad(stdscr, true);
        noecho();


        "Type any character to see it in bold\n".with_c_str(|m| {
            printw(m);
        });

        ch = getch(); // Without raw the input would be buffered to line break

        if ch == KEY_F1 { // Without keypad we'd miss F1
            "F1 Key pressed".with_c_str(|m| printw(m));
        } else {
            "The pressed key is".with_c_str(|m| printw(m));
            attron(A_BOLD());
            // FSK: actual invocation was printw("%c", ch);
            // FSK: I am now curious as to whether curses needs to
            // FSK: muck with the internals within format strings,
            // FSK: and thus my simplification of passing one (fmt!'ed) string
            // FSK: will be broken.
            let ch = std::char::from_u32(ch as u32).unwrap();
            (format!("{}", ch)).with_c_str(|m| printw(m));
            attroff(A_BOLD());
        }
        refresh();
        getch();
        endwin();                     /* Terminate and cleanup  */
    }
}

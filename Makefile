ncurses-lib-built: ncurses.rs
	rustc --lib $<
	touch $@

hello%: hello%.rs ncurses-lib-built
	rustc -L. $<

clean:
	rm -f libncurses-*.dylib
	rm -rf *.dSYM
	rm -f ncurses-lib-built
	rm -f hello3 hello4

.PHONY: clean

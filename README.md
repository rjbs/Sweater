# SWTR: Software Tools in Rust

Software Tools is [Brian W.
Kernighan](https://en.wikipedia.org/wiki/Brian_Kernighan) and [P. J.
Plauger](https://en.wikipedia.org/wiki/P._J._Plauger)'s classic book about how
to write good programs that are good tools.  It was originally published with
programs presented in Ratfor, a language built on top of Fortran.  Later,
Kernighan and Plauger published _Software Tools in Pascal_, which adapted the
software tools to Pascal and led Kernighan to write [Why Pascal is Not My
Favorite Programming Language](http://www.lysator.liu.se/c/bwk-on-pascal.html)

The programs in Software Tools are simple, but they are not trivial.  They do
things that real programs would have to do, and they're open-ended, leaving the
programmer with room to add more useful behaviors.

[Mark Jason Dominus](http://blog.plover.com/) once told me about [Software
Tools in Haskell](http://www.crsr.net/Programming_Languages/SoftwareTools/),
which undertook to produce the Software Tools programs in Haskell.
(Obviously).  It struck me that these would be good programs for getting a
handle on a new language, and I kept meaning to test this out.

This repository is my attempt to finally do so, by writing (so far terrible)
implementations of the Software Tools in Rust.

The contents of `./t` are basic tests for the programs, written in Perl 5.  My
hope is that these tests will be useful in future Software Tools in *Whatever*
projects.

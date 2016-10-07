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

# The Tools

This is a list, in book order, of the software tools.  Names and summaries are
sometimes my own, giving the more familiar UNIX name in place of the book's
name.

## Chapter 1: Getting Started

* `cp`: copy a file to a new name
* `wc`: count the number of bytes, words, and lines in a file
* `detab`: expand tab characters to line up with tabstops

## Chapter 2: Filters

* `entab`: collapse space characters into tabs
* `overstrike`: some sort of weird typewriter filter… better figure this out
* `compress`: a quick and lousy compression utility
* `expand`: the opposite of `compress`
* `crypt`: symmetric xor-based encryption
* `tr`: perform character transliteration

## Chapter 3: Files

* `diff`: compare two files and print the differences
* `include`: print a file, expanding instructions to include other files
* `cat`: print *n* files
* `ar`: archive a group of files into a single compressed archive file

## Chapter 4: Sorting

* `sort`: print the lines of a file in sorted order
* `uniq`: strip adjacent duplicate lines in a stream
* `common`: print lines found in only one or both of a pair of files

## Chapter 5: Text Patterns

…

## Chapter 6: Editing

…

## Chapter 7: Formatting

…

## Chapter 8: Macro Processing

…

## Chapter 9: A Ratfor-Fortran Translator

…


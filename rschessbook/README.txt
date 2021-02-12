Create chess opening books in polyglot format

It has been written following the papers of Michel Van den Bergh.

Reading the original program I realized that internally it works with a 16x12 board, 
and later transplants it to an 8x8 board inside an array of length 192. 

Why not work directly with a 64 square board?

So... The internal structure of the chessboard comes from "secondchess" - 
https://github.com/emdio/secondchess - (64 square board), 
duly adapted by adding and creating the hash keys and the implementation of 
the creation of the book.

There is only one option on the command line (no merge, etc...) :

"""
SYNTAX
* rschessbook make-book [-pgn inputfile] [-bin outputfile] [-max-ply ply]
*
* if -bin parameter is omitted then the name book.bin will be created
* if -max-ply is omitted then 20 half-moves will be assigned
"""

A reader of the opening book, written in python, in the python_book_reader 
directory is also accompanied by this program. 

Here the comments are in Spanish (my native language)
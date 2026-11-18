from pest import Pair
from pest import Parser

def make_parser() -> Parser:
  # Read grammar rules from a file.
  with open("louder.pest", encoding="utf-8") as fd:  
    grammar = fd.read()

  return Parser.from_grammar(grammar)
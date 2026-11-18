from louder_python.louder_ast import make_parser

def test_parser():
  with open("tests/test_parser_example_1.l", 'r') as f:
    print(make_parser().parse("Conversation", f.read()).dumps())
type: user
username: user1
> relationship_type_1

type: relationship
flags: READ | EXECUTE
> user2

type: user
username: user2
> user1 % invalid connection type should be ignored and warned

tags: tag1, tag2
flags: PRIVATE | AUTO
recipients: user1, user2
my_data: soifoiajfgoireqguvh
text: some text

some more text

user1
even more text
> tag1
> tag2
> tag3 % invalid connection via missing tag should be ignored and warned

tags: tag1

tags: tag2


# definitions

## uniqueness
some properties are "unique to X". this is enforced and these properties should never have to be set manually.

## sets
`set(O): Vec`

lists of objects tolerant of duplicates and sorting. this means that duplicates can be included in the list, but the list will functionally be the same with the duplicates removed.

unless otherwise said, the default set for a value is the empty set.

## positions
`position(O): int`

positions are unique integer ids that refer to nodes in a conversation

## variants
`variant(O): LouderVariant`

variants are values that may be integers, floats, strings or booleans. variants are usually named.

## additional data
`additional_data(O, k: String): LouderVariant`

additional data refers to any property affixed to an object that is not in the object's definition. it consists of a mapping between string names and variant values

## flags
`flag(O): int`

flags are integers whose bits are used to further specify the behavior of an object

## type
`typename(O): String`

types are strings given to an object's "type" field. The default type is "node". If a type is not used here, the engine will still load objects of that type, but they will exhibit no behavior other than existing. this is different from the "data type" of a variable, which is the primitive type of some data in programming terms.

## unions
unions are data with multiple possible data types which are not variants.

## context
a context is a set of users, conversations, events, and a config.

the config is a bunch of options to tweak how the engine works. instead of being rigorously defined, other features will say they are "toggleable via config" or "adjustable via config".

## users
users are composed of a string name, flags, a second set of flags called "others", and a set of relationships. their name is unique within a context.

the flags of users are READ, WRITE, and EXECUTE. and default to READ | EXECUTE

the string name identifying a user is called a "username" `user(ctx: Context, name: String): User` is defined if a user is defined in ctx for the given name, as long as that name is not "". the default name is "".



## relationships
relationships are composed of flags (same flags as users), and a set of users.

## conversations
a conversarion is composed of a string name, and a set of hubs, and a list of connections.

A conversation name is unique within the context, `conversation(ctx: Context, name: String): Conversation` behaves like user()

## hubs
A hub is composed of a position, integer "priority", username, set of usernames "recipients", flags, string "text", a number of choices, and a set of strings "tags".

a position is unique within a conversation. `hub(C: Conversation, position: int): Hub` is defined if there exists a hub in C at the give position.

## connections
connections are composed of two positions "from" and "to" and an integer "port"

## choices
choice(C: Conversation, a: Hub, b: Hub) is defined iff a, b in nodes(C), and there exists a connection c in connections(C) such that from from(c) == position(a) and to(c) == position(b).

## access
access(u: User) = flags(u)

if users(ctx) defines a path u, r1, u1, r2, u2, .., rn, v such that r1 in relationships(u), uk in users(rk), and v in users(rn), access(ctx: Context, u: User, v: User) is the maximum or-sum of flags over this path. Otherwise, access(ctx, u, v) = others(v)

if user(ctx, username(n)) is defined, access(ctx: Context, u: User, n: Hub) = access(ctx, u, user(ctx, username(n))). Otherwise, access(ctx, u, n) = READ | EXECUTE

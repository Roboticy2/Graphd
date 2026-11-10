# schema
```

<key>:<value>
user
text
> <tag>
label: object 1 % this key doesn't do anything, it's just a label

label: object 2
text

```

newlines separate fields. 2+ newlines and whitespace separate objects

`<key>:<value>` defines values on an object. If the key is named below, it's considered a "keyword". Otherwise, it's a key in the object's additional data.

objects can have unlabeled lines. If there are 2 or more, the first is interpreted as `user:<line 1>`, and the rest are interpreted as `text:<line 2>\n<line 3>...`. If there is only one, it is interpreted as `text:<line>`. You can also name the keys explicitly as `user:<user>` and `text:<text>`

the "type" field tells the engine how to interpret each object. Built in types are: hub, user, relationship

for hubs, the `> <tag>` syntax tells the engine to establish a connection the current hub to all hubs matching `<tag>`. the `link` field is interpreted as a tag, also, setting the hub's link flag to true and establishing a link to the first hub matching the tag. Multiple matches will cause a warning.

for users, `> <tag>` syntax searches for a relationship. For relationships, `> <tag>` searches for users. Overlapping tags between object types will cause a warning.

fields with whitespace or special characters in them aside from `a-zA-Z0-9_` will cause a warning.

# definitions

## uniqueness
some properties are "unique to X". this is enforced and these properties should never have to be set manually.

## sets
`set(O): Vec`

lists of objects tolerant of duplicates and sorting. this means that duplicates can be included in the list, but the list will functionally be the same with the duplicates removed.

unless otherwise said, the default set for a value is the empty set.

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
a context is a set of users, a set of strings called "present", conversations, and a config.

the config is a bunch of options to tweak how the engine works, represented as additional data. instead of being rigorously defined, other features will say they are "toggleable via config" or "adjustable via config".

## users
users are composed of a string name, a second string "owner", flags, a set of strings "tags", a second set of flags called "others", and a set of relationships. their name is unique within a context. 

the flags of users are READ, WRITE, and EXECUTE. and default to READ | EXECUTE

the string name identifying a user is called a "username" user(ctx: Context, name: String): User is defined if a user is defined in ctx for the given name, as long as that name is not "". the default name is "". present(ctx) is a set of usernames.

"owner" refers to the name of the conversation which introduced this user. Users can be defined alongside hubs in the above schema. If a conversation is removed, and a user is owned by that conversation, the context can be configured such that the user will also be removed.

## relationships
relationships are composed of a set of strings "tags", flags (same flags as users), and a set of users.

## conversations
a conversation is composed of a string name, and a set of hubs, and a list of connections.

A conversation name is unique within the context, conversation(ctx: Context, name: String): Conversation behaves like user()

## hubs
A hub is composed of a position, integer "priority", username, set of usernames "recipients", an integer "link", flags, string "text", a number of choices, and a set of strings "tags".

a position is unique within a conversation. `hub(C: Conversation, position: int): Hub` is defined if there exists a hub in C at the give position.

hub flags are AUTO, INTERRUPT, LINK, and PRIVATE

## hubs: copies
"Floating" or "orphan" copies of a hub can be made. if a in hubs(C), copy(a) = a2, where position(a2) = position(a) (!!). But a2 does not "belong" to the conversation, and is not a member of hubs(C), so it's position does not need to be unique.

Since checking for choices and other logic on hubs depends on the position, a2 can be substituted for a in these cases and still return meaningful values.

## connections
connections are composed of two positions "from" and "to" and an integer "port"

## choices
choice(C: Conversation, a: Hub, b: Hub) is defined if a, b in hubs(C), and there exists a connection c in connections(C) such that from from(c) == position(a) and to(c) == position(b) and not flags(b) | AUTO and not flags(b) | LINK

## access
access(u: User) = flags(u)

## access: relationships
if users(ctx) defines a path u, r1, u1, r2, u2, .., rn, v such that r1 in relationships(u), uk in users(rk), and v in users(rn), access(ctx: Context, u: User, v: User) is the maximum or-sum of flags over this path. Otherwise, access(ctx, u, v) = others(v)

## access: hubs
if not flags(n) | PRIVATE, and user(ctx, username(n)) is defined, then access(ctx: Context, u: User, n: Hub) = access(ctx, u, user(ctx, username(n))). Otherwise, access(ctx, u, n) = READ | EXECUTE

## access: recipients
if flags(n) | PRIVATE, and user(ctx, username(n)) is defined, then access(ctx: Context, u: User, n: Hub) = the union of access(ctx, u, user(ctx, v)) for each v in recipients(n). Otherwise, access(ctx, u, n) = READ | EXECUTE

Note that if recipients(n) is empty, this is a zero union, or just 0. Meaning nobody can access this hub. This is intended, though it should be a warning.

## execution
can_execute(ctx, C, u, a, b) is true iff choice(C, a, b) is defined and access(ctx, u, b) | EXECUTE,

can_execute_if_singleplayer(ctx, C, u, a, b) is true iff choice(C, a, b) is defined, access(ctx, u, b) | READ, and there exists no v != u such that username(v) in present(ctx) and can_execute(ctx, C, v, a, b)

if can_execute(ctx, C, u, a, b) or can_execute_if_singleplayer(ctx, C, u, a, b), then execute_choice(ctx, C, u, a, b) is defined as a list of events.

can_execute_either(ctx, C, u, a, b) = can_execute(ctx, C, u, a, b) | can_execute_singleplayer(ctx, C, u, a, b)

## events
events things happening in a context. They are composed of a "rule", an integer indicating how the event happened, and a union of one of the available sub-event types:
 - conversation added event. composed of a conversation name
 - conversation removed event. composed of a conversation name
 - hub added event. composed of a conversation name and a position
 - hub removed event. composed of a conversation name and a position
 - user added event. composed of a username
 - user removed event. composed of a username
 - user presented event. composed of a username, represents a user being added to present(ctx)
 - user departed event. composed of a username, represents a user being removed from present(ctx)
 - config changed event. composed of additional data
 - execution event. composed of a user name, conversation name, two positions "from" and "to", and execution event flags.
 - interrupt event. composed of a user name, conversation name, and a position
 - warning event. composed of an object and a string "message"
 - error event. composed of an object and a string "message"

execution event flags are BACKGROUND, FORCE, and FOLLOW.

execution and interrupt events are emitted during "execution" in a conversation, which will be elaborated on elsewhere.

## execution: interrupts
if flags(b) | INTERRUPT, and can_execute_either(ctx, C, u, a, b), then execute_choice(ctx, C, u, a, b) = { e } and e = {RULE_INTERRUPT, interrupt_event, username(u), name(C), position(b)}

if execute_choice(ctx, C, u, a, b) is a set containing a single interrupt event, then verdict(e, v: bool) = execute_choice(ctx, C, u, a, b) - e

## execution: interrupts on choices
interrupts can also affect the availability of choices. 

if can_execute_either(ctx, C, u, a, b), for all hubs h such that choice(C, a, b) is defined, execute_choice(ctx, C, u, a, b) contains {RULE_INTERRUPT_CHOICE, interrupt_event, username(u), name(C), position(h)}.

if execute_choice(ctx, C, u, a, b) contains multiple interrupts events e, verdict(e, v) = execute_choice(ctx, C, u, a, b) - e.

## execution: choices
if not flags(b) | AUTO and not flags(b) | LINK, and either can_execute_either(ctx, C, u, a, b), then execute_choice(ctx, C, u, a, b) contains {r, execution_event, username(u), name(C), position(a), position(b), 0}

if there exists no v != u such that username(v) in present(ctx) and can_execute(ctx, C, v, a, b), and not can_execute(ctx, C, u, a, b), r = RULE_SINGLEPLAYER
if there exists v != u such that username(v) in present(ctx) and can_execute(ctx, C, v, a, b), and username(b) = username(u), r = RULE_STANDARD
if there exists v != u such that username(v) in present(ctx) and can_execute(ctx, C, v, a, b), and username(b) != username(u), r = RULE_PERMISSIONS
if, for some a in hubs(C), the union of the sets of hubs B subset hubs(C) for which can_execute(ctx, C, v, a, b) is true for all b in B, for each v such that username(v) in present(ctx), contains one element, r = RULE_SPOKE

These are the 3 rules which primarily support multiplayer. To start a conversation with multiple players, place two or more valid usernames into present(ctx). If a sequence of choices is only visible to user u because of permissions, RULE_SINGLEPLAYER plus the guard in the main condition will allow them to move

Note that with how interrupts are defined, these rules will still appear even after a verdict, since the value of the verdict depends on execute_choice() and the rule of execute choice depends on these conditions.

## execution: tracing
if flags(a) | f for some hub flag f, trace(ctx, C, u, a, f) is a list of hubs a, h1, .., hn such that a in hubs(C), flags(a) | f, hk in hubs(C), flags(hk) | f, access(ctx, u, hk) | EXECUTE

## auto traces
trace_auto(ctx, C, u, a) = trace(ctx, C, u, a, AUTO)

## execution: auto


## link traces

## execution: link

## execution: priority and following

## execution: priority and background events

## execution: warp

## conversations: loading

## context: editing

## conversations: saving

## conversations: searching

## meditation: types of searches
this proposed system contains: a maximal path search algorithm (user permissions), a depth first path search algorithm (auto and link), and a union of sets (RULE_SPOKE). Louder previously had suggested features for automatically detecting hubs of type "explore", "continue", or "neither" based the shape of the conversation, requiring full searches. Dialogue is a surprisingly graph search heavy task, when you start to get fancy with it. These searches are the most complex parts of the system. The way to keep them simple is to keep them "dumb". dfs is easy enough, but RULE_SPOKE and user permissions could cause some trouble. These are not magic thinking machines, they are automation tools. 

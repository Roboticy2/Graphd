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

for users, `> <tag>` syntax searches for a relationship. For relationships, `> <tag>` searches for users (by name, not tag). Overlapping tags between object types will cause a warning.

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
a context is a set of users, conversations, a config, a map of strings to sets of strings called "present", a map of events to booleans called "verdicts", and a map of usernames to integers called "positions".

the config is a bunch of options to tweak how the engine works, represented as additional data. instead of being rigorously defined, other features will say they are "toggleable via config" or "adjustable via config".

## users
users are composed of a string name, a second string "owner", flags, a set of strings "groups", a second set of flags called "others", and a set of relationships. their name is unique within a context. 

the flags of users are READ, WRITE, and EXECUTE. and default to READ | EXECUTE

the string name identifying a user is called a "username" user(ctx: Context, name: String): User is defined if a user is defined in ctx for the given name, as long as that name is not "". the default name is "". present(ctx: Context, C: Conversation) is a set of usernames.

"owner" refers to the name of the conversation which introduced this user. Users can be defined alongside hubs in the above schema. If a conversation is removed, and a user is owned by that conversation, the context can be configured such that the user will also be removed.

## relationships
relationships are composed of a set of strings "tags", flags (same flags as users), and a set of users.

## conversations
a conversation is composed of a string name, and a set of hubs, and a list of connections.

A conversation name is unique within the context, conversation(ctx: Context, name: String): Conversation behaves like user()

## hubs
A hub is composed of a position, integer "priority", username, set of usernames "recipients", an integer "link", flags, string "text", an integer "choices", and a set of strings "tags".

a position is unique within a conversation. `hub(C: Conversation, position: int): Hub` is defined if there exists a hub in C at the give position.

priority and choices default to 0, link to -1 or POSITION_NULL.

username defaults to "".

hub flags are AUTO, LINK, INTERRUPT, CHECK, and PRIVATE

## hubs: copies
"Floating" or "orphan" copies of a hub can be made. if a in hubs(C), copy(a) = a2, where position(a2) = position(a) (!!). But a2 does not "belong" to the conversation, and is not a member of hubs(C), so it's position does not need to be unique.

Since checking for choices and other logic on hubs depends on the position, a2 can be substituted for a in these cases and still return meaningful values.

## connections
connections are composed of two positions "from" and "to" and an integer "port". The ports of all connections in from a hub h in connections(C) are unique and contiguous from 0 to choices(h)-1

connection(C, from, port) uniquely identifies a connection, if it is defined.

## choices
choice(ctx: Context, C: Conversation, u: User, a: Hub, b: Hub) is defined if a, b in hubs(C), and there exists a connection c in connections(C) such that from from(c) == position(a) and to(c) == position(b) and not flags(b) | AUTO

the context and user will be used later when we add more definitions to choices. 

## access
access(u: User) = flags(u)

## access: relationships
if users(ctx) defines a path u, r1, u1, r2, u2, .., rn, v such that r1 in relationships(u), uk in users(rk), and v in users(rn), access(ctx: Context, u: User, v: User) is the maximum or-sum of flags over this path. Otherwise, access(ctx, u, v) = others(v)

## access: hubs
if not flags(n) | PRIVATE, and user(ctx, username(n)) is defined, then access(ctx: Context, u: User, n: Hub) = access(ctx, u, user(ctx, username(n))). Otherwise, access(ctx, u, n) = READ | EXECUTE

## access: recipients
if flags(n) | PRIVATE, and user(ctx, username(n)) is defined, then access(ctx: Context, u: User, n: Hub) = the union of access(ctx, u, user(ctx, v)) for each v in recipients(n). Otherwise, access(ctx, u, n) = READ | EXECUTE

Note that if recipients(n) is empty, this is a zero union, or just 0. Meaning nobody can access this hub. This is intended, though it should be a warning.

## execution: verdicts
when verdict(ctx: Context, e: interrupt event, v: bool) is called, the "verdicts" cache associates the event with the verdict. verdicts are typically defined by an external program, by calling the "verdict" function on a context, but if verdicts are in the cache, they can be recovered automatically.

Note: interrupt events caused by RULE_CHECK are removed from the cache after each call to execute_choice, defined later, while events caused by RULE_INTERRUPT are preserved. See proposed features for more comments about why this is annoying.

get_verdict(ctx: Context, e: interrupt event): bool is defined when e is in the verdict cache.

verdicts(ctx: Context): set returns all verdicts in the verdict cache.

## execution
can_follow(ctx, u, b) is true iff access(ctx, u, b) | EXECUTE and there exists no {e, v} in verdicts(ctx) such that rule(e) = RULE_CHECK and v is false.

can_execute(ctx, C, u, a, b) is true iff choice(ctx, C, u, a, b) is defined and can_follow(ctx, u, b) is true

can_execute_if_singleplayer(ctx, C, u, a, b) is true iff choice(ctx, C, u, a, b) is defined, access(ctx, u, b) | READ, and there exists no v != u such that username(v) in present(ctx, C) and can_execute(ctx, C, v, a, b)

if can_execute(ctx, C, u, a, b) or can_execute_if_singleplayer(ctx, C, u, a, b), then execute_choice(ctx, C, u, a, b) is defined as a list of events.

can_execute_either(ctx, C, u, a, b) = can_execute(ctx, C, u, a, b) | can_execute_singleplayer(ctx, C, u, a, b)

## events
events things happening in a context. They are composed of a "rule", an integer indicating how the event happened, and a union of one of the available sub-event types:
 - conversation added event. composed of a conversation name
 - conversation removed event. composed of a conversation name
 - hub added event. composed of a conversation name and a position
 - hub removed event. composed of a conversation name, a position, and a set of connections
 - connection added event. composed of a conversation name, and a connection object
 - connection removed event. composed of a conversation name, one position "from", and a port.
 - user added event. composed of a username
 - user removed event. composed of a username
 - user presented event. composed of a conversation name and username, represents a user being added to present(ctx, C)
 - user departed event. composed of a conversation name and username, represents a user being removed from present(ctx, C)
 - config changed event. composed of additional data
 - execution event. composed of a user name, conversation name, a position "from" and a hub "to", and execution event flags.
 - interrupt event. composed of a user name, conversation name, and a position
 - warning event. composed of an object and a string "message"
 - error event. composed of an object and a string "message"

execution event flags are BACKGROUND, FORCE, and FOLLOW.

execution and interrupt events are emitted during "execution" in a conversation, which will be elaborated on elsewhere.

## execution: checks
interrupts can also affect the availability of choices. 

if not flags(b) | LINK and can_execute_either(ctx, C, u, a, b), for all hubs h such that flags(h) | CHECK and choice(ctx, C, u, a, b) is defined, execute_choice(ctx, C, u, a, h) contains {RULE_CHECK, interrupt_event, username(u), name(C), h, 0}.

if execute_choice(ctx, C, u, a, b) contains multiple interrupts events e, verdict(ctx, e, v) contains execute_choice(ctx, C, u, a, b) - e.

Since the value of the verdict is cached, 

## execution: interrupts
if flags(b) | INTERRUPT | LINK, and can_execute_either(ctx, C, u, a, b) and e not in , then execute_choice(ctx, C, u, a, b) contains e = {RULE_INTERRUPT, interrupt_event, username(u), name(C), b, 0}

if execute_choice(ctx, C, u, a, b) is a set containing a single interrupt event, then verdict(ctx, e, v) contains execute_choice(ctx, C, u, a, b) - e

## execution: choices
if not flags(b) | AUTO | LINK, and either can_execute_either(ctx, C, u, a, b), then execute_choice(ctx, C, u, a, b) contains {r, execution_event, username(u), name(C), position(a), b, 0}

if there exists no v != u such that username(v) in present(ctx, C) and can_execute(ctx, C, v, a, b), and not can_execute(ctx, C, u, a, b), r = RULE_SINGLEPLAYER
if there exists v != u such that username(v) in present(ctx, C) and can_execute(ctx, C, v, a, b), and username(b) = username(u), r = RULE_STANDARD
if there exists v != u such that username(v) in present(ctx, C) and can_execute(ctx, C, v, a, b), and username(b) != username(u), r = RULE_PERMISSIONS
if, for some a in hubs(C), the union of the sets of hubs B subset hubs(C) for which can_execute(ctx, C, v, a, b) is true for all b in B, for each v such that username(v) in present(ctx, C), contains one element, r = RULE_SPOKE

These are the 3 rules which primarily support multiplayer. To start a conversation with multiple players, place two or more valid usernames into present(ctx, C). If a sequence of choices is only visible to user u because of permissions, RULE_SINGLEPLAYER plus the guard in the main condition will allow them to move

Note that with how interrupts are defined, these rules will still appear even after a verdict, since the value of the verdict depends on execute_choice() and the rule of execute choice depends on these conditions.

## tracing
if flags(h1) | f for some hub flag f, trace(ctx, C, u, h1, f) is a set of hubs h1, .., hn, check1, .., checkm, such that:
 - hk in hubs(C),
 - flags(hk) | f for h1..hn,
 - not flags(hk) | INTERRUPT or there exists e = {RULE_INTERRUPT, interrupt_event, username(u), name(C), position(hk)} in verdicts(ctx)
 - not flags(hk) | CHECK or there exists e = {RULE_CHECK, interrupt_event, username(u), name(C), position(hk)} in verdicts(ctx)
 - flags(checkk) | CHECK for check1..check(m-1),
 - if not flags(checkk) | CHECK | INTERRUPT, not flags(checkk) | f for check1..checkm,
 - access(ctx, C, u, x) for x in h1..hn, check1..checkm
 - connections c1..c(n-1) exist in connections(C) such that ck is from hk to h(k+1) for hk..h(n-1) 
 - connections d1..dm exist in connections(C) such that dk is from hn to checkk
(depth first search on lowest available connection, then spread out, halting at interrupts or when f runs out)

If no such list exists, then there must be a cycle in C of hubs with flag f containing h1. In this case, trace(ctx, C, u, h1, f) is equal to some such cycle.

trace_end(ctx, C, u, h1, f) = check1..checkm if the list form exists. the trace is a cycle, trace_end(ctx, C, u, h1, f) = j, a copy of h such that not flag(j) | f.

Intuition: the goal is to reduce dialogue to a linear sequence within the domain of the auto hubs. Checks and interrupts have to handed off to the user. Once those caches are filled, the definition of the trace changes to match.

## auto traces
trace_auto(ctx, C, u, a) = trace(ctx, C, u, a, AUTO) and trace_auto_end(ctx, C, u, a) = trace_end(ctx, C, u, a, AUTO)

## choices: auto
choice(ctx, C, u, a, b) is defined if there exists a hub h such that a connection c exists in connections(C) from a to h, flags(h) | AUTO, and b is in trace_auto_end(ctx, C, u, h)

## execution: auto interrupts
if choice_auto(ctx, C, u, a, b) is defined, and flags(b) | INTERRUPT | AUTO, then if e = {RULE_INTERRUPT, interrupt_event, username(u), name(C), position(h)}, if verdict(ctx, e, v) and v, verdict(ctx, e, v) = execute_choice(ctx, C, u, b, h), where h is the first check1 in trace_auto_end(ctx, C, u, b).

## link traces
if flags(h1) | LINK, trace_link(hn) = h1..hn such that:
 - link(hk) = position(h(k+1)) for h1..h(n-1)

## link resolution
for two hubs a, b, a * b = a copy of a with all non-default fields and additional data of b overriding a's data, with the exception of position. Flags, tags, and recipients are unioned. Choices are replaced. Finally, the LINK flag is disabled.

Note: disabling the link flag is critical to make the "execution: link" section compatible with other sections, which rely on the LINK flag not being present in the first place. disabling the link flag signals that the link has been resolved.

## execution: link
if flags(b) | LINK and can_execute_either(ctx, C, u, a, \bigprod trace_link(ctx, C, u, b)) is defined, then execute_choice(ctx, C, u, a, b) contains {r, execution_event, username(u), name(C), position(a), \bigprod trace_link(ctx, C, u, b), 0}. rules behave the same as in "execution: choices."

## execution: simplified priority
See proposed features for a stronger version of priority.

if execute_choice(ctx, C, u, a, b) contains {r, execution_event, un, Cn, apos, x, f}, and can_follow(ctx, v, x) for some v != u in users(ctx), then execute_choice(ctx, C, u, a, b) contains {RULE_FOLLOW, execution_event, username(v), Cn, -1, x, FOLLOW}

## execution: force motion
force_motion(ctx, C, u, a, b) can be invoked to force a motion. force_motion(ctx, C, u, a, b) contains execute_choice(ctx, C, u, a, b)

## conversations: loading
load_conversation(ctx: Context, path: String) = {RULE_CONVERSATION_ADDED, conversation_added, path} adds a conversation name loaded from file path with name path. Warnings are emitted as described in the schema section.

conversation(ctx: Context, path: String): Conversation is defined if the conversation at path is loaded. Returns the conversation object.

remove_conversation(ctx: Context, path: String) = {RULE_CONVERSATION_REMOVED, conversation_removed, path} if conversation(ctx, path), {RULE_ERROR_UNDEFINED_CONVERSATION, path, "conversation at {path} not found"}. removes a conversation by name. 

## conversations: editing
add_hub(C: Conversation, u: Hub, change_position: bool) = {{RULE_HUB_ADDED, hub_added, name(C), position(x)}}

if change_position is true, u's position will be changed automatically to be unique, creating a new hub, x. Returns event if u's position would need to be changed to add it to the conversation. If change_position is false and the function returns {{RULE_ERROR_DOUBLE_POSITION, error_message, u, "the position {position(u)} already exists in the conversation {name(C)}"}}

remove_hub(C: Conversation, p: int) contains {RULE_HUB_REMOVED, hub_removed, name(C), p, removed_connections} where remove_connections is the subset of connections(C) of connections to or from the hub at position p. If remove_connections is not empty, connection_removed events are also included.

removes the hub at position p if it was defined, returning {{RULE_ERROR_UNDEFINED_POSITION, error_message, p, "no hub at position {p} in conversation {name(C)}"}}

add_connection(C: Conversation, c: Connection, replace: bool) = {{RULE_CONNECTION_ADDED, connection_added, name(C), c, replaced}}

adds a connection c. If replace is true, will replace an existing connection at the same from/port pair if it exists, and return a copy of the replaced connection. If replace is false and such a connection already exists, returns {{RULE_ERROR_DOUBLE_CONNECTION, error_message, c, "connection already exists at position {from(c)} and port {port(c)} in conversation {name(C)}"}}

remove_connection(C: Conversation, position: int, port: int) = {{RULE_CONNECTION_REMOVED, remove_connection, name(C), position, port}}

removes a connection at position/port pair. If no such connection exists, returns {{RULE_ERROR_UNDEFINED_POSITION_PORT_PAIR, error_message, p, "no connection at position {position} and port {port} in conversation {name(C)}"}}

## context: editing
present_user(ctx: Context, cname: String, name: String) presents a user if they are defined in users(ctx), and all users with EXECUTE access from this user, failing otherwise.

depart_user(ctx: Context, cname: String, name: String) departs a user if they are both defined in users(ctx) and their name is in present(ctx, C), if another user no longer has EXECUTE access from any user, they are also departed. Missing either comes with a unique error event.

## conversations: searching
search_hubs(C: Conversation, tags: set): set returns all hubs in a conversation matching all given tags.

search_users(ctx: Context, groups: set): set returns all users in a context matching all given groups.

search_relationships(ctx: Context, tags: set): set returns all relationships in a context matching all given tags.

## meditation: types of searches
this proposed system contains: a maximal path search algorithm (user permissions), a depth first path search algorithm (auto and link), and a union of sets (RULE_SPOKE). Louder previously had suggested features for automatically detecting hubs of type "explore", "continue", or "neither" based the shape of the conversation, requiring full searches. Dialogue is a surprisingly graph search heavy task, when you start to get fancy with it. These searches are the most complex parts of the system. The way to keep them simple is to keep them "dumb". dfs is easy enough, but RULE_SPOKE and user permissions could cause some trouble. These are not magic thinking machines, they are automation tools. 

This is already taking triple the time budget I set aside to design, too. It's too complex for me. As far as design goes, the hardest thing to nail down is the user interaction via verdicts and execution.

# proposed features + alternatives

## proposed feature: configuration of verdict cache
The context can be configured to not store verdicts caused by RULE_INTERRUPT, to clear the verdict memory when a verdict caused by RULE_INTERRUPT arrives, to clear the verdict memory of events on hubs with tags matching a given set of tags, or to clear the verdict memory for events on hubs with tags matching a given set of tags intersected with the tags of the hub of the incoming verdict, or none of the above (default)

This is too complex for now. But I'm going to implement as if "none of the above" is selected here since it matches the old model the best. There's also a subtle behavior here that I'm not sure how to classify. I want to not cache verdicts on RULE_CHECK events because those are conditions. The standard flow is "check something, then something changes, then check again". the check versus interrupt flags provide the distinction between checking and doing something, and the planned caching behavior provides a sensible default for "measure twice, cut once."

## proposed feature: history and serialization
Saving edited conversations is out of scope for the first sprint. Serialization comes with other considerations. The "execution" of the engine so far is mostly stateless. If saving a loaded conversation is on the table, what about a trace of user positions? event logs?

## proposed feature: proper event queue
The current program is simple enough to work without a full internal event queue. But event queues are ubiquitous for a reason. Maybe it would be pertinent to include one in a future sprint.

This one I might be able to pull forward to the current sprint. As I add multiple functions for events I see how unwieldy it could get to not have a centralized event manager. I can keep the "returns event or empty" syntax for this rules file, but just implement it differently.

## proposed feature: stronger priority
There's a lot of unused data in the rules above having to do with priority. In general, I think this is an essential multiplayer feature. However, I don't think it's needed for the demo. The idea is that which users follow whom depends on what priority they currently hold, derived from their current hub. This requires storing user positions, which I'm not sure how it is best to approach.

## proposed feature: live user and relationship editing
requires stronger definition of how users, relationships, and hubs interact in a conversation.

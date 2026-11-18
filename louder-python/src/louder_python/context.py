import pandas

from louder_python.conversation import Conversation

class Context:
  # users in this context
  users: pandas.DataFrame

  # tags identifying users
  user_in: pandas.DataFrame

  # tags users use to relate to relationships
  user_relationships_out: pandas.DataFrame


  relationships: pandas.DataFrame
  # tags identifying relationships
  relationships_user_in: pandas.DataFrame

  # Conversation upgraded into an iterator of conversations by adding a column
  # to conversation tables of conversation id.
  conversation: Conversation
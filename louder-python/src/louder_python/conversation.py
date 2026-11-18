import pandas

class Conversation:
  hubs: pandas.DataFrame

  #tags identifying hubs
  hub_in: pandas.DataFrame

  #tags that hubs use to link to other hubs
  hub_out: pandas.DataFrame

  #recipients of private hubs. hubs in this table are private
  hub_recipients: pandas.DataFrame
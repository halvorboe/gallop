from gallop.wrench import chaos
from gallop.api import APIServer

IMAGE = """
-------------------------------------------------

        ,--,
  _ ___/ /\|
 ;( )__, )
; //   '--;
  \     |
   ^    ^

-------------------------------------------------
GALLOP - An analytics database buildt for speed.
-------------------------------------------------
"""


def run():
    chaos()

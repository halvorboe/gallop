import random

from datetime import datetime, timedelta


URLS = [
    "https://example.net/",
    "https://example.com/",
    "http://www.example.net/",
    "http://www.example.org/birthday",
    "http://bells.example.com/bone",
    "https://example.com/bag/baseball?army=behavior&books=advertisement",
    "http://www.example.net/",
    "https://example.com/bridge",
    "https://www.example.com/angle/bedroom",
    "http://acoustics.example.com/",
    "http://basket.example.com/bubble/arithmetic.html",
    "http://www.example.com/belief",
    "https://bath.example.edu/branch?blade=bead&bedroom=back",
    "https://www.example.org/addition",
    "http://boot.example.com/#bells",
    "https://example.com/?breath=approval",
    "https://example.com/",
    "https://believe.example.com/#ants",
    "http://www.example.com/?battle=amusement",
    "https://example.edu/account.html",
    "http://example.com/?bubble=brass",
    "https://www.example.com/believe/bag",
    "https://example.com/",
    "http://example.com/",
    "http://www.example.com/?birth=believe&authority=bedroom",
    "http://ants.example.org/",
    "http://www.example.com/animal.php",
    "http://example.com/?bead=blood#ants",
]
BROWSERS = ["Safari", "Firefox", "Chrome"]
COUNTRY = ["NOR", "USA"]


class Table:
    """

    """

    pass


class Row:
    """
    Entry in a table. 
    Key is a timestamp.
    """

    timestamp: int

    @staticmethod
    def random():
        return Row(
            timestamp=(
                datetime.now() - timedelta(seconds=random.randint(0, 60 * 60 *24 * 365 * 5))
            ).timestamp(),
            data={
                "browser": random.choice(BROWSERS),
                "country": random.choice(COUNTRY),
            },
        )

    def time(self):
        return datetime.fromtimestamp(self.timestamp)

    def __init__(self, timestamp, data):
        self.timestamp = timestamp
        self.data = data

    def __repr__(self):
        return f"Row(timestamp={self.timestamp}, data={self.data})"


class Segment:
    """
    Atmoic unit for distrobitung rows across nodes. 
    """

    def __init__(self):
        self.rows = []

    def add(self, row: Row):
        self.rows.append(row)

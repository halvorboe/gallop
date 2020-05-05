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


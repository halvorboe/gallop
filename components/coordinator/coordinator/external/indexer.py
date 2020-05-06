import abc
from typing import List

from coordinator.data import Row, Segment


class InMemoryIndexer:
    def __init__(self):
        self.segments = []

    def index(self, segment: Segment):
        self.segments.append(segment)

    def query(self, query: str) -> List[Row]:
        res = []
        for segment in self.segments:
            for row in segment.rows:
                if query in list(row.data.values())[0]:
                    res.append(row)
        return res

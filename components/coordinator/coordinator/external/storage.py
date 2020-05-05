from typing import List

from coordinator.data import Segment, Row


class FileStorage:

    def __init__(self):
        self.segments = [Segment()]
    
    def insert(self, row: Row):
        self.segments[0].add(row)

    def select(self):
        pass

    def select_segments(self):
        return self.segments


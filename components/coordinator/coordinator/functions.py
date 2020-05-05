from coordinator.data import Row

from coordinator.external import FileStorage, InMemoryIndexer

class Coordinator:

    """
    Add inn config.
    Storage should split into segments.
    """

    def __init__(self):
        self.storage = FileStorage()
        self.indexers = [InMemoryIndexer()]

    def insert(self, row: Row):
        self.storage.insert(row)
   
    def select(self):
       self.storage.select()
    
    def query(self, query_string: str):
        return self.indexers[0].query(query_string)
    
    def sync(self):
        segments = self.storage.select_segments()
        for segment in segments:
            self.indexers[0].index(segment)
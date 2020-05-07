from coordinator.data import Row

from coordinator.external import FilePacker, InMemoryIndexer


class Coordinator:

    """
    Add inn config.
    Packer should split into segments.
    """

    def __init__(self):
        self.packer = FilePacker()
        self.indexers = [InMemoryIndexer()]

    def insert(self, row: Row):
        self.packer.insert(row)

    def select(self):
        self.packer.select()

    def query(self, query_string: str):
        return self.indexers[0].query(query_string)

    def sync(self):
        segments = self.packer.select_segments()
        for segment in segments:
            self.indexers[0].index(segment)

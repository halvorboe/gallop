
import grpc
from tqdm import tqdm
import json

import coordinator.protos.packer_pb2 as packer
import coordinator.protos.packer_pb2_grpc as packer_grpc

import coordinator.protos.common_pb2 as common

channel = grpc.insecure_channel('localhost:8081')
stub = packer_grpc.PackerStub(channel)

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


def random_timestamp():
    return int((datetime.now() - timedelta(seconds=random.randint(0, 60 * 60 *24 * 365 * 5))).timestamp() * 1_000_000_000)

def random_data():
    return json.dumps({"url": random.choice(URLS), "browser": random.choice(BROWSERS), "country": random.choice(COUNTRY)})

for n in tqdm(range(1_000_000)):
    table = "hello-world"
    row = common.Row()
    row.timestamp = random_timestamp()
    row.data = random_data()
    req = packer.InsertRequest(table=table, row=row)
    stub.Insert(req)

print("Fetching segment...")

segments_response = stub.Segments(packer.SegmentsRequest())
print(type(segments_response))
for segment in segments_response.segments:
    print("table: ", segment.table)
    print("resolution: ", segment.resolution)
    print("timestamp: ", segment.timestamp)
    print("id: ", segment.partition_id)

print("Done...")

# result = stub.Segment(packer.SegmentRequest())
# print(result)

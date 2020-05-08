
import json
import random
from datetime import datetime, timedelta


import grpc
from tqdm import tqdm

import gallop.protos.packer_pb2 as packer
import gallop.protos.packer_pb2_grpc as packer_grpc

import gallop.protos.common_pb2 as common

from gallop import constants

def chaos():

    channel = grpc.insecure_channel('localhost:8081')
    stub = packer_grpc.PackerStub(channel)

    for _ in tqdm(range(1_000)):
        table = "hello-world"
        row = common.Row()
        row.timestamp = random_timestamp()
        row.data = random_data()
        req = packer.InsertRequest(table=table, row=row)
        stub.Insert(req)

    
    segments_response = stub.Segments(packer.SegmentsRequest())
    print(type(segments_response))
    for segment_id in segments_response.segments:
        print("table: ", segment_id.table)
        print("resolution: ", segment_id.resolution)
        print("timestamp: ", segment_id.timestamp)
        print("id: ", segment_id.partition_id)
        segment_resp = stub.Segment(packer.SegmentRequest(segment_id=segment_id))
        print("resp:", segment_resp.segment.rows)
    







def random_timestamp():
    return int((datetime.now() - timedelta(seconds=random.randint(0, 60 * 60 *24 * 365 * 5))).timestamp() * 1_000_000_000)

def random_data():
    return json.dumps({"url": random.choice(constants.URLS), "browser": random.choice(constants.BROWSERS), "country": random.choice(constants.COUNTRY)})



print("Fetching segment...")

print("Done...")

# result = stub.Segment(packer.SegmentRequest())
# print(result)

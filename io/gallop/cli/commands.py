import json
import random
from datetime import datetime, timedelta


import grpc
from tqdm import tqdm

import gallop.protos.packer_pb2 as packer
import gallop.protos.packer_pb2_grpc as packer_grpc

import gallop.protos.indexer_pb2 as indexer
import gallop.protos.indexer_pb2_grpc as indexer_grpc

import gallop.protos.common_pb2 as common

from gallop import constants


def chaos():
    packer_stub = packer_grpc.PackerStub(grpc.insecure_channel("localhost:8081"))

    indexer_stub = indexer_grpc.IndexerStub(grpc.insecure_channel("localhost:8082"))

    for _ in tqdm(range(100_000)):
        table = "hello-world"
        row = common.Row()
        row.timestamp = random_timestamp()
        row.data = random_data()
        req = packer.InsertRequest(table=table, row=row)
        packer_stub.Insert(req)

    segments_response = packer_stub.Segments(packer.SegmentsRequest())
    for segment_id in tqdm(segments_response.segments):
        segment_resp = packer_stub.Segment(packer.SegmentRequest(segment_id=segment_id))
        req = indexer.BindRequest(segment_id=segment_id)
        indexer_stub.Bind(req)


def random_timestamp():
    return int(
        (
            datetime.now()
            - timedelta(seconds=random.randint(0, 60 * 60 * 24 * 365 * 5))
        ).timestamp()
        * 10_000_000
    )


def random_data():
    return json.dumps(
        {
            "url": random.choice(constants.URLS),
            "browser": random.choice(constants.BROWSERS),
            "country": random.choice(constants.COUNTRY),
        }
    )


def run():
    chaos()

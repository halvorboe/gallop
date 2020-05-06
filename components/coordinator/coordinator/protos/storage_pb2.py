# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: storage.proto

from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


import common_pb2 as common__pb2


DESCRIPTOR = _descriptor.FileDescriptor(
  name='storage.proto',
  package='protos',
  syntax='proto3',
  serialized_options=None,
  serialized_pb=b'\n\rstorage.proto\x12\x06protos\x1a\x0c\x63ommon.proto\"\x8b\x01\n\x0bSegmentMeta\x12\n\n\x02id\x18\x01 \x01(\t\x12\x11\n\ttimestamp\x18\x02 \x01(\x04\x12\x32\n\nresolution\x18\x03 \x01(\x0e\x32\x1e.protos.SegmentMeta.Resolution\")\n\nResolution\x12\x08\n\x04HOUR\x10\x00\x12\x07\n\x03\x44\x41Y\x10\x01\x12\x08\n\x04WEEK\x10\x02\"F\n\x07Segment\x12!\n\x04meta\x18\x01 \x01(\x0b\x32\x13.protos.SegmentMeta\x12\x18\n\x03Row\x18\x02 \x03(\x0b\x32\x0b.protos.Row\"\x10\n\x0eSegmentRequest\"4\n\x0fSegmentResponse\x12!\n\x08segments\x18\x01 \x01(\x0b\x32\x0f.protos.Segment\"\x11\n\x0fSegmentsRequest\"9\n\x10SegmentsResponse\x12%\n\x08segments\x18\x02 \x03(\x0b\x32\x13.protos.SegmentMeta\"1\n\x10\x43onfigureRequest\x12\x1d\n\x06tables\x18\x01 \x03(\x0b\x32\r.protos.Table\"=\n\rInsertRequest\x12\x12\n\ntable_name\x18\x01 \x01(\t\x12\x18\n\x03row\x18\x02 \x01(\x0b\x32\x0b.protos.Row2\xf2\x01\n\x07Storage\x12\x36\n\tConfigure\x12\x18.protos.ConfigureRequest\x1a\r.protos.Error\"\x00\x12\x30\n\x06Insert\x12\x15.protos.InsertRequest\x1a\r.protos.Error\"\x00\x12<\n\x07Segment\x12\x16.protos.SegmentRequest\x1a\x17.protos.SegmentResponse\"\x00\x12?\n\x08Segments\x12\x17.protos.SegmentsRequest\x1a\x18.protos.SegmentsResponse\"\x00\x62\x06proto3'
  ,
  dependencies=[common__pb2.DESCRIPTOR,])



_SEGMENTMETA_RESOLUTION = _descriptor.EnumDescriptor(
  name='Resolution',
  full_name='protos.SegmentMeta.Resolution',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='HOUR', index=0, number=0,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='DAY', index=1, number=1,
      serialized_options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='WEEK', index=2, number=2,
      serialized_options=None,
      type=None),
  ],
  containing_type=None,
  serialized_options=None,
  serialized_start=138,
  serialized_end=179,
)
_sym_db.RegisterEnumDescriptor(_SEGMENTMETA_RESOLUTION)


_SEGMENTMETA = _descriptor.Descriptor(
  name='SegmentMeta',
  full_name='protos.SegmentMeta',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='protos.SegmentMeta.id', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='timestamp', full_name='protos.SegmentMeta.timestamp', index=1,
      number=2, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='resolution', full_name='protos.SegmentMeta.resolution', index=2,
      number=3, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
    _SEGMENTMETA_RESOLUTION,
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=40,
  serialized_end=179,
)


_SEGMENT = _descriptor.Descriptor(
  name='Segment',
  full_name='protos.Segment',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='meta', full_name='protos.Segment.meta', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='Row', full_name='protos.Segment.Row', index=1,
      number=2, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=181,
  serialized_end=251,
)


_SEGMENTREQUEST = _descriptor.Descriptor(
  name='SegmentRequest',
  full_name='protos.SegmentRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=253,
  serialized_end=269,
)


_SEGMENTRESPONSE = _descriptor.Descriptor(
  name='SegmentResponse',
  full_name='protos.SegmentResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='segments', full_name='protos.SegmentResponse.segments', index=0,
      number=1, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=271,
  serialized_end=323,
)


_SEGMENTSREQUEST = _descriptor.Descriptor(
  name='SegmentsRequest',
  full_name='protos.SegmentsRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=325,
  serialized_end=342,
)


_SEGMENTSRESPONSE = _descriptor.Descriptor(
  name='SegmentsResponse',
  full_name='protos.SegmentsResponse',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='segments', full_name='protos.SegmentsResponse.segments', index=0,
      number=2, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=344,
  serialized_end=401,
)


_CONFIGUREREQUEST = _descriptor.Descriptor(
  name='ConfigureRequest',
  full_name='protos.ConfigureRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='tables', full_name='protos.ConfigureRequest.tables', index=0,
      number=1, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=403,
  serialized_end=452,
)


_INSERTREQUEST = _descriptor.Descriptor(
  name='InsertRequest',
  full_name='protos.InsertRequest',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='table_name', full_name='protos.InsertRequest.table_name', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=b"".decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
    _descriptor.FieldDescriptor(
      name='row', full_name='protos.InsertRequest.row', index=1,
      number=2, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      serialized_options=None, file=DESCRIPTOR),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  serialized_options=None,
  is_extendable=False,
  syntax='proto3',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=454,
  serialized_end=515,
)

_SEGMENTMETA.fields_by_name['resolution'].enum_type = _SEGMENTMETA_RESOLUTION
_SEGMENTMETA_RESOLUTION.containing_type = _SEGMENTMETA
_SEGMENT.fields_by_name['meta'].message_type = _SEGMENTMETA
_SEGMENT.fields_by_name['Row'].message_type = common__pb2._ROW
_SEGMENTRESPONSE.fields_by_name['segments'].message_type = _SEGMENT
_SEGMENTSRESPONSE.fields_by_name['segments'].message_type = _SEGMENTMETA
_CONFIGUREREQUEST.fields_by_name['tables'].message_type = common__pb2._TABLE
_INSERTREQUEST.fields_by_name['row'].message_type = common__pb2._ROW
DESCRIPTOR.message_types_by_name['SegmentMeta'] = _SEGMENTMETA
DESCRIPTOR.message_types_by_name['Segment'] = _SEGMENT
DESCRIPTOR.message_types_by_name['SegmentRequest'] = _SEGMENTREQUEST
DESCRIPTOR.message_types_by_name['SegmentResponse'] = _SEGMENTRESPONSE
DESCRIPTOR.message_types_by_name['SegmentsRequest'] = _SEGMENTSREQUEST
DESCRIPTOR.message_types_by_name['SegmentsResponse'] = _SEGMENTSRESPONSE
DESCRIPTOR.message_types_by_name['ConfigureRequest'] = _CONFIGUREREQUEST
DESCRIPTOR.message_types_by_name['InsertRequest'] = _INSERTREQUEST
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

SegmentMeta = _reflection.GeneratedProtocolMessageType('SegmentMeta', (_message.Message,), {
  'DESCRIPTOR' : _SEGMENTMETA,
  '__module__' : 'storage_pb2'
  # @@protoc_insertion_point(class_scope:protos.SegmentMeta)
  })
_sym_db.RegisterMessage(SegmentMeta)

Segment = _reflection.GeneratedProtocolMessageType('Segment', (_message.Message,), {
  'DESCRIPTOR' : _SEGMENT,
  '__module__' : 'storage_pb2'
  # @@protoc_insertion_point(class_scope:protos.Segment)
  })
_sym_db.RegisterMessage(Segment)

SegmentRequest = _reflection.GeneratedProtocolMessageType('SegmentRequest', (_message.Message,), {
  'DESCRIPTOR' : _SEGMENTREQUEST,
  '__module__' : 'storage_pb2'
  # @@protoc_insertion_point(class_scope:protos.SegmentRequest)
  })
_sym_db.RegisterMessage(SegmentRequest)

SegmentResponse = _reflection.GeneratedProtocolMessageType('SegmentResponse', (_message.Message,), {
  'DESCRIPTOR' : _SEGMENTRESPONSE,
  '__module__' : 'storage_pb2'
  # @@protoc_insertion_point(class_scope:protos.SegmentResponse)
  })
_sym_db.RegisterMessage(SegmentResponse)

SegmentsRequest = _reflection.GeneratedProtocolMessageType('SegmentsRequest', (_message.Message,), {
  'DESCRIPTOR' : _SEGMENTSREQUEST,
  '__module__' : 'storage_pb2'
  # @@protoc_insertion_point(class_scope:protos.SegmentsRequest)
  })
_sym_db.RegisterMessage(SegmentsRequest)

SegmentsResponse = _reflection.GeneratedProtocolMessageType('SegmentsResponse', (_message.Message,), {
  'DESCRIPTOR' : _SEGMENTSRESPONSE,
  '__module__' : 'storage_pb2'
  # @@protoc_insertion_point(class_scope:protos.SegmentsResponse)
  })
_sym_db.RegisterMessage(SegmentsResponse)

ConfigureRequest = _reflection.GeneratedProtocolMessageType('ConfigureRequest', (_message.Message,), {
  'DESCRIPTOR' : _CONFIGUREREQUEST,
  '__module__' : 'storage_pb2'
  # @@protoc_insertion_point(class_scope:protos.ConfigureRequest)
  })
_sym_db.RegisterMessage(ConfigureRequest)

InsertRequest = _reflection.GeneratedProtocolMessageType('InsertRequest', (_message.Message,), {
  'DESCRIPTOR' : _INSERTREQUEST,
  '__module__' : 'storage_pb2'
  # @@protoc_insertion_point(class_scope:protos.InsertRequest)
  })
_sym_db.RegisterMessage(InsertRequest)



_STORAGE = _descriptor.ServiceDescriptor(
  name='Storage',
  full_name='protos.Storage',
  file=DESCRIPTOR,
  index=0,
  serialized_options=None,
  serialized_start=518,
  serialized_end=760,
  methods=[
  _descriptor.MethodDescriptor(
    name='Configure',
    full_name='protos.Storage.Configure',
    index=0,
    containing_service=None,
    input_type=_CONFIGUREREQUEST,
    output_type=common__pb2._ERROR,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='Insert',
    full_name='protos.Storage.Insert',
    index=1,
    containing_service=None,
    input_type=_INSERTREQUEST,
    output_type=common__pb2._ERROR,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='Segment',
    full_name='protos.Storage.Segment',
    index=2,
    containing_service=None,
    input_type=_SEGMENTREQUEST,
    output_type=_SEGMENTRESPONSE,
    serialized_options=None,
  ),
  _descriptor.MethodDescriptor(
    name='Segments',
    full_name='protos.Storage.Segments',
    index=3,
    containing_service=None,
    input_type=_SEGMENTSREQUEST,
    output_type=_SEGMENTSRESPONSE,
    serialized_options=None,
  ),
])
_sym_db.RegisterServiceDescriptor(_STORAGE)

DESCRIPTOR.services_by_name['Storage'] = _STORAGE

# @@protoc_insertion_point(module_scope)
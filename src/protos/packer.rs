// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `packer.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct SegmentRequest {
    // message fields
    pub segment_id: ::protobuf::SingularPtrField<super::common::SegmentId>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SegmentRequest {
    fn default() -> &'a SegmentRequest {
        <SegmentRequest as ::protobuf::Message>::default_instance()
    }
}

impl SegmentRequest {
    pub fn new() -> SegmentRequest {
        ::std::default::Default::default()
    }

    // .protos.SegmentId segment_id = 1;


    pub fn get_segment_id(&self) -> &super::common::SegmentId {
        self.segment_id.as_ref().unwrap_or_else(|| super::common::SegmentId::default_instance())
    }
    pub fn clear_segment_id(&mut self) {
        self.segment_id.clear();
    }

    pub fn has_segment_id(&self) -> bool {
        self.segment_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_segment_id(&mut self, v: super::common::SegmentId) {
        self.segment_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_segment_id(&mut self) -> &mut super::common::SegmentId {
        if self.segment_id.is_none() {
            self.segment_id.set_default();
        }
        self.segment_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_segment_id(&mut self) -> super::common::SegmentId {
        self.segment_id.take().unwrap_or_else(|| super::common::SegmentId::new())
    }
}

impl ::protobuf::Message for SegmentRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.segment_id {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.segment_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.segment_id.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.segment_id.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SegmentRequest {
        SegmentRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::SegmentId>>(
                    "segment_id",
                    |m: &SegmentRequest| { &m.segment_id },
                    |m: &mut SegmentRequest| { &mut m.segment_id },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<SegmentRequest>(
                    "SegmentRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SegmentRequest {
        static mut instance: ::protobuf::lazy::Lazy<SegmentRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(SegmentRequest::new)
        }
    }
}

impl ::protobuf::Clear for SegmentRequest {
    fn clear(&mut self) {
        self.segment_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SegmentRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SegmentRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SegmentResponse {
    // message fields
    pub segment: ::protobuf::SingularPtrField<super::common::Segment>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SegmentResponse {
    fn default() -> &'a SegmentResponse {
        <SegmentResponse as ::protobuf::Message>::default_instance()
    }
}

impl SegmentResponse {
    pub fn new() -> SegmentResponse {
        ::std::default::Default::default()
    }

    // .protos.Segment segment = 1;


    pub fn get_segment(&self) -> &super::common::Segment {
        self.segment.as_ref().unwrap_or_else(|| super::common::Segment::default_instance())
    }
    pub fn clear_segment(&mut self) {
        self.segment.clear();
    }

    pub fn has_segment(&self) -> bool {
        self.segment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_segment(&mut self, v: super::common::Segment) {
        self.segment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_segment(&mut self) -> &mut super::common::Segment {
        if self.segment.is_none() {
            self.segment.set_default();
        }
        self.segment.as_mut().unwrap()
    }

    // Take field
    pub fn take_segment(&mut self) -> super::common::Segment {
        self.segment.take().unwrap_or_else(|| super::common::Segment::new())
    }
}

impl ::protobuf::Message for SegmentResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.segment {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.segment)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.segment.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.segment.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SegmentResponse {
        SegmentResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Segment>>(
                    "segment",
                    |m: &SegmentResponse| { &m.segment },
                    |m: &mut SegmentResponse| { &mut m.segment },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<SegmentResponse>(
                    "SegmentResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SegmentResponse {
        static mut instance: ::protobuf::lazy::Lazy<SegmentResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(SegmentResponse::new)
        }
    }
}

impl ::protobuf::Clear for SegmentResponse {
    fn clear(&mut self) {
        self.segment.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SegmentResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SegmentResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SegmentsRequest {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SegmentsRequest {
    fn default() -> &'a SegmentsRequest {
        <SegmentsRequest as ::protobuf::Message>::default_instance()
    }
}

impl SegmentsRequest {
    pub fn new() -> SegmentsRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for SegmentsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SegmentsRequest {
        SegmentsRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<SegmentsRequest>(
                    "SegmentsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SegmentsRequest {
        static mut instance: ::protobuf::lazy::Lazy<SegmentsRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(SegmentsRequest::new)
        }
    }
}

impl ::protobuf::Clear for SegmentsRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SegmentsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SegmentsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SegmentsResponse {
    // message fields
    pub segments: ::protobuf::RepeatedField<super::common::SegmentId>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a SegmentsResponse {
    fn default() -> &'a SegmentsResponse {
        <SegmentsResponse as ::protobuf::Message>::default_instance()
    }
}

impl SegmentsResponse {
    pub fn new() -> SegmentsResponse {
        ::std::default::Default::default()
    }

    // repeated .protos.SegmentId segments = 2;


    pub fn get_segments(&self) -> &[super::common::SegmentId] {
        &self.segments
    }
    pub fn clear_segments(&mut self) {
        self.segments.clear();
    }

    // Param is passed by value, moved
    pub fn set_segments(&mut self, v: ::protobuf::RepeatedField<super::common::SegmentId>) {
        self.segments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_segments(&mut self) -> &mut ::protobuf::RepeatedField<super::common::SegmentId> {
        &mut self.segments
    }

    // Take field
    pub fn take_segments(&mut self) -> ::protobuf::RepeatedField<super::common::SegmentId> {
        ::std::mem::replace(&mut self.segments, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for SegmentsResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.segments {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.segments)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.segments {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.segments {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> SegmentsResponse {
        SegmentsResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::SegmentId>>(
                    "segments",
                    |m: &SegmentsResponse| { &m.segments },
                    |m: &mut SegmentsResponse| { &mut m.segments },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<SegmentsResponse>(
                    "SegmentsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static SegmentsResponse {
        static mut instance: ::protobuf::lazy::Lazy<SegmentsResponse> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(SegmentsResponse::new)
        }
    }
}

impl ::protobuf::Clear for SegmentsResponse {
    fn clear(&mut self) {
        self.segments.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SegmentsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SegmentsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ConfigureRequest {
    // message fields
    pub tables: ::protobuf::RepeatedField<super::common::Table>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ConfigureRequest {
    fn default() -> &'a ConfigureRequest {
        <ConfigureRequest as ::protobuf::Message>::default_instance()
    }
}

impl ConfigureRequest {
    pub fn new() -> ConfigureRequest {
        ::std::default::Default::default()
    }

    // repeated .protos.Table tables = 1;


    pub fn get_tables(&self) -> &[super::common::Table] {
        &self.tables
    }
    pub fn clear_tables(&mut self) {
        self.tables.clear();
    }

    // Param is passed by value, moved
    pub fn set_tables(&mut self, v: ::protobuf::RepeatedField<super::common::Table>) {
        self.tables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tables(&mut self) -> &mut ::protobuf::RepeatedField<super::common::Table> {
        &mut self.tables
    }

    // Take field
    pub fn take_tables(&mut self) -> ::protobuf::RepeatedField<super::common::Table> {
        ::std::mem::replace(&mut self.tables, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for ConfigureRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.tables {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tables)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.tables {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.tables {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ConfigureRequest {
        ConfigureRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Table>>(
                    "tables",
                    |m: &ConfigureRequest| { &m.tables },
                    |m: &mut ConfigureRequest| { &mut m.tables },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<ConfigureRequest>(
                    "ConfigureRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static ConfigureRequest {
        static mut instance: ::protobuf::lazy::Lazy<ConfigureRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(ConfigureRequest::new)
        }
    }
}

impl ::protobuf::Clear for ConfigureRequest {
    fn clear(&mut self) {
        self.tables.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ConfigureRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ConfigureRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InsertRequest {
    // message fields
    pub table: ::std::string::String,
    pub row: ::protobuf::SingularPtrField<super::common::Row>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a InsertRequest {
    fn default() -> &'a InsertRequest {
        <InsertRequest as ::protobuf::Message>::default_instance()
    }
}

impl InsertRequest {
    pub fn new() -> InsertRequest {
        ::std::default::Default::default()
    }

    // string table = 1;


    pub fn get_table(&self) -> &str {
        &self.table
    }
    pub fn clear_table(&mut self) {
        self.table.clear();
    }

    // Param is passed by value, moved
    pub fn set_table(&mut self, v: ::std::string::String) {
        self.table = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_table(&mut self) -> &mut ::std::string::String {
        &mut self.table
    }

    // Take field
    pub fn take_table(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.table, ::std::string::String::new())
    }

    // .protos.Row row = 2;


    pub fn get_row(&self) -> &super::common::Row {
        self.row.as_ref().unwrap_or_else(|| super::common::Row::default_instance())
    }
    pub fn clear_row(&mut self) {
        self.row.clear();
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: super::common::Row) {
        self.row = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row(&mut self) -> &mut super::common::Row {
        if self.row.is_none() {
            self.row.set_default();
        }
        self.row.as_mut().unwrap()
    }

    // Take field
    pub fn take_row(&mut self) -> super::common::Row {
        self.row.take().unwrap_or_else(|| super::common::Row::new())
    }
}

impl ::protobuf::Message for InsertRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.row {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.table)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.row)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.table.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.table);
        }
        if let Some(ref v) = self.row.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.table.is_empty() {
            os.write_string(1, &self.table)?;
        }
        if let Some(ref v) = self.row.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> InsertRequest {
        InsertRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "table",
                    |m: &InsertRequest| { &m.table },
                    |m: &mut InsertRequest| { &mut m.table },
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::common::Row>>(
                    "row",
                    |m: &InsertRequest| { &m.row },
                    |m: &mut InsertRequest| { &mut m.row },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<InsertRequest>(
                    "InsertRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static InsertRequest {
        static mut instance: ::protobuf::lazy::Lazy<InsertRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(InsertRequest::new)
        }
    }
}

impl ::protobuf::Clear for InsertRequest {
    fn clear(&mut self) {
        self.table.clear();
        self.row.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InsertRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InsertRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cpacker.proto\x12\x06protos\x1a\x0ccommon.proto\"B\n\x0eSegmentRequ\
    est\x120\n\nsegment_id\x18\x01\x20\x01(\x0b2\x11.protos.SegmentIdR\tsegm\
    entId\"<\n\x0fSegmentResponse\x12)\n\x07segment\x18\x01\x20\x01(\x0b2\
    \x0f.protos.SegmentR\x07segment\"\x11\n\x0fSegmentsRequest\"A\n\x10Segme\
    ntsResponse\x12-\n\x08segments\x18\x02\x20\x03(\x0b2\x11.protos.SegmentI\
    dR\x08segments\"9\n\x10ConfigureRequest\x12%\n\x06tables\x18\x01\x20\x03\
    (\x0b2\r.protos.TableR\x06tables\"D\n\rInsertRequest\x12\x14\n\x05table\
    \x18\x01\x20\x01(\tR\x05table\x12\x1d\n\x03row\x18\x02\x20\x01(\x0b2\x0b\
    .protos.RowR\x03row2\xb9\x01\n\x06Packer\x120\n\x06Insert\x12\x15.protos\
    .InsertRequest\x1a\r.protos.Error\"\0\x12<\n\x07Segment\x12\x16.protos.S\
    egmentRequest\x1a\x17.protos.SegmentResponse\"\0\x12?\n\x08Segments\x12\
    \x17.protos.SegmentsRequest\x1a\x18.protos.SegmentsResponse\"\0J\x96\x06\
    \n\x06\x12\x04\0\0%\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x02\x08\x0e\n\t\n\x02\x03\0\x12\x03\x04\x07\x15\n\n\n\x02\x04\0\
    \x12\x04\x06\0\x08\x01\n\n\n\x03\x04\0\x01\x12\x03\x06\x08\x16\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\x07\x02\x1b\n\r\n\x05\x04\0\x02\0\x04\x12\x04\
    \x07\x02\x06\x18\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x07\x02\x0b\n\x0c\n\
    \x05\x04\0\x02\0\x01\x12\x03\x07\x0c\x16\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\x07\x19\x1a\n\n\n\x02\x04\x01\x12\x04\n\0\x0c\x01\n\n\n\x03\x04\x01\
    \x01\x12\x03\n\x08\x17\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x0b\x02\x17\n\r\
    \n\x05\x04\x01\x02\0\x04\x12\x04\x0b\x02\n\x19\n\x0c\n\x05\x04\x01\x02\0\
    \x06\x12\x03\x0b\x02\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x0b\n\x11\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x0b\x15\x16\n\n\n\x02\x04\x02\x12\
    \x04\x0e\0\x0f\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0e\x08\x17\n\n\n\x02\
    \x04\x03\x12\x04\x11\0\x13\x01\n\n\n\x03\x04\x03\x01\x12\x03\x11\x08\x18\
    \n\x0b\n\x04\x04\x03\x02\0\x12\x03\x12\x02\"\n\x0c\n\x05\x04\x03\x02\0\
    \x04\x12\x03\x12\x02\n\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x03\x12\x0b\x14\
    \n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x12\x15\x1d\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x03\x12\x20!\n\n\n\x02\x04\x04\x12\x04\x17\0\x19\x01\n\n\
    \n\x03\x04\x04\x01\x12\x03\x17\x08\x18\n\x0b\n\x04\x04\x04\x02\0\x12\x03\
    \x18\x02\x1c\n\x0c\n\x05\x04\x04\x02\0\x04\x12\x03\x18\x02\n\n\x0c\n\x05\
    \x04\x04\x02\0\x06\x12\x03\x18\x0b\x10\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03\x18\x11\x17\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03\x18\x1a\x1b\n\n\n\
    \x02\x04\x05\x12\x04\x1b\0\x1e\x01\n\n\n\x03\x04\x05\x01\x12\x03\x1b\x08\
    \x15\n\x0b\n\x04\x04\x05\x02\0\x12\x03\x1c\x02\x13\n\r\n\x05\x04\x05\x02\
    \0\x04\x12\x04\x1c\x02\x1b\x17\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03\x1c\
    \x02\x08\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03\x1c\t\x0e\n\x0c\n\x05\x04\
    \x05\x02\0\x03\x12\x03\x1c\x11\x12\n\x0b\n\x04\x04\x05\x02\x01\x12\x03\
    \x1d\x02\x0e\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04\x1d\x02\x1c\x13\n\x0c\
    \n\x05\x04\x05\x02\x01\x06\x12\x03\x1d\x02\x05\n\x0c\n\x05\x04\x05\x02\
    \x01\x01\x12\x03\x1d\x06\t\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03\x1d\
    \x0c\r\n\n\n\x02\x06\0\x12\x04!\0%\x01\n\n\n\x03\x06\0\x01\x12\x03!\x08\
    \x0e\n\x0b\n\x04\x06\0\x02\0\x12\x03\"\x02.\n\x0c\n\x05\x06\0\x02\0\x01\
    \x12\x03\"\x06\x0c\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\"\r\x1a\n\x0c\n\
    \x05\x06\0\x02\0\x03\x12\x03\"%*\n\x0b\n\x04\x06\0\x02\x01\x12\x03#\x02:\
    \n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03#\x06\r\n\x0c\n\x05\x06\0\x02\x01\
    \x02\x12\x03#\x0e\x1c\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03#'6\n\x0b\n\
    \x04\x06\0\x02\x02\x12\x03$\x02=\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03$\
    \x06\x0e\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03$\x0f\x1e\n\x0c\n\x05\x06\
    \0\x02\x02\x03\x12\x03$)9b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

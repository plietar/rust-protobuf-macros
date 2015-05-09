// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Person {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    id: ::std::option::Option<i32>,
    email: ::protobuf::RepeatedField<::std::string::String>,
    phone: ::protobuf::RepeatedField<Person_PhoneNumber>,
    job: ::protobuf::SingularPtrField<Person_Job>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Person {
    pub fn new() -> Person {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Person {
        static mut instance: ::protobuf::lazy::Lazy<Person> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Person,
        };
        unsafe {
            instance.get(|| {
                Person {
                    name: ::protobuf::SingularField::none(),
                    id: ::std::option::Option::None,
                    email: ::protobuf::RepeatedField::new(),
                    phone: ::protobuf::RepeatedField::new(),
                    job: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required int32 id = 2;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i32) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    // repeated string email = 3;

    pub fn clear_email(&mut self) {
        self.email.clear();
    }

    // Param is passed by value, moved
    pub fn set_email(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.email = v;
    }

    // Mutable pointer to the field.
    pub fn mut_email<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.email
    }

    // Take field
    pub fn take_email(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.email, ::protobuf::RepeatedField::new())
    }

    pub fn get_email<'a>(&'a self) -> &'a [::std::string::String] {
        &self.email
    }

    // repeated .Person.PhoneNumber phone = 4;

    pub fn clear_phone(&mut self) {
        self.phone.clear();
    }

    // Param is passed by value, moved
    pub fn set_phone(&mut self, v: ::protobuf::RepeatedField<Person_PhoneNumber>) {
        self.phone = v;
    }

    // Mutable pointer to the field.
    pub fn mut_phone<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Person_PhoneNumber> {
        &mut self.phone
    }

    // Take field
    pub fn take_phone(&mut self) -> ::protobuf::RepeatedField<Person_PhoneNumber> {
        ::std::mem::replace(&mut self.phone, ::protobuf::RepeatedField::new())
    }

    pub fn get_phone<'a>(&'a self) -> &'a [Person_PhoneNumber] {
        &self.phone
    }

    // optional .Person.Job job = 5;

    pub fn clear_job(&mut self) {
        self.job.clear();
    }

    pub fn has_job(&self) -> bool {
        self.job.is_some()
    }

    // Param is passed by value, moved
    pub fn set_job(&mut self, v: Person_Job) {
        self.job = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_job<'a>(&'a mut self) -> &'a mut Person_Job {
        if self.job.is_none() {
            self.job.set_default();
        };
        self.job.as_mut().unwrap()
    }

    // Take field
    pub fn take_job(&mut self) -> Person_Job {
        self.job.take().unwrap_or_else(|| Person_Job::new())
    }

    pub fn get_job<'a>(&'a self) -> &'a Person_Job {
        self.job.as_ref().unwrap_or_else(|| Person_Job::default_instance())
    }
}

impl ::protobuf::Message for Person {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.email));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.phone));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.job.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.email.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.phone.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.job.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.id {
            try!(os.write_int32(2, v));
        };
        for v in self.email.iter() {
            try!(os.write_string(3, &v));
        };
        for v in self.phone.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.job.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Person>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Person {
    fn new() -> Person {
        Person::new()
    }

    fn descriptor_static(_: ::std::option::Option<Person>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Person::has_name,
                    Person::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "id",
                    Person::has_id,
                    Person::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "email",
                    Person::get_email,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "phone",
                    Person::get_phone,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "job",
                    Person::has_job,
                    Person::get_job,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Person>(
                    "Person",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Person {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_id();
        self.clear_email();
        self.clear_phone();
        self.clear_job();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.name == other.name &&
        self.id == other.id &&
        self.email == other.email &&
        self.phone == other.phone &&
        self.job == other.job &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Person {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Person_PhoneNumber {
    // message fields
    number: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<Person_PhoneType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Person_PhoneNumber {
    pub fn new() -> Person_PhoneNumber {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Person_PhoneNumber {
        static mut instance: ::protobuf::lazy::Lazy<Person_PhoneNumber> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Person_PhoneNumber,
        };
        unsafe {
            instance.get(|| {
                Person_PhoneNumber {
                    number: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string number = 1;

    pub fn clear_number(&mut self) {
        self.number.clear();
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: ::std::string::String) {
        self.number = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_number<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.number.is_none() {
            self.number.set_default();
        };
        self.number.as_mut().unwrap()
    }

    // Take field
    pub fn take_number(&mut self) -> ::std::string::String {
        self.number.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_number<'a>(&'a self) -> &'a str {
        match self.number.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .Person.PhoneType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Person_PhoneType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> Person_PhoneType {
        self.field_type.unwrap_or(Person_PhoneType::HOME)
    }
}

impl ::protobuf::Message for Person_PhoneNumber {
    fn is_initialized(&self) -> bool {
        if self.number.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.number.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.number.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.number.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(2, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Person_PhoneNumber>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Person_PhoneNumber {
    fn new() -> Person_PhoneNumber {
        Person_PhoneNumber::new()
    }

    fn descriptor_static(_: ::std::option::Option<Person_PhoneNumber>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "number",
                    Person_PhoneNumber::has_number,
                    Person_PhoneNumber::get_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    Person_PhoneNumber::has_field_type,
                    Person_PhoneNumber::get_field_type,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Person_PhoneNumber>(
                    "Person_PhoneNumber",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Person_PhoneNumber {
    fn clear(&mut self) {
        self.clear_number();
        self.clear_field_type();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Person_PhoneNumber {
    fn eq(&self, other: &Person_PhoneNumber) -> bool {
        self.number == other.number &&
        self.field_type == other.field_type &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Person_PhoneNumber {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Person_Job {
    // message fields
    title: ::protobuf::SingularField<::std::string::String>,
    company: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Person_Job {
    pub fn new() -> Person_Job {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Person_Job {
        static mut instance: ::protobuf::lazy::Lazy<Person_Job> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Person_Job,
        };
        unsafe {
            instance.get(|| {
                Person_Job {
                    title: ::protobuf::SingularField::none(),
                    company: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string title = 1;

    pub fn clear_title(&mut self) {
        self.title.clear();
    }

    pub fn has_title(&self) -> bool {
        self.title.is_some()
    }

    // Param is passed by value, moved
    pub fn set_title(&mut self, v: ::std::string::String) {
        self.title = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_title<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.title.is_none() {
            self.title.set_default();
        };
        self.title.as_mut().unwrap()
    }

    // Take field
    pub fn take_title(&mut self) -> ::std::string::String {
        self.title.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_title<'a>(&'a self) -> &'a str {
        match self.title.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string company = 2;

    pub fn clear_company(&mut self) {
        self.company.clear();
    }

    pub fn has_company(&self) -> bool {
        self.company.is_some()
    }

    // Param is passed by value, moved
    pub fn set_company(&mut self, v: ::std::string::String) {
        self.company = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_company<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.company.is_none() {
            self.company.set_default();
        };
        self.company.as_mut().unwrap()
    }

    // Take field
    pub fn take_company(&mut self) -> ::std::string::String {
        self.company.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_company<'a>(&'a self) -> &'a str {
        match self.company.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Person_Job {
    fn is_initialized(&self) -> bool {
        if self.title.is_none() {
            return false;
        };
        if self.company.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.title.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.company.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.title.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.company.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.title.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.company.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Person_Job>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Person_Job {
    fn new() -> Person_Job {
        Person_Job::new()
    }

    fn descriptor_static(_: ::std::option::Option<Person_Job>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "title",
                    Person_Job::has_title,
                    Person_Job::get_title,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "company",
                    Person_Job::has_company,
                    Person_Job::get_company,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Person_Job>(
                    "Person_Job",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Person_Job {
    fn clear(&mut self) {
        self.clear_title();
        self.clear_company();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Person_Job {
    fn eq(&self, other: &Person_Job) -> bool {
        self.title == other.title &&
        self.company == other.company &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Person_Job {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Person_PhoneType {
    MOBILE = 0,
    HOME = 1,
    WORK = 2,
}

impl ::protobuf::ProtobufEnum for Person_PhoneType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Person_PhoneType> {
        match value {
            0 => ::std::option::Option::Some(Person_PhoneType::MOBILE),
            1 => ::std::option::Option::Some(Person_PhoneType::HOME),
            2 => ::std::option::Option::Some(Person_PhoneType::WORK),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Person_PhoneType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Person_PhoneType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Person_PhoneType {
}

#[derive(Clone,Default)]
pub struct AddressBook {
    // message fields
    person: ::protobuf::RepeatedField<Person>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AddressBook {
    pub fn new() -> AddressBook {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AddressBook {
        static mut instance: ::protobuf::lazy::Lazy<AddressBook> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AddressBook,
        };
        unsafe {
            instance.get(|| {
                AddressBook {
                    person: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .Person person = 1;

    pub fn clear_person(&mut self) {
        self.person.clear();
    }

    // Param is passed by value, moved
    pub fn set_person(&mut self, v: ::protobuf::RepeatedField<Person>) {
        self.person = v;
    }

    // Mutable pointer to the field.
    pub fn mut_person<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Person> {
        &mut self.person
    }

    // Take field
    pub fn take_person(&mut self) -> ::protobuf::RepeatedField<Person> {
        ::std::mem::replace(&mut self.person, ::protobuf::RepeatedField::new())
    }

    pub fn get_person<'a>(&'a self) -> &'a [Person] {
        &self.person
    }
}

impl ::protobuf::Message for AddressBook {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.person));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.person.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.person.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AddressBook>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AddressBook {
    fn new() -> AddressBook {
        AddressBook::new()
    }

    fn descriptor_static(_: ::std::option::Option<AddressBook>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "person",
                    AddressBook::get_person,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AddressBook>(
                    "AddressBook",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AddressBook {
    fn clear(&mut self) {
        self.clear_person();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AddressBook {
    fn eq(&self, other: &AddressBook) -> bool {
        self.person == other.person &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AddressBook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x62, 0x6f, 0x6f, 0x6b, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x89, 0x02, 0x0a, 0x06, 0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02,
    0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x05, 0x12, 0x0d, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69,
    0x6c, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x12, 0x22, 0x0a, 0x05, 0x70, 0x68, 0x6f, 0x6e, 0x65,
    0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x2e,
    0x50, 0x68, 0x6f, 0x6e, 0x65, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x18, 0x0a, 0x03, 0x6a,
    0x6f, 0x62, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x50, 0x65, 0x72, 0x73, 0x6f,
    0x6e, 0x2e, 0x4a, 0x6f, 0x62, 0x1a, 0x44, 0x0a, 0x0b, 0x50, 0x68, 0x6f, 0x6e, 0x65, 0x4e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x12, 0x0e, 0x0a, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x25, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0e, 0x32, 0x11, 0x2e, 0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x2e, 0x50, 0x68, 0x6f, 0x6e,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x04, 0x48, 0x4f, 0x4d, 0x45, 0x1a, 0x25, 0x0a, 0x03, 0x4a,
    0x6f, 0x62, 0x12, 0x0d, 0x0a, 0x05, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x6e, 0x79, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x22, 0x2b, 0x0a, 0x09, 0x50, 0x68, 0x6f, 0x6e, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x0a, 0x0a, 0x06, 0x4d, 0x4f, 0x42, 0x49, 0x4c, 0x45, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x48,
    0x4f, 0x4d, 0x45, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x57, 0x4f, 0x52, 0x4b, 0x10, 0x02, 0x22,
    0x26, 0x0a, 0x0b, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x42, 0x6f, 0x6f, 0x6b, 0x12, 0x17,
    0x0a, 0x06, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x07,
    0x2e, 0x50, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x4a, 0x93, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x00, 0x00, 0x18, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x00, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x01, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x01, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x01, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x01, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x01, 0x19, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x02, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x02, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x02, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x02, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x02, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x03, 0x02,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x03, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x03, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x03, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04,
    0x00, 0x12, 0x04, 0x05, 0x02, 0x09, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x05, 0x07, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x06, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x06, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x06, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x07, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x07, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x07, 0x0b, 0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08,
    0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08,
    0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x08,
    0x0b, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x00, 0x12, 0x04, 0x0b, 0x02, 0x0e, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x0a, 0x15, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x04, 0x1f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0c, 0x04, 0x0c, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0c, 0x0d, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x14, 0x1a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x04, 0x31, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0d, 0x0d, 0x16, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0d, 0x17, 0x1b, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d, 0x1e, 0x1f, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x0d, 0x20, 0x30, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x07, 0x12, 0x03, 0x0d, 0x2b, 0x2f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x10, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06,
    0x12, 0x03, 0x10, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x10, 0x17, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x01, 0x12, 0x04, 0x12, 0x02, 0x15, 0x03, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x12, 0x0a, 0x0d, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x13, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x13, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x14, 0x19, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x14, 0x04, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x14, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x14, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x04, 0x12, 0x03, 0x17, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x17, 0x0b, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x17,
    0x0f, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x17, 0x15, 0x16,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1a, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x01, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00,
    0x12, 0x03, 0x1b, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1b, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x12, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x1b, 0x1c,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

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

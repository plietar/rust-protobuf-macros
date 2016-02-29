# rust-protobuf-macros
Macros designed to make protobuf easier to use in rust.

The macros use [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf),
but provide a simpler syntax to set and get fields from protobuf objects.

## Installation
The macros are implemented using a compiler plugin, which requires rust nightly.
rust beta will NOT work !

Add to your `Cargo.toml`
```toml
[dependencies.protobuf]
git = "https://github.com/stepancheg/rust-protobuf.git"
[dependencies.protobuf_macros]
git = "https://github.com/plietar/rust-protobuf-macros.git"
```

Then enable it in your crate :
```rust
#![feature(plugin)]
#![plugin(protobuf_macros)]
```

## Usage
The examples use the following schema :
```protobuf
message Person {
  required string name = 1;
  required int32 id = 2;
  repeated string email = 3;

  enum PhoneType {
    MOBILE = 0;
    HOME = 1;
    WORK = 2;
  }

  message PhoneNumber {
    required string number = 1;
    optional PhoneType type = 2 [default = HOME];
  }

  repeated PhoneNumber phone = 4;

  message Job {
    required string title = 1;
    required string company = 2;
  }

  optional Job job = 5;
}

message AddressBook {
  repeated Person person = 1;
}
```

### protobuf_init!
The `protobuf_init!` macro is used to fill an existing protobuf object.

```rust
let person = protobuf_init!(Person::new(), {
    name: "Joe",
    id: 42,
    email => [
        "joe@domain.com",
        "joe@other.com"
    ],
    phone => [
        @{
            number: "0123456789",
            field_type: Person_PhoneType::HOME
        },
        @{
            number: "9876543210",
            field_type: Person_PhoneType::WORK
        }
    ]

    job => {
        title: "Boss",
        company: "Big Corp"
    }
});
```

### protobuf_bind!
The `protobuf_bind!` macro is used to extract data from a protobuf object.
The variable names on the right are created by the macro using a let statement.

```rust
protobuf_bind!(person, {
    name: person_name,
    id: person_id,
    email: emails,
    phone: phone_numbers,
    job => {
        title: job_title,
        company: company
    }
});
```

Note that it is not possible to extract the repeated fields. Instead, you will get
a `RepeatedField` object. You can iterate on it, and call `protobuf_bind!` again
on its elements.

```rust
for phone in phone_numbers {
    protobuf_bind!(phone, {
        number: number,
        field_type: phone_type,
    });
}
```


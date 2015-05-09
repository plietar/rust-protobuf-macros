#![feature(plugin)]
#![plugin(protobuf_macros)]

extern crate protobuf;

mod addressbook;

use addressbook::{Person, Person_PhoneType};

fn main() {
    let person = protobuf_init!(Person::new(), {
        name: "Joe".to_string(),
        id: 42,
        email => [
            "joe@domain.com".to_string(),
            "joe@other.com".to_string()
        ],
        phone => [
            @{
                number: "0123456789".to_string(),
                field_type: Person_PhoneType::HOME
            },
            @{
                number: "9876543210".to_string(),
                field_type: Person_PhoneType::WORK
            }
        ],
        job => {
            title: "Boss".to_string(),
            company: "Big Corp".to_string()
        }
    });

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

    println!("Name: {}", person_name);
    println!("ID: {}", person_id);
    println!("Job: {} at {}", job_title, company);

    println!("Email: ");
    for email in emails {
        println!(" * {}", email);
    }

    println!("Phone:");
    for phone in phone_numbers {
        protobuf_bind!(phone, {
            number: number,
            field_type: phone_type,
        });
        println!(" * {} ({:?})", number, phone_type);
    }
}


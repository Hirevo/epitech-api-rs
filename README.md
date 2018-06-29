# EPITECH-API

This is a Rust library built on top of [reqwest](https://github.com/seanmonstar/reqwest/) for interacting with the EPITECH intranet.  
This library focuses on ease-of-use and type-safety.  

I am quite new at Rust and even more so in Rust library design, so any help or idea to improve the API is very much welcome.  

## Goal

This library attempts to make use of the type-system and the builder-pattern to make sure that if the code compiles, all intranet requests at runtime are guaranteed to be valid.  

It also aims to stick a type on intranet resources so that every possible members are clearly represented and safely accessible (through custom Deserialize trait implementations).  

## How to use

Everything originates from the `EpitechClient` struct.

You can create an `EpitechClient` this way:
```rust
let result = EpitechClient::builder()
    .autologin("[INSERT AUTOLOGIN LINK HERE]")
    .authenticate(); // This returns a `Result<EpitechClient, Error>`.

let client = match result {
    Some(client) => client,
    None => , // Handle authentication error here.
};
```
Right after this, you're already authenticated to the intranet and ready to proceed with requests.

You can, for instance, request the list of all students in a promotion this way:
```rust
// This makes the request and returns a `Option<Vec<UserEntry>>`.
let result = api.fetch_student_list()
    .location(Location::Strasbourg)
    .promo(Promo::Tek2)
    .year(2017)
    .send();
```

`make_request` allows you to make an arbitrary request to the intranet (therefore also losing type-safety):
```rust
// Notice that only the path component of the route can be passed to the method.
let my_student_infos = match client.make_request("/user") {
    Some(text) => , // Here, `text` represents the raw intranet response body.
    None => , // Handle request error here.
};
```

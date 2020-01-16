EPITECH-API
===========

![Crates.io](https://img.shields.io/crates/v/epitech_api)
![Crates.io](https://img.shields.io/crates/l/epitech_api)
![Crates.io](https://img.shields.io/crates/d/epitech_api)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=Hirevo/epitech-api-rs)](https://dependabot.com)


This is a Rust library built on top of [reqwest](https://github.com/seanmonstar/reqwest) for interacting with the EPITECH intranet.  
This library focuses on ease-of-use and type-safety.  

Goal
----

This project aims to stick a type on intranet resources so that every possible members are clearly represented and safely accessible.  

How to use
----------

Everything originates from the `Client` struct.

You can create an `Client` this way:

```rust
use epitech_api::{Client, Error};

let result = Client::builder()
    .autologin("[INSERT AUTOLOGIN LINK HERE]")
    .authenticate()
    .await; // This returns a `Result<Client, Error>`.

let client = match result {
    Ok(client) => client,
    Err(err) => , // Handle authentication error here.
};
```

Right after this, you're already authenticated to the intranet and ready to proceed with requests.

You can, for instance, request the list of all students in a promotion this way:

```rust
// This makes the request and returns a `Result<Vec<UserEntry>, Error>`.
let result = client.fetch_student_list()
    .location(Location::Strasbourg)
    .promo(Promo::Tek2)
    .year(2020)
    .send()
    .await;
```

`Client::make_request` allows you to make an arbitrary request to the intranet:

```rust
// Notice that only the path component of the route can be passed to the method.
let my_student_infos = match client.make_request("/user").await {
    Ok(text: String) => , // Here, `text` represents the raw intranet response body.
    Err(err: Error) => , // Handle request error here.
};
```

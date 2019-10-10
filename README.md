EPITECH-API
===========

This is a Rust library built on top of [reqwest](https://github.com/seanmonstar/reqwest/) for interacting with the EPITECH intranet.  
This library focuses on ease-of-use and type-safety.  

Goal
----

This project aims to stick a type on intranet resources so that every possible members are clearly represented and safely accessible (through custom Deserialize trait implementations).  

How to use
----------

Everything originates from the `EpitechClient` struct.

You can create an `EpitechClient` this way:

```rust
let result = EpitechClient::builder()
    .autologin("[INSERT AUTOLOGIN LINK HERE]")
    .authenticate(); // This returns a `Result<EpitechClient, EpitechClientError>`.

let client = match result {
    Ok(client) => client,
    Err(err) => , // Handle authentication error here.
};
```

Right after this, you're already authenticated to the intranet and ready to proceed with requests.

You can, for instance, request the list of all students in a promotion this way:

```rust
// This makes the request and returns a `Result<Vec<UserEntry>, EpitechClientError>`.
let result = api.fetch_student_list()
    .location(Location::Strasbourg)
    .promo(Promo::Tek2)
    .year(2017)
    .send();
```

`EpitechClient::make_request` allows you to make an arbitrary request to the intranet (therefore also losing type-safety):

```rust
// Notice that only the path component of the route can be passed to the method.
let my_student_infos = match client.make_request("/user") {
    Ok(text) => , // Here, `text` represents the raw intranet response body.
    Err(err) => , // Handle request error here.
};
```

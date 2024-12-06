# Tapi


![Platform](https://img.shields.io/badge/platform-windows%20%7C%20macos%20%7C%20linux-lightgrey)
![Release Version](https://img.shields.io/github/v/release/jolvera2001/tapi?include_prereleases)
[![Coverage](https://codecov.io/gh/jolvera2001/tapi/graph/badge.svg)](https://codecov.io/gh/jolvera2001/tapi)

Tapi is a small project I thought of to learn more about Rust and using Wasm with it. This project is essentially just Postman, and API tester where you can send API requests with parameters and Json Bodies. Currently, the current version only does:

- Making requests without saving
- Specifying parameters
- Include JSON bodies
- Do the usual GET, POST, PUT, DELETE calls

Some other features are being planned, like being able to save requests in collections, and some online storage syncing is being considered as well.

Instructions on using this will come out later, I gotta figure out how to set up Github actions to make released for each build and stuff.

## Tech Stack

Mentioned before, this project was just a way for me to learn more about rust (in the roughest way possible tbh). This project is using:

- Tauri
- Wasm (Syncamore as the frontend)

Could I have used js instead? Yeah, but that's not that fun.

## Future plans

- [ ] Version 0.2.0
  - [ ] Request saving
  - [ ] Collections storing requests
  - [ ] Sidebar requests tree view ( with whatever UI I think fits)
- [ ] Version 0.3.0
  - [ ] gRPC support

## Thoughts

Although this isn't meant to *replace* Postman, this is one of the few projects I've felt interested in putting a lot more work into. This is more of a learning experience for me, especially since now I'm thinking about setting up Github actions and outline some actual versioning plans.

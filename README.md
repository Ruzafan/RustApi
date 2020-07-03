# RustApi

For runing this API you will need to have installed first:
- Rust https://www.rust-lang.org/tools/install
- A mongodb instance (without credentials) running on port 27017, you can use docker https://hub.docker.com/_/mongo

And that's it!

I'm usually working with vs code, but another IDE will also be fine.


Once we are ready to start the API we will open a terminal and type: 
- cargo build (to build the project)
- cargo run (to start the API)


Available endpoint:

- localhost:3030/add_user/{user_name}
- localhost:3030/get_user/{user_name}

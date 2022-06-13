# spinning up for local dev

    sudo docker-compose up

then, in another terminal:

    sqlx migrate run

    cargo run

# creating migrations

    sqlx migrate add

I need to start writing down migrations for the case that a migration is applied to prod that needs to be undone. I don't think sqlx has a way to distinguish between up or down migrations in the migrations folder, so for now those will just go in the down_migrations folder and can just be moved into the migrations folder if necessary.

# general organization

Each entity has it's own folder, with a model.rs file which basically has the structs for use in request handlers, and and implementation with functions for db interactions like creating, deleting, updating, & finding.

On top of that, most of the main entities also have an api_handlers folder, where I write the handlers for different routes related to those entities. Those handlers are then referenced in the routes folder, where they're all given namespaces in an init function that gets passed to main

There's also a utils folder, where I put things like session/policy validation that is used in request handlers to reduce the amount of code that gets repeated everywhere. Theres probably more stuff that can get moved there, but I haven't really had the time to see it yet.

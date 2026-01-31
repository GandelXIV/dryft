# How to add a backend
1. Create an aptly named directory in `src/backend/`.
2. Inside, make a `mod.rs` file.
3. In `src/backend.rs`, add your module on top with all the other ones.
4. Define a codename for your BE in the `select` function.
5. In your `mod.rs`, Import the `Backend` trait and implement ALL the required functions.
6. Finally, create your build profile in `src/targets/`
7. Test it, and ship it!

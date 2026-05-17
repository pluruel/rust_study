// Entry point of the binary crate
// Since lib.rs is a separate crate, access it via the package name (restaurant), not crate::
// librestaurant.rlib is statically linked at build time into a single binary

fn main() {
    restaurant::eat_at_restaurant();
}

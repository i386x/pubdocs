mod foo;
mod quux;

fn main() {
    crate::quux::baz::inline::inner::hello();
    crate::foo::bar::fthread::fthread_local::join();
}

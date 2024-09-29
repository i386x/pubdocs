fn implements_clone<T: Clone>(x: &T) {
    let _ = x.clone();

    println!("  * closure implements `Clone`");
}

fn implements_copy<T: Copy>(x: &T) {
    let _ = x;

    println!("  * closure implements `Copy`");
}

fn implements_fn_once<F: FnOnce(&str)>(f: F, s: &str) {
    f(s);
}

fn implements_fn_mut<F: FnMut(&str)>(mut f: F, s: &str) {
    f(s);
}

fn implements_fn<F: Fn(&str)>(f: F, s: &str) {
    f(s);
}

fn immutable_capture() {
    let s = String::from("abc");

    println!("Immutable capture demo:");

    // Taking immutable reference:
    println!("  * taking immutable reference: {s}");

    // `s` captured by immutable reference:
    let f = |x: &str| println!("{x}: {s}");

    // Allowed as we can have as many immutable references as we can:
    println!("  * taking immutable reference again: {s}");

    // `f` does not move out any captured variables,
    // hence `f` implements `FnMut`:
    implements_fn_mut(f, "  * closure implements `FnMut`");

    // `f` does not mutate or move out any captured variables,
    // hence `f` implements `Fn`:
    implements_fn(f, "  * closure implements `Fn`");

    // `f` implements `FnOnce` by default:
    implements_fn_once(f, "  * closure implements `FnOnce`");

    // Allowed as we can have as many immutable references as we can:
    println!("  * taking immutable reference for the third time: {s}");
}

fn unique_immutable_capture() {
    let mut b = false;
    // Borrow as mutable:
    let mrb = &mut b;

    println!("Unique immutable capture demo:");

    // `b` is borrowed as mutable so it cannot be borrowed as immutable:
    // println!("  * taking immutable reference: {b}");  // Error

    // `mrb` is borrowed here as immutable:
    println!("  * taking immutable reference of mutable reference: {mrb}");

    // `mrb` is captured by unique immutable reference:
    //   * `mut` is required because of `&mut f` used later
    let mut f = |x: &str| {
        println!("{x}");
        *mrb = true;
    };

    // `b` is borrowed as mutable so it cannot be borrowed as immutable:
    // println!("  * taking immutable reference again: {b}");  // Error

    // `mrb` cannot be borrowed as immutable because `f` requires unique
    // access:
    // println!("  * taking immutable reference of mutable reference again: {mrb}");  // Error

    // `implements_fn_mut(f)` is not possible because:
    //   * `f` is used later in `implements_fn_once(f)` expression which means
    //     `implements_fn_mut` cannot move `f`
    //   * `f` does not implement `Copy` because `mrb` is captured by unique
    //     immutable reference
    //
    // `implements_fn_mut(&f)` is not possible because `&f` is not possible
    // because `f` does not implement `Fn` because `f` mutates its captured
    // variable.
    //
    // Hence `implements_fn_mut(&mut f)` is the only possibility as `f`
    // implements `FnMut` as `f` does not move out captured variables and hence
    // it can be called by mutable reference (`&mut f`):
    implements_fn_mut(&mut f, "  * closure implements `FnMut`");

    // `f` does not implement `Fn`:
    // implements_fn(f, "  * closure implements `Fn`");  // Error

    // `f` implements `FnOnce` by default, `implements_fn_once` takes ownership
    // of `f`:
    implements_fn_once(f, "  * closure implements `FnOnce`");

    // This is correct because lifetime of both `f` and `mrb` ends here:
    println!("  * taking immutable reference after the closure drop: {b}");
}

fn mutable_capture() {
    let mut v = vec![1];

    println!("Mutable capture demo:");

    // `v` is borrowed here as immutable:
    println!("  * taking immutable reference of mutable object: {v:?}");

    // `v` is captured by mutable reference:
    //   * `mut` is required because of `&mut f` used later
    let mut f = |x: &str| {
        v.push(2);
        println!("{x}");
    };

    // `v` cannot be borrowed as immutable since it has been captured as
    // mutable inside `f`:
    // println!("  * taking immutable reference of mutable object for the 2nd time: {v:?}");  // Error

    // `f` does not move out captured variables and hence `f` implements
    // `FnMut`:
    implements_fn_mut(&mut f, "  * closure implements `FnMut`");

    // `v` cannot be borrowed as immutable since it has been captured as
    // mutable inside `f`:
    // println!("  * taking immutable reference of mutable object for the 3rd time: {v:?}");  // Error

    // `f` does not implement `Fn`:
    // implements_fn(f, "  * closure implements `Fn`");  // Error

    // `f` implements `FnOnce` by default, ownership of `f` is taken here:
    implements_fn_once(f, "  * closure implements `FnOnce`");

    // This is correct because `f` is dropped here and so the mutable reference
    // to `v`:
    println!("  * taking immutable reference of mutable object after the closure drop: {v:?}");
}

#[derive(Debug)]
struct NonCloneable;

#[derive(Clone, Debug)]
struct Cloneable;

#[derive(Copy, Clone, Debug)]
struct Copyable;

struct EventHandler {
    poke_handler: Box<dyn Fn()>,
    close_handler: Box<dyn FnOnce()>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self {
            poke_handler: Box::new(|| {}),
            close_handler: Box::new(|| {}),
        }
    }
}

impl EventHandler {
    fn new() -> Self {
        Self::default()
    }

    fn on_poke<F>(&mut self, f: F)
    where
        F: Fn() + 'static,
    {
        self.poke_handler = Box::new(f);
    }

    fn on_close<F>(&mut self, f: F)
    where
        F: FnOnce() + 'static,
    {
        self.close_handler = Box::new(f);
    }

    fn poke(&self) {
        (self.poke_handler)();
    }

    fn close(self) {
        (self.close_handler)();
    }
}

fn move_capture() {
    println!("Move capture demo:");

    {
        let mut eh = EventHandler::new();

        {
            let token = NonCloneable;

            // `token` is captured by move:
            //   * `f` now owns `token`
            //   * notice that `move` is not used here, compiler can deduce it
            let f = || {
                let token = token;

                println!("  * {token:?}");
            };

            // `token` is not valid here since it is owned now by `f`:
            // println!("  * taking immutable reference after move: {token:?}");  // Error

            // `f` does not implement `Clone` because its captured variables do
            // not implement `Clone`:
            // implements_clone(&f);  // Error

            // `f` does not implement `Copy` because its captured variables do
            // not implement `Copy`:
            // implements_copy(&f);  // Error

            // `f` is moved outside of this scope together with `token` which
            // it owns:
            eh.on_close(f);
        }

        // `f` and `token` are used here:
        eh.close();

        // `f` and `token` are dropped here.
    }

    {
        let mut eh = EventHandler::new();

        {
            let token = Cloneable;

            // `token` is captured by move:
            //   * `f` now owns `token`
            //   * notice that `move` is not used here, compiler can deduce it
            let f = || {
                let token = token;

                println!("  * {token:?}");
            };

            // `token` is not valid here since it is owned now by `f`:
            // println!("  * taking immutable reference after move: {token:?}");  // Error

            // `f` implements `Clone` because all its variables are captured by
            // move and implement `Clone`:
            implements_clone(&f);

            // `f` does not implement `Copy` because its captured variables do
            // not implement `Copy`:
            // implements_copy(&f);  // Error

            // `f` is moved outside of this scope together with `token` which
            // it owns:
            eh.on_close(f);
        }

        // `f` and `token` are used here:
        eh.close();

        // `f` and `token` are dropped here.
    }

    {
        let mut eh = EventHandler::new();

        {
            let token = Copyable;

            // `token` is captured by copy:
            //   * `move` keyword is necessary because without `move` compiler
            //     complains that `f` may outlive borrowed `token`; in fact,
            //     since `token` implements `Copy`, compiler prefers the
            //     capture by copy but this requires the presence of the `move`
            //     keyword
            let f = move || {
                let token = token;

                println!("  * {token:?}");
            };

            // This is correct because `token` has been captured by copy:
            println!("  * taking immutable reference after copy: {token:?}");

            // `f` implements `Clone` because all its variables are captured by
            // copy and implement `Clone`:
            implements_clone(&f);

            // `f` implements `Copy` because all its variables are captured by
            // copy and implement `Copy`:
            implements_copy(&f);

            // `f` is copied because it implements `Copy`:
            eh.on_close(f);

            // `f` can still be referenced because it was not moved:
            implements_copy(&f);

            // `token` can still be referenced because it was not moved:
            println!("  * taking immutable reference after copy: {token:?}");

            // Original `f` with its copy of `token` and `token` are dropped
            // here.
        }

        // Copy of `f` is used here:
        eh.close();

        // Copy of `f` together with its copy of `token` is dropped here.
    }

    {
        let mut eh = EventHandler::new();

        {
            let token = NonCloneable;

            // `f` captures `token` by move but still implements `Fn`:
            //   * when `f` is called, immutable reference to `token` is taken
            //     and `token` is still owned by `f` so this allow `Fn` to be
            //     implemented; since only immutable reference to `token` is
            //     used in the `f`'s body, `f` can be called by reference as
            //     well; that is, `f` and `token` are shared with the caller,
            //     not move to it
            //   * without `&` in `&token`, when `f` is called, `f` and `token`
            //     becomes owned by the caller and hence `f` cannot be called
            //     for the second time
            let f = move || {
                let token = &token;

                println!("  * {token:?}");
            };

            // `token` is owned by `f`:
            // println!("  * taking immutable reference after move: {token:?}");  // Error

            // `f` does not implement `Clone` because `token` does not
            // implement `Clone`:
            // implements_clone(&f);

            // `f` does not implement `Copy` because `token` does not implement
            // `Copy`:
            // implements_copy(&f);

            // `f` and `token` are move out of this scope:
            eh.on_poke(f);
        }

        // `f` and `token` are borrowed here:
        eh.poke();

        // `f` and `token` are borrowed here for the second time:
        eh.poke();

        // `f` and `token` are dropped here.
    }

    {
        let mut eh = EventHandler::new();

        {
            let token = Cloneable;

            // `f` captures `token` by move:
            let f = move || {
                let token = &token;

                println!("  * {token:?}");
            };

            // `token` is owned by `f`:
            // println!("  * taking immutable reference after move: {token:?}");  // Error

            // `f` implements `Clone` because all its variables are captured by
            // move and implement `Clone`:
            implements_clone(&f);

            // `f` does not implement `Copy` because `token` does not implement
            // `Copy`:
            // implements_copy(&f);  // Error

            // `f` and `token` are move out of this scope:
            eh.on_poke(f);
        }

        // `f` and `token` are borrowed here:
        eh.poke();

        // `f` and `token` are borrowed here again:
        eh.poke();

        // `f` and `token` are dropped here.
    }

    {
        let mut eh = EventHandler::new();

        {
            let token = Copyable;

            // `f` captures `token` by copy:
            //   * notice the omission of `&` in `token`; since `token`
            //     implements `Copy`, `token` can be copied on every call of
            //     `f`
            let f = move || {
                let token = token;

                println!("  * {token:?}");
            };

            // This is correct because `token` has been captured by copy:
            println!("  * taking immutable reference after copy: {token:?}");

            // `f` implements `Clone` because all its variables are captured by
            // copy and implement `Clone`:
            implements_clone(&f);

            // `f` implements `Copy` because all its variables are captured by
            // copy and implement `Copy`:
            implements_copy(&f);

            // `f` is copied together with its copy of `token`:
            eh.on_poke(f);

            // `f` still can be used since it has been copied:
            implements_copy(&f);

            // `token` still can be used since it has been copied:
            println!("  * taking immutable reference after copy: {token:?}");

            // `f`, its copy of `token`, and `token` are dropped here.
        }

        // Copy of `f` and its copy of `token` are used here, reference to the
        // copy of `f` is taken during the call:
        eh.poke();

        // Copy of `f` and its copy of `token` are used here, reference to the
        // copy of `f` is taken during the call:
        eh.poke();

        // Copy of `f` together with its copy of `token` are dropped here.
    }
}

fn no_capture() {
    println!("No capture demo:");

    let x = 3;
    let y = 5;

    // Closure type can be coerced to `fn()` type as it captures no variables:
    let add: fn(i32, i32) -> i32 = |a, b| a + b;

    println!("  * {} + {} = {}", x, y, add(x, y));
}

fn main() {
    immutable_capture();
    println!("");
    unique_immutable_capture();
    println!("");
    mutable_capture();
    println!("");
    move_capture();
    println!("");
    no_capture();
}

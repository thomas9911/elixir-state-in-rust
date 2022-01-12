pub mod map;

rustler::init!(
    "Elixir.ElixirStateInRust.Map",
    [
        map::new,
        map::new_with_list,
        map::put,
        map::get,
        map::is_empty,
        map::pop,
        map::len,
        map::contains,
        map::clone,
    ],
    load = map::load
);

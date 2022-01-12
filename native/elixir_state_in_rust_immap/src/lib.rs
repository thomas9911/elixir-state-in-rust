pub mod immap;

rustler::init!(
    "Elixir.ElixirStateInRust.ImMap",
    [
        immap::new,
        immap::new_with_list,
        immap::put,
        immap::get,
        immap::is_empty,
        immap::pop,
        immap::len,
        immap::contains,
        immap::clone,
    ],
    load = immap::load
);

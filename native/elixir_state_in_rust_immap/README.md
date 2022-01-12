# NIF for Elixir.ElixirStateInRust.ImMap

## To build the NIF module:

- Your NIF will now build along with your project.

## To load the NIF:

```elixir
defmodule ElixirStateInRust.ImMap do
    use Rustler, otp_app: :elixir_state_in_rust, crate: "elixir_state_in_rust_immap"

    # When your NIF is loaded, it will override this function.
    def add(_a, _b), do: :erlang.nif_error(:nif_not_loaded)
end
```

## Examples

[This](https://github.com/hansihe/NifIo) is a complete example of a NIF written in Rust.

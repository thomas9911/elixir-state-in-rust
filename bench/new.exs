alias ElixirStateInRust.Map, as: RustMap
alias ElixirStateInRust.ImMap, as: ImRustMap

Benchee.run(
  %{
    "new elixir map" => fn input -> Map.new(input) end,
    "new rust map" => fn input -> RustMap.new(input) end,
    "new im rust map" => fn input -> ImRustMap.new(input) end
  },
  inputs: %{
    "Small" => 0..100 |> Enum.map(&to_string/1) |> Enum.zip(2..102)
  }
)

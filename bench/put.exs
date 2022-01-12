alias ElixirStateInRust.Map, as: RustMap
alias ElixirStateInRust.ImMap, as: ImRustMap

elixir_map_reduce = fn input, elixir_map ->
  Enum.reduce(input, elixir_map, fn {k, v}, acc ->
    Map.put(acc, k, v)
  end)
end

rust_map_reduce = fn input, rust_map ->
  Enum.reduce(input, rust_map, fn {k, v}, acc ->
    RustMap.put(acc, k, v)
  end)
end

imrust_map_reduce = fn input, rust_map ->
  Enum.reduce(input, rust_map, fn {k, v}, acc ->
    ImRustMap.put(acc, k, v)
  end)
end

Benchee.run(
  %{
    "put elixir map" => fn input -> elixir_map_reduce.(input, Map.new()) end,
    "put rust map" => fn input -> rust_map_reduce.(input, RustMap.new()) end,
    "put im rust map" => fn input -> imrust_map_reduce.(input, ImRustMap.new()) end
  },
  inputs: %{
    "Small" => 250..300 |> Enum.map(&to_string/1) |> Enum.zip(2..102)
  }
)

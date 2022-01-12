alias ElixirStateInRust.Map, as: RustMap
alias ElixirStateInRust.ImMap, as: ImRustMap

map = 0..100 |> Enum.map(&to_string/1) |> Enum.zip(2..102) |> Map.new()
rust_map = 0..100 |> Enum.map(&to_string/1) |> Enum.zip(2..102) |> RustMap.new()
im_rust_map = 0..100 |> Enum.map(&to_string/1) |> Enum.zip(2..102) |> ImRustMap.new()

Map.get(map, "55") |> IO.inspect()
RustMap.get(rust_map, "55") |> IO.inspect()
ImRustMap.get(im_rust_map, "55") |> IO.inspect()

Benchee.run(%{
  "get elixir map" => fn -> Map.get(map, "55") end,
  "get rust map" => fn -> RustMap.get(rust_map, "55") end,
  "get im rust map" => fn -> ImRustMap.get(im_rust_map, "55") end
})

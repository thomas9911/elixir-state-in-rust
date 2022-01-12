defmodule ElixirStateInRust.ImMapTest do
  use ExUnit.Case
  doctest ElixirStateInRust.ImMap

  alias ElixirStateInRust.ImMap, as: RustMap

  test "works" do
    map = RustMap.new()
    map2 = map |> RustMap.put("stuff", 1)

    assert nil == RustMap.get(map, "stuff")
    assert 1 == RustMap.get(map2, "stuff")
    assert 1 == RustMap.get(map2, :stuff)
    assert nil == RustMap.get(map2, "another")
    assert nil == RustMap.get(map2, :another)
  end

  test "new with string key list" do
    map = RustMap.new([{"test", 1}, {"stuff", 2}])

    assert 2 == RustMap.get(map, "stuff")
    assert 1 == RustMap.get(map, "test")
  end

  test "new with keylist" do
    map = RustMap.new(test: 1, stuff: 2)

    assert 2 == RustMap.get(map, "stuff")
    assert 2 == RustMap.get(map, :stuff)
    assert 1 == RustMap.get(map, "test")
  end

  test "new with list error" do
    assert_raise ArgumentError, fn ->
      RustMap.new(["test", {"stuff", 2}])
    end
  end

  test "new with transformation" do
    map = RustMap.new([test: 1, stuff: 2], fn {key, value} -> {key, "#{value}"} end)

    assert "2" == RustMap.get(map, "stuff")
    assert "1" == RustMap.get(map, "test")
  end

  test "enumerator" do
    map = RustMap.new(test: 1, stuff: 2)

    assert %{"test" => 1, "stuff" => 2} = Map.new(map)
    assert map_size(Map.new(map)) == 2
  end

  test "collector" do
    map = Enum.into(%{"stuff" => 5, "test" => 10}, RustMap.new())

    assert 5 == RustMap.get(map, "stuff")
    assert 10 == RustMap.get(map, "test")

    map = Enum.into(%{stuff: 5, test: 10}, RustMap.new())

    assert 5 == RustMap.get(map, "stuff")
    assert 10 == RustMap.get(map, "test")
  end

  test "clone" do
    map = RustMap.new(test: "testing")
    map2 = RustMap.clone(map)
    # are not equal
    assert map != map2

    # do contain the same data
    assert Map.new(map) == Map.new(map2)

    assert map_size(Map.new(map)) == 1
    assert map_size(Map.new(map2)) == 1
  end
end

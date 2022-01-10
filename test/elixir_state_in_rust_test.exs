defmodule ElixirStateInRustTest do
  use ExUnit.Case
  doctest ElixirStateInRust

  test "works" do
    map = ElixirStateInRust.new()
    map |> ElixirStateInRust.put("stuff", 1)

    assert 1 == ElixirStateInRust.get(map, "stuff")
    assert nil == ElixirStateInRust.get(map, "another")
  end

  test "new with string key list" do
    map = ElixirStateInRust.new([{"test", 1}, {"stuff", 2}])

    assert 2 == ElixirStateInRust.get(map, "stuff")
    assert 1 == ElixirStateInRust.get(map, "test")
  end

  test "new with keylist" do
    map = ElixirStateInRust.new(test: 1, stuff: 2)

    assert 2 == ElixirStateInRust.get(map, "stuff")
    assert 1 == ElixirStateInRust.get(map, "test")
  end

  test "new with list error" do
    assert_raise ArgumentError, fn ->
      ElixirStateInRust.new(["test", {"stuff", 2}])
    end
  end

  test "new with transformation" do
    map = ElixirStateInRust.new([test: 1, stuff: 2], fn {key, value} -> {key, value + 1} end)

    assert 3 == ElixirStateInRust.get(map, "stuff")
    assert 2 == ElixirStateInRust.get(map, "test")
  end

  test "enumerator" do
    map = ElixirStateInRust.new(test: 1, stuff: 2)

    assert %{"test" => 1, "stuff" => 2} = Map.new(map)
  end

  test "collector" do
    map = Enum.into(%{"stuff" => 5, "test" => 10}, ElixirStateInRust.new())

    assert 5 == ElixirStateInRust.get(map, "stuff")
    assert 10 == ElixirStateInRust.get(map, "test")

    map = Enum.into(%{stuff: 5, test: 10}, ElixirStateInRust.new())

    assert 5 == ElixirStateInRust.get(map, "stuff")
    assert 10 == ElixirStateInRust.get(map, "test")
  end
end

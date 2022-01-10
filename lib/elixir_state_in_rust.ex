defmodule ElixirStateInRust do
  defstruct [:reference]
  use Rustler, otp_app: :elixir_state_in_rust, crate: "elixir_state_in_rust"

  @opaque t :: %ElixirStateInRust{}

  # When your NIF is loaded, it will override this function.
  def get(_map, _key), do: :erlang.nif_error(:nif_not_loaded)
  def put(_map, _key, _value), do: :erlang.nif_error(:nif_not_loaded)
  def empty?(map), do: _empty(map)
  defp _empty(_map), do: :erlang.nif_error(:nif_not_loaded)
  def pop(_map), do: :erlang.nif_error(:nif_not_loaded)
  def len(_map), do: :erlang.nif_error(:nif_not_loaded)
  def contains(_map, _key), do: :erlang.nif_error(:nif_not_loaded)

  def new(), do: :erlang.nif_error(:nif_not_loaded)
  def new(_list), do: :erlang.nif_error(:nif_not_loaded)

  def new(list, transformation) do
    list
    |> Enum.map(transformation)
    |> new()
  end
end

defimpl Enumerable, for: ElixirStateInRust do
  def reduce(_list, {:halt, acc}, _fun), do: {:halted, acc}
  def reduce(list, {:suspend, acc}, fun), do: {:suspended, acc, &reduce(list, &1, fun)}

  def reduce(data, {:cont, acc}, fun) do
    if ElixirStateInRust.empty?(data) do
      {:done, acc}
    else
      head = ElixirStateInRust.pop(data)
      reduce(data, fun.(head, acc), fun)
    end
  end

  def count(map), do: {:ok, ElixirStateInRust.len(map)}
  def member?(map, key), do: {:ok, ElixirStateInRust.contains(map, key)}
  def slice(_list), do: {:error, __MODULE__}
end

defimpl Collectable, for: ElixirStateInRust do
  def into(map) do
    fun = fn
      acc, {:cont, {key, value}} ->
        ElixirStateInRust.put(acc, key, value)

      acc, :done ->
        acc

      _, :halt ->
        :ok
    end

    {map, fun}
  end
end

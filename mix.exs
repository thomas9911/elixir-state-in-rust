defmodule ElixirStateInRust.MixProject do
  use Mix.Project

  def project do
    [
      app: :elixir_state_in_rust,
      version: "0.1.0",
      elixir: "~> 1.12",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.23.0"},
      {:ex_doc, "~> 0.24", only: :dev, runtime: false},
      {:benchee, "~> 1.0", only: :bench}
    ]
  end
end

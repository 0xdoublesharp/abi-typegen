defmodule AbiTypegenElixirE2E.MixProject do
  use Mix.Project

  def project do
    [
      app: :abi_typegen_elixir_e2e,
      version: "0.1.0",
      elixir: "~> 1.15",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:ethers, "== 0.8.0"},
      # Ethers.Signer.Local uses this optional dependency for the Anvil write test.
      {:ex_secp256k1, "== 0.8.0", only: :test},
      # Override idna 7.1.0 to 8.0+ to resolve EEF-CVE-2026-117678.
      {:idna, "~> 8.0", override: true}
    ]
  end
end

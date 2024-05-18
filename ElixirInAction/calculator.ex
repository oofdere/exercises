defmodule Calculator do
  def start(init \\ 0) do
    spawn(fn ->
      loop(init)
    end)
  end

  defp loop(i) do
    new_state =
      receive do
        {:value, caller} ->
          send(caller, i)
          i

        {:add, n} ->
          i + n

        {:sub, n} ->
          i - n

        {:mul, n} ->
          i * n

        {:div, n} ->
          i / n

        _ ->
          n
      end

    loop(new_state)
  end

  def value(pid) do
    send(pid, {:value, self()})

    receive do
      i ->
        IO.puts(i)
        i
    end
  end

  def add(pid, n) do
    send(pid, {:add, n})
  end

  def sub(pid, n) do
    send(pid, {:sub, n})
  end

  def mul(pid, n) do
    send(pid, {:mul, n})
  end

  def div(pid, n) do
    send(pid, {:div, n})
  end
end

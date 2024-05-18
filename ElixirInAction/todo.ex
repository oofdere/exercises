defmodule TodoList do
  defstruct next_id: 1, entries: %{}

  def new(entries \\ []) do
    Enum.reduce(
      entries,
      %TodoList{},
      fn entry, todo_list -> add_entry(todo_list, entry) end
    )
  end

  def add_entry(todo_list, entry) do
    entry = Map.put(entry, :id, todo_list.next_id)

    new_entries =
      Map.put(
        todo_list.entries,
        todo_list.next_id,
        entry
      )

    %TodoList{todo_list | entries: new_entries, next_id: todo_list.next_id + 1}
  end

  def entries(todo_list, date) do
    todo_list.entries
    |> Map.values()
    |> Enum.filter(fn entry -> entry.date == date end)
  end

  def update_entry(todo_list, entry_id, updater_fun) do
    case Map.fetch(todo_list.entries, entry_id) do
      :error ->
        todo_list

      {:ok, old_entry} ->
        new_entry = updater_fun.(old_entry)
        new_entries = Map.put(todo_list.entries, new_entry.id, new_entry)
        %TodoList{todo_list | entries: new_entries}
    end
  end

  def delete_entry(todo_list, entry_id) do
    %TodoList{
      todo_list
      | entries: Map.delete(todo_list.entries, entry_id)
    }
  end
end

defmodule TodoList.CsvImporter do
  def import(filename) do
    File.stream!(filename, :line)
    |> Stream.map(&String.trim_trailing/1)
    |> Stream.map(&String.split(&1, ","))
    |> Enum.map(fn [d, t] -> %{date: Date.from_iso8601!(d), title: t} end)
    |> TodoList.new()
  end
end

defmodule TodoServer do
  def start(list \\ TodoList.new()) do
    spawn(fn -> loop(list) end)
  end

  defp loop(list) do
    upd =
      receive do
        msg -> op(list, msg)
      end

    loop(upd)
  end

  defp op(list, {:add_entry, entry}) do
    TodoList.add_entry(list, entry)
  end

  defp op(list, {:delete_entry, id}) do
    TodoList.delete_entry(list, id)
  end

  defp op(list, {:update_entry, id, fun}) do
    TodoList.update_entry(list, id, fun)
  end

  defp op(list, {:entries, date, caller}) do
    send(caller, TodoList.entries(list, date))
    list
  end

  defp op(list, _) do
    IO.puts("Invalid operation.")
    list
  end

  def add_entry(pid, entry) do
    send(pid, {:add_entry, entry})
  end

  def delete_entry(pid, id) do
    send(pid, {:delete_entry, id})
  end

  def update_entry(pid, id, fun) do
    send(pid, {:add_entry, id, fun})
  end

  def entries(pid, date) do
    send(pid, {:entries, date, self()})

    receive do
      m -> m
    end
  end
end

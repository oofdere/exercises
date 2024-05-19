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
  use GenServer

  def init(entries \\ []) do
    {:ok, TodoList.new(entries)}
  end

  def start(entries \\ []) do
    GenServer.start(TodoServer, entries)
  end

  def handle_cast({:add_entry, entry}, list) do
    {:noreply, TodoList.add_entry(list, entry)}
  end

  def handle_cast({:delete_entry, id}, list) do
    {:noreply, TodoList.delete_entry(list, id)}
  end

  def handle_cast({:update_entry, id, fun}, list) do
    {:noreply, TodoList.update_entry(list, id, fun)}
  end

  def handle_call({:entries, date}, _caller, list) do
    {:reply, TodoList.entries(list, date), list}
  end

  def add_entry(pid, entry) do
    GenServer.cast(pid, {:add_entry, entry})
  end

  def delete_entry(pid, id) do
    GenServer.cast(pid, {:delete_entry, id})
  end

  def update_entry(pid, id, fun) do
    GenServer.cast(pid, {:add_entry, id, fun})
  end

  def entries(pid, date) do
    GenServer.call(pid, {:entries, date})
  end
end

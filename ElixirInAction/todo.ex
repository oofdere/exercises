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
  def init() do
    TodoList.new()
  end

  def start() do
    ServerProcess.start(TodoServer)
  end

  def handle_cast({:add_entry, entry}, list) do
    TodoList.add_entry(list, entry)
  end

  def handle_cast({:delete_entry, id}, list) do
    TodoList.delete_entry(list, id)
  end

  def handle_cast({:update_entry, id, fun}, list) do
    TodoList.update_entry(list, id, fun)
  end

  def handle_call({:entries, date}, list) do
    {TodoList.entries(list, date), list}
  end

  def add_entry(pid, entry) do
    ServerProcess.cast(pid, {:add_entry, entry})
  end

  def delete_entry(pid, id) do
    ServerProcess.cast(pid, {:delete_entry, id})
  end

  def update_entry(pid, id, fun) do
    ServerProcess.cast(pid, {:add_entry, id, fun})
  end

  def entries(pid, date) do
    ServerProcess.call(pid, {:entries, date})
  end
end

defmodule ServerProcess do
  def start(callback_module) do
    spawn(fn ->
      initial_state = callback_module.init()
      loop(callback_module, initial_state)
    end)
  end

  def loop(callback_module, current_state) do
    receive do
      {:call, request, caller} ->
        IO.puts("handling call")

        {response, new_state} =
          callback_module.handle_call(
            request,
            current_state
          )

        send(caller, {:response, response})

        loop(callback_module, new_state)

      {:cast, request} ->
        IO.puts("handling cast")

        new_state =
          callback_module.handle_cast(
            request,
            current_state
          )

        IO.inspect(new_state)

        loop(callback_module, new_state)
    end
  end

  def call(server_pid, request) do
    send(server_pid, {:call, request, self()})

    receive do
      {:response, response} ->
        response
    end
  end

  def cast(server_pid, request) do
    send(server_pid, {:cast, request})
  end
end

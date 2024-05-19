defmodule Todo.Server do
  use GenServer

  def init(entries \\ []) do
    {:ok, Todo.List.new(entries)}
  end

  def start(entries \\ []) do
    GenServer.start(Todo.Server, entries)
  end

  def handle_cast({:add_entry, entry}, list) do
    {:noreply, Todo.List.add_entry(list, entry)}
  end

  def handle_cast({:delete_entry, id}, list) do
    {:noreply, Todo.List.delete_entry(list, id)}
  end

  def handle_cast({:update_entry, id, fun}, list) do
    {:noreply, Todo.List.update_entry(list, id, fun)}
  end

  def handle_call({:entries, date}, _caller, list) do
    {:reply, Todo.List.entries(list, date), list}
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

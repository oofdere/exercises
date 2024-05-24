defmodule Todo.Server do
  use GenServer

  def init(list_name) do
    IO.puts("starting to-do server")
    {:ok, {list_name, nil}, {:continue, :init}}
  end

  def handle_continue(:init, {name, nil}) do
    todo_list = Todo.Database.get(name) || Todo.List.new()
    {:noreply, {name, todo_list}}
  end

  def start_link(list_name) do
    GenServer.start_link(Todo.Server, list_name)
  end

  def handle_cast({:add_entry, entry}, {name, list}) do
    new_list = Todo.List.add_entry(list, entry)
    Todo.Database.store(name, new_list)
    {:noreply, {name, new_list}}
  end

  def handle_cast({:delete_entry, id}, {name, list}) do
    new_list = Todo.List.delete_entry(list, id)
    Todo.Database.store(name, new_list)
    {:noreply, {name, new_list}}
  end

  def handle_cast({:update_entry, id, fun}, {name, list}) do
    new_list = Todo.List.update_entry(list, id, fun)
    Todo.Database.store(name, new_list)
    {:noreply, {name, new_list}}
  end

  def handle_call({:entries, date}, _caller, {name, list}) do
    {:reply, Todo.List.entries(list, date), {name, list}}
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

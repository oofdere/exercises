defmodule Todo.Cache do
  use GenServer

  def init(_) do
    Todo.Database.start()
    {:ok, %{}}
  end

  def handle_call({:server_process, list_name}, _, todo_servers) do
    case Map.fetch(todo_servers, list_name) do
      {:ok, todo_server} ->
        {:reply, todo_server, todo_servers}

      :error ->
        {:ok, new_server} = Todo.Server.start(list_name)

        {
          :reply,
          new_server,
          Map.put(todo_servers, list_name, new_server)
        }
    end
  end

  def start do
    GenServer.start(__MODULE__, nil)
  end

  def server_process(cache_pid, list_name) do
    GenServer.call(cache_pid, {:server_process, list_name})
  end
end

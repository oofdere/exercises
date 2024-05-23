defmodule Todo.DatabaseWorker do
  use GenServer

  # just a wrapper for initializing a worker
  def start(path) do
    GenServer.start(__MODULE__, path)
  end

  # GenServer calls this when starting up, passing in the second argument of GenServer.start
  def init(path) do
    IO.inspect("#{inspect(self())}: initializing")
    # path in this tuple is basically the worker's perisitent state
    {:ok, path}
  end

  def store(pid, key, data) do
    GenServer.cast(pid, {:store, key, data})
  end

  def get(pid, key) do
    GenServer.call(pid, {:get, key})
  end

  def handle_cast({:store, key, data}, state) do
    IO.inspect("#{inspect(self())}: storing #{inspect(key)}")

    spawn(fn ->
      key
      |> file_name(state)
      |> File.write!(:erlang.term_to_binary(data))
    end)

    {:noreply, state}
  end

  def handle_call({:get, key}, caller, state) do
    IO.inspect("#{inspect(self())}: loading #{inspect(key)}")

    spawn(fn ->
      data =
        case File.read(file_name(key, state)) do
          {:ok, contents} -> :erlang.binary_to_term(contents)
          _ -> nil
        end

      GenServer.reply(caller, data)
    end)

    {:noreply, state}
  end

  defp file_name(key, path) do
    Path.join(path, to_string(key))
  end
end

defmodule Todo.Database do
  alias Todo.DatabaseWorker
  use GenServer

  @db_folder "./persist"

  def start do
    GenServer.start(__MODULE__, nil, name: __MODULE__)
  end

  def store(key, data) do
    GenServer.cast(__MODULE__, {:store, key, data})
  end

  def get(key) do
    GenServer.call(__MODULE__, {:get, key})
  end

  def init(_) do
    File.mkdir_p!(@db_folder)
    # start workers, put them in a map with number keys 0..2
    workers = %{
      0 => spawn(),
      1 => spawn(),
      2 => spawn()
    }

    {:ok, workers}
  end

  defp spawn do
    {:ok, pid} = DatabaseWorker.start(@db_folder)
    pid
  end

  def handle_cast({:store, key, data}, workers) do
    # cast to worker
    choose_worker(key)
    |> (&Map.fetch!(workers, &1)).()
    |> DatabaseWorker.store(key, data)

    {:noreply, workers}
  end

  def handle_call({:get, key}, caller, workers) do
    spawn(fn ->
      # call worker
      data =
        choose_worker(key)
        |> IO.inspect()
        |> (&Map.fetch!(workers, &1)).()
        |> IO.inspect()
        |> DatabaseWorker.get(key)

      GenServer.reply(caller, data)
    end)

    {:noreply, workers}
  end

  def choose_worker(key) do
    :erlang.phash2(key, 3)
  end
end

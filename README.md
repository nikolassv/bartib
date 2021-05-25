# Bartib

Bartib is a time tracker for the command line. It safes a journal of all tracked tasks in a plaintext file.

## Build

Build it with cargo:

```
cargo build --release
```

## The `.bartib` file

Bartib safes a journal of all tracked tasks in a plaintext file. The file can either be specified via the `-f / --file` cli option or as a `BARTIB_FILE` environment variable.  

## Commands

### Help

Print help informations:

```
bartib -h
```

### Start a new task

Start a new task with a short description and an associated project:

```
bartib start -p "The name of the associated project" -d "A description of the task"
```

This will also stop all currently running tasks.

### Stop a running task

Stops the currently running task:

```
bartib stop
```

### List all currently running tasks

```
bartib current
```

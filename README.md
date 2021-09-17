# Bartib

Bartib is a time tracker for the command line. It safes a journal of all tracked activities in a plaintext file.

## Build

Build it with cargo:

```
cargo build --release --bin bartib
```

## The journal file

Bartib safes a journal of all tracked activities in a plaintext file. The file can either be specified via the `-f / --file` cli option or as a `BARTIB_FILE` environment variable.  

## Commands

### Help

Print help information:

```
bartib -h
```

### Start a new activity

Start a new activity with a short description and an associated project:

```
bartib start -p "The name of the associated project" -d "A description of the activity"
```

All currently tracked activites will be stopped. If the specified file does not exist yet it will be created.

The `-t/--time` option specifies at which time of the current day the new activity starts (and any currently running activity stops):

```
bartib start -p "The name of the associated project" -d "A description of the activity" -t 13:45
```

### Stop a running activity

Stops the currently running activity:

```
bartib stop
```

The `-t/--time` option specifies at which time of the current day the activities stop.

### Continue the last activity

```
bartib continue [-p "Another project"] [-d "Another description"]
```

This continues the last activity. If an activity is currently tracked, bartib stops and restarts this activity. The associated project and description may be overwritten by setting a `-p / --project` or `-d / --description` option.

The `-t/--time` option specifies at which time of the current day the  activity (re-)starts.

### List activities

All activities:

```
bartib list
```

Do not group activities by date:

```
bartib list --no_grouping
```

List activities in a given time range:

```
bartib list --from 2021-03-01 --to 2021-11-01
```

List activities on a given day:

```
bartib list --date 2021-05-17
```

List activities of special days:

```
bartib list --today
bartib list --yesterday
```

### List all currently running activities

```
bartib current
```

### Create a report

This will create a report of how much time has been spent on which projects and activities:

```
bartib report
```

The `report` subcommand accepts most of the arguments that the `list` subcommand accepts:

```
bartib report --today
bartib report --yesterday
bartib report --from 2021-09-01 --to 2021-09-05
bartib report --date 2021-09-03
```

### Edit activities

To change tracked activities, just open the file with your activities log in any text editor. To facilitate this, bartib offers the `edit` subcommand:

```
bartib edit
```

This will open your log in the editor you have defined in your `EDITOR` environment variable. Alternatively you can specify the editor command via the `-e/--editor` option:

```
bartib edit -e vim
```

### Show last activity

```
bartib last
```

### List all projects

This command lists all projects for which an activity has ever been logged:

```
bartib projects
```

This is especially useful for autocompletion. For example, adding this line to the `.bashrc` enables autocompletion for project names:

```
complete -W "$(bartib projects)" bartib
```
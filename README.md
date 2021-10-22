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

### Tracking activities

#### Start a new activity

Start a new activity with a short description and an associated project:

```
bartib start -p "The name of the associated project" -d "A description of the activity"
```

All currently tracked activites will be stopped. If the specified file does not exist yet it will be created.

The `-t/--time` option specifies at which time of the current day the new activity starts (and any currently running activity stops):

```
bartib start -p "The name of the associated project" -d "A description of the activity" -t 13:45
```

#### Stop a running activity

Stops the currently running activity:

```
bartib stop
```

The `-t/--time` option specifies at which time of the current day the activities stop.

#### Continue a recent activity

Instead of typing a project and a description for each new activity, it is possible to continue a recent activity. The `last` subcommand prints a list of recently 
used projects and descriptions. Per default it prints 10 lines. The `-n/--number` option may be used to print a longer list.

```
bartib last [-n 25]
```

In the list each activity will be asigned an index. This index can be used with the `continue` subcommand to restart the selected activitiy. The description
and or the project of this activity can be overwritten for the new activity with the help of the `-d/--description` respectively the `-p/--project` argument.
The `-t/--time` option may be used to specify at which time of the current day the activity should be restarted. If an activity is currently tracked, bartib 
stops it at the time the new activity starts.

```
bartib continue [3] [-p "Another project"] [-d "Another description"] [-t 8:45]
```

The default value for the index parameter is `0` which always points to the most recently tracked activity. Therefore `bartib continue` without any parameters
or options may be used to continue the most recently tracked activity.

#### Cancel all running activities

This command cancels all running activities by deleting their entry in the activity log:

```
bartib cancel
```

### Reporting and listing activities

#### Create a report

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

#### List activities

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

### Miscellaneous commands


#### Edit activities

To change tracked activities, just open the file with your activities log in any text editor. To facilitate this, bartib offers the `edit` subcommand:

```
bartib edit
```

This will open your log in the editor you have defined in your `EDITOR` environment variable. Alternatively you can specify the editor command via the `-e/--editor` option:

```
bartib edit -e vim
```


#### List all currently running activities

```
bartib current
```


#### List all projects

This command lists all projects for which an activity has ever been logged:

```
bartib projects
```

This is especially useful for autocompletion. For example, adding this line to the `.bashrc` enables autocompletion for project names:

```
complete -W "$(bartib projects)" bartib
```
# Bartib

Bartib is an easy to use time tracking tool for the command line. It safes a journal of all tracked activities in a plaintext file and allows you to create flexible reports.

## Tutorial

Alice is not chasing white rabbits any more. She has a real job now with real clients and project managers. Therefore, she has to keep track of how she uses the hours of her working day. See how she uses Bartib and learn how you can use too.

At 8:00 in the morning, Alices arives at the office. She got an email from her project manager who asks her to start working right away on _Urgent Task X_ on _Important Project A_. So Alice types on the command line:

```
~ $ bartib start -d "Urgent Task X" -p "Important Project"
```

And Bartib confirms:

```
Started activity: "Urgent Task X" (Important Project) at 2021-10-29 08:00
```

At 8:42 one of her colleagues drops by and they decide to have a coffee. As she cannot bill this time to her customers, Alice stops the running activity in Bartib:

```
~ $ bartib stop
Stopped activity: "Urgent Task X" (Important Project) started at 2021-10-29 08:43 (43m)
```

Almost 20 minutes later she is back at her desks and continues work:

```
~ $ bartib continue
Started activity: "Urgent Task X" (Important Project) at 2021-10-29 09:01
``` 

At 10:13 another email arrives: _Urgent Task X_ has to wait! Now _More Urgent Task Y_ from _Just Another Project B_ has to be carried out immediately.

Alices types:

```
~ $ bartib start -d "More Urgent Task Y" -p "Just Another Project B"
Stopped activity: "Urgent Task X" (Important Project) started at 2021-10-29 09:01 (1h 12m)
Started activity: "More Urgent Task Y" (Just Another Project B) at 2021-10-29 10:13
```

See how Bartib just stops the running activity when another one starts? No need to stop it manually.

It was a productive morning. After _More Urgent Task Y_ Alice worked on other projects and other taks, but now it is time for lunch and Alice lets Bartib list all the activities she tracked today until now:

```
~ $ bartib list --today

Started Stopped Description        Project                Duration 
08:00   08:43   Urgent Task X      Important Project         43m      
09:01   10:13   Urgent Task X      Important Project      1h 12m      
10:13   10:35   More Urgent Task Y Just Another Project B    22m      
10:35   10:53   Urgent Task X      Important Project         18m      
10:53   11:45   Simple Task Z      Less Important Project    52m       
11:45   12:34   Boring Task XY     Internal Project C        49m    
```

After her lunch brake Alice wants to continue work on _More Urgent Task Y_. Instead of typing the task description and the project name again, she asks Bartib for a list of all the tasks she has recently worked on:

```
~ $ bartib last

 #  Description        Project                
[3] More Urgent Task Y Just Another Project B 
[2] Urgent Task X      Important Project      
[1] Simple Task Z      Less Important Project 
[0] Boring Task XY     Internal Project C 
```

And she instructs Bartib to continue task #3:

```
~ $ bartib continue 3
Started activity: "More Urgent Task Y" (Just Another Project B) at 2021-10-29 12:52
```

A long working day continues. A quarter past seven it is finally time to go home. Alice stops her last activity and asks Bartib for a report:

```
~ $ bartib report --today

Important Project................................. 2h 43m
    Another Task xyz..............................    15m
    Important Call with the Client................    35m
    Urgent Task X................................. 1h 53m

Internal Project C................................ 4h 30m
    Another Meeting...............................    45m
    Boring Task XY................................ 1h 15m
    Long Meeting with Everyone from the Department 2h 30m

Just Another Project B............................    45m
    More Urgent Task Y............................    45m

Less Important Project............................ 2h 27m
    Simple Task No. 5............................. 1h 35m
    Simple Task Z.................................    52m

Total............................................. 9h 25m
```

Another exciting day at the company for Alice! And thanks to Bartib tracking her time was breeze.


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

The `report` subcommand accepts several arguments to refine the selection of activities:

```
bartib report --today
bartib report --yesterday
bartib report --from 2021-09-01 --to 2021-09-05
bartib report --date 2021-09-03
bartib report --project "The most exciting project"
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

The `list` subcommand accepts several arguments to refine the selection of activities:

```
bartib list --today
bartib list --yesterday
bartib list --from 2021-09-01 --to 2021-09-05
bartib list --date 2021-09-03
bartib list --project "The most exciting project"
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

#### Check bartib file

Checks the bartib file for lines that it can not parse as activities:

```
bartib check
```
# Bartib

![Illustration of the White Rabbit from Alice in Wonderland](misc/white-rabbit.png "Oh dear! Oh dear! I shall be too late")

Bartib is an easy to use time tracking tool for the command line. It saves a log of all tracked activities as a plaintext file and allows you to create flexible reports.

[![Crates info](https://img.shields.io/crates/v/bartib.svg)](https://crates.io/crates/bartib)
[![License: GPL](https://img.shields.io/badge/license-GPL-blue)](LICENSE)
[![Number of Stars](https://img.shields.io/github/stars/nikolassv/bartib.svg?style=flat&logo=github&colorB=green&label=stars)](https://github.com/nikolassv/bartib/stargazers)
![Rust](https://img.shields.io/github/languages/top/nikolassv/bartib?color=orange)
[![Tests](https://github.com/nikolassv/bartib/actions/workflows/test.yml/badge.svg)](https://github.com/nikolassv/bartib/actions/workflows/test.yml)

## Contents

- [Bartib](#bartib)
  - [Contents](#contents)
  - [Tutorial](#tutorial)
  - [How To ...](#how-to-)
    - [How to install Bartib](#how-to-install-bartib)
      - [Download an executable](#download-an-executable)
      - [With Cargo](#with-cargo)
      - [From the AUR (Arch Linux)](#from-the-aur-arch-linux)
      - [Via homebrew](#via-homebrew)
      - [Via apk (Alpine Linux)](#via-apk-alpine-linux)
    - [How to build Bartib](#how-to-build-bartib)
    - [How to define in which file to save the log of your activities](#how-to-define-in-which-file-to-save-the-log-of-your-activities)
    - [How to edit or delete tracked activities](#how-to-edit-or-delete-tracked-activities)
    - [How to activate auto completion](#how-to-activate-auto-completion)
  - [Command overview](#command-overview)
    - [The essentials](#the-essentials)
    - [Getting Help](#getting-help)
    - [Tracking activities](#tracking-activities)
    - [Reporting and listing activities](#reporting-and-listing-activities)
    - [Edit activities](#edit-activities)
    - [Doing other stuff](#doing-other-stuff)

## Tutorial

Alice is not chasing white rabbits any more. She has a real job now with real clients and project managers. Therefore, she has to keep track of how she uses the hours of her working day. See how Alice uses Bartib for this and learn how you can use it, too.

At 8:00 a.m. Alice arrives at the office. She got an email from her project manager who asks her to start working right away on _Urgent Task X_ from _Important Project A_. So Alice types at the command line:

```console
alice@work: ~ $ bartib start -d "Urgent Task X" -p "Important Project"
```

And Bartib confirms:

```
Started activity: "Urgent Task X" (Important Project) at 2021-10-29 08:00
```

At 8:43 one of her colleagues drops by and they decide to have a coffee. As she cannot bill this time to her clients, Alice stops the running activity in Bartib:

```console
alice@work: ~ $ bartib stop
Stopped activity: "Urgent Task X" (Important Project) started at 2021-10-29 08:00 (43m)
```

Almost 10 minutes later she is back at her desk and continues work:

```console
alice@work: ~ $ bartib continue
Started activity: "Urgent Task X" (Important Project) at 2021-10-29 08:51
``` 

At 10:13 another email arrives: _Urgent Task X_ has to wait! Now _More Urgent Task Y_ from _Just Another Project B_ has to be carried out immediately.

Alice types:

```console
alice@work: ~ $ bartib start -d "More Urgent Task Y" -p "Just Another Project B"
Stopped activity: "Urgent Task X" (Important Project) started at 2021-10-29 09:01 (1h 12m)
Started activity: "More Urgent Task Y" (Just Another Project B) at 2021-10-29 10:13
```

See how Bartib just stops the running activity when another one starts? No need to stop it manually.

It is a productive morning. After _More Urgent Task Y_ Alice works on other projects and other tasks, but now it is time for lunch and Alice lets Bartib list all the activities she has tracked today until now:

```console
alice@work: ~ $ bartib list --today

Started Stopped Description        Project                Duration 
08:00   08:43   Urgent Task X      Important Project         43m      
08:51   10:13   Urgent Task X      Important Project      1h 22m      
10:13   10:35   More Urgent Task Y Just Another Project B    22m      
10:35   10:53   Urgent Task X      Important Project         18m      
10:53   11:45   Simple Task Z      Less Important Project    52m       
11:45   12:34   Boring Task XY     Internal Project C        49m    
```

After her lunch break Alice wants to continue work on _More Urgent Task Y_. Instead of typing the task description and the project name again, she asks Bartib for a list of all the tasks she has recently worked on:

```console
alice@work: ~ $ bartib last

 #  Description        Project                
[3] More Urgent Task Y Just Another Project B 
[2] Urgent Task X      Important Project      
[1] Simple Task Z      Less Important Project 
[0] Boring Task XY     Internal Project C 
```

And she instructs Bartib to continue task #3:

```console
alice@work: ~ $ bartib continue 3
Started activity: "More Urgent Task Y" (Just Another Project B) at 2021-10-29 12:52
```

An exciting day at work continues. As it is a Friday Alice decides to already leave work at shortly after seven. She stops her latest activity and asks Bartib for a report:

```console
alice@work: ~ $ bartib report --today

Important Project.................................  2h 43m
    Another Task xyz..............................     15m
    Important Call with the Client................     35m
    Urgent Task X.................................  1h 53m

Internal Project C................................  4h 30m
    Another Meeting...............................     45m
    Boring Task XY................................  1h 15m
    Long Meeting with Everyone from the Department  2h 30m

Just Another Project B............................     45m
    More Urgent Task Y............................     45m

Less Important Project............................  2h 27m
    Simple Task No. 5.............................  1h 35m
    Simple Task Z.................................     52m

Total............................................. 10h 25m
```

Alice is happy. This was just another great day at the company and thanks to Bartib tracking her time was a breeze.

Do you want to be as happy as Alice? Use Bartib!

## How To ...

### How to install Bartib

#### Download an executable

Simply download a suitable executable from <https://github.com/nikolassv/bartib/releases> and copy it in some directory that is listed in your `PATH` (e.g. ~/bin).

#### With Cargo

You may also use cargo to install Bartib from crates.io:

```bash
cargo install bartib
```

#### From the AUR (Arch Linux)

```sh
yay -S bartib
```

#### Via homebrew

```sh
brew install bartib
```

#### Via apk (Alpine Linux)

```sh
apk add bartib
```

(Currently bartib is only available in the [testing repository](https://pkgs.alpinelinux.org/packages?name=bartib))

#### General Packaging Status

[![Packaging status](https://repology.org/badge/vertical-allrepos/bartib.svg)](https://repology.org/project/bartib/versions)

### How to build Bartib

Bartib is written in rust. You may build it yourself with the help of cargo. Just clone this repository and execute the `cargo build` command in its main directory:

```bash
cargo build --release
```

### How to define in which file to save the log of your activities

You may either specify the absolute path to your log as an extra parameter (`--file` or `-f`) to your bartib command:

```bash
bartib -f /home/username/activities.bartib report
```

Or you may set the environment variable `BARTIB_FILE` to the path of your log. Just add this line to your `.profile` file:

```bash
export BARTIB_FILE="/home/username/activities.bartib"
```

If the specified log file does not exist yet Bartib creates it.

### How to edit or delete tracked activities

Just open your activity log in your favorite text editor to edit or delete former activities. You may even add new activities manually in this file. The format is self explanatory.

Bartib even offers the `bartib edit` command which opens the log in the editor defined by your `EDITOR` environment variable. If you are unsure whether your edits are readable by bartib, use the `bartib check` command. It will inform you about any parsing errors.

### How to activate auto completion

Bartib offers a simple auto completion for project names. This saves you from typing out long project names each time you start a new task. Just source the script [misc/bartibCompletion.sh](misc/bartibCompletion.sh) in your `.bashrc` to enable it.

For fish users, add the [misc/bartib.fish](misc/bartib.fish) to either the `~/.config/fish/completions/` or `~/.local/share/fish/vendor_completions.d/` directory.
Currently, you must set the `BARTIB_FILE` in your fish shell for the project and description names completions.

## Command overview

All these commands require that you have set the `BARTIB_FILE` environment variable to the file path of your activity log. Otherwise they require an additional `-f/--file` parameter between `bartib` and the subcommand (see above: [How to define in which file to save the log of your activities](#how-to-define-in-which-file-to-save-the-log-of-your-activities)).

### The essentials

```bash
bartib -h    # get help
bartib start -p "name of the project" -d "description of the activity"    # start a new activity
bartib stop    # stop an activity
bartib list --today    # list all activities of the current day
bartib report --today    # create a report for today
```

### Getting Help

```bash
bartib -h    # Print a concise help
bartib start -h    # Print a help for any subcommand
```

### Tracking activities

```bash
bartib start -p "The name of the associated project" -d "A description of the activity"    # Start a new activity with a short description and an associated project
bartib start -p "The name of the associated project" -d "A description of the activity" -t 13:45    # Start a new activity at a given time

bartib stop    # Stop the currently running activity
bartib stop -t 14:00    # Stop the currently running activity at a given time

bartib last    # Print a list of the ten most recently used projects and descriptions
bartib last -n 25   # Prints a list of recently used projects and descriptions with more entries

# All numbers used with the following commands refer to the indexes in the list created with `bartib last`
bartib continue 5    # Start an activity with a recently used project and description
bartib continue    # Continue the latest activity
bartib continue 3 -d "Another description"    # Continue activity number 3 but overwrite the description
bartib continue 7 -t 8:15    # Continue activity number 7 but have it started at a given time

bartib cancel    # Cancels a running activity by deleting its entry in the activity log
```

### Reporting and listing activities

```bash
bartib report    # create a report of how much time has been spent on which projects and activities
bartib report --today    # create a report for today
bartib report --yesterday    # create a report for yesterday
bartib report --current_week    # create a report for the current week (since monday)
bartib report --last_week    # create a report for the last week
bartib report --date 2021-09-03    # create a report for a given day
bartib report --from 2021-09-01 --to 2021-09-05    # create a report for a given time range
bartib report --project "The most exciting project"    # create a report for a given project
bartib report --project "Maint?nance *"    # use '?' and '*' as wildcards in project names
bartib report --round 15m # rounds the start and end time to the nearest duration. Durations can be in minutes or hours. E.g. 15m or 4h

bartib list    # list all activities grouped by day
bartib list --no_grouping    # list all activities but do not group them by day

bartib list --today    # list todays' activities
bartib list --yesterday    # list yesterdays' activities
bartib list --current_week    # list activities of the current week (since monday)
bartib list --last_week    # list activities of the last week
bartib list --date 2021-09-03    # list activities on a given day
bartib list --from 2021-09-01 --to 2021-09-05    # list activities in a given time range
bartib list --project "The most exciting project"    # list activities for a given project
bartib list --round 15m # rounds the start and end time to the nearest duration. Durations can be in minutes or hours. E.g. 15m or 4h

bartib search "exiting"   # search all descriptions and projects for a specific term
bartib search "e*t?ng"   # use '?' and '*' as wildcards
```

### Edit activities

```bash
bartib change -d "A new description"   # change the description of the current activity
bartib change -p "Another project"   # change the project for the current activity
bartib change -t 8:15   # change the start time of the current activity

bartib edit   # open the activity log in the editor you have defined in your `EDITOR` environment variable
bartib edit -e vim    # open the activity log in a given editor
```

### Doing other stuff

```bash
bartib current    # show currently running activity
bartib projects    # list all projects ever used
bartib projects -c # show current project only

bartib check    # check your activity log for invalid lines
bartib sanity    # check for activities with logical errors (e.g activities with negative duration)
```

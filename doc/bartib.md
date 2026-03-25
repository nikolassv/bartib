# bartib

## NAME

bartib — a simple command-line time tracker

## SYNOPSIS

```
bartib -f FILE SUBCOMMAND [OPTIONS]
```

The file can also be supplied via the `BARTIB_FILE` environment variable instead of the `-f` flag (see **GLOBAL OPTIONS** and **ENVIRONMENT**).

## DESCRIPTION

Bartib records time-tracking activities to a plain text file. Each activity belongs to a project and carries a short description. Activities can be started, stopped, continued, and queried at any time. Reports aggregate tracked time by project and activity.

The activity log is a human-readable text file that can be edited manually. See [bartib-file-format.md](bartib-file-format.md) for a full description of the format.

## GLOBAL OPTIONS

`-f FILE`
: Path to the activity log file. Overrides the `BARTIB_FILE` environment variable. Required unless `BARTIB_FILE` is set.

`-h`, `--help`
: Print a help summary and exit.

`--version`
: Print the version number and exit.

## SUBCOMMANDS

### start

```
bartib start -p PROJECT -d DESCRIPTION [-t TIME]
```

Start a new activity. Any currently running activities are stopped automatically before the new one begins.

**Options**

`-p PROJECT`, `--project PROJECT`
: The project the activity belongs to. Required.

`-d DESCRIPTION`, `--description DESCRIPTION`
: A short description of the activity. Required.

`-t TIME`, `--time TIME`
: Start the activity at the given time instead of now. Format: `HH:MM`.

---

### stop

```
bartib stop [-t TIME]
```

Stop all currently running activities.

**Options**

`-t TIME`, `--time TIME`
: Record the given time as the end time instead of now. Format: `HH:MM`.

---

### continue

```
bartib continue [NUMBER] [-p PROJECT] [-d DESCRIPTION] [-t TIME]
```

Start a new activity reusing the project and description of a recently used activity. The optional `NUMBER` argument refers to the index shown by `bartib last` (default: `0`, i.e. the most recent activity). Any currently running activities are stopped automatically.

**Options**

`NUMBER`
: Index of the activity to continue as listed by `bartib last`. Defaults to `0`.

`-p PROJECT`, `--project PROJECT`
: Override the project name.

`-d DESCRIPTION`, `--description DESCRIPTION`
: Override the description.

`-t TIME`, `--time TIME`
: Start the activity at the given time instead of now. Format: `HH:MM`.

---

### cancel

```
bartib cancel
```

Cancel all currently running activities by removing their entries from the log entirely. Unlike `stop`, no end time is recorded and the entries are deleted.

---

### change

```
bartib change [-p PROJECT] [-d DESCRIPTION] [-t TIME]
```

Modify the currently running activity. All currently running activities are updated. At least one option must be given.

**Options**

`-p PROJECT`, `--project PROJECT`
: Set a new project name.

`-d DESCRIPTION`, `--description DESCRIPTION`
: Set a new description.

`-t TIME`, `--time TIME`
: Set a new start time. Format: `HH:MM`.

---

### current

```
bartib current
```

List all currently running activities (i.e. activities with no end time).

---

### list

```
bartib list [FILTER OPTIONS] [-p PROJECT] [-n NUMBER] [--no_grouping] [--round DURATION]
```

List tracked activities in chronological order, optionally filtered by date or project. By default activities are grouped by day.

**Filter options** (mutually exclusive)

`--today`
: Show only activities from today.

`--yesterday`
: Show only activities from yesterday.

`--current_week`
: Show only activities from the current week (Monday through Sunday).

`--last_week`
: Show only activities from the previous week.

`-d DATE`, `--date DATE`
: Show only activities from the given date. Format: `YYYY-MM-DD`.

`--from FROM_DATE`
: Start of a date range (inclusive). Format: `YYYY-MM-DD`.

`--to TO_DATE`
: End of a date range (inclusive). Format: `YYYY-MM-DD`.

**Other options**

`-p PROJECT`, `--project PROJECT`
: Show only activities belonging to the given project. Supports `?` and `*` wildcards.

`-n NUMBER`, `--number NUMBER`
: Limit output to the most recent NUMBER activities.

`--no_grouping`
: Do not group activities by date.

`--round DURATION`
: Round start and end times to the nearest multiple of DURATION before display. Format: a number followed by `m` (minutes) or `h` (hours), e.g. `15m` or `1h`. Does not modify the log file.

---

### report

```
bartib report [FILTER OPTIONS] [-p PROJECT] [--round DURATION]
```

Print a report of time spent per project and activity. Supports the same filter and round options as `list`.

**Filter options** (mutually exclusive)

`--today`, `--yesterday`, `--current_week`, `--last_week`, `-d DATE`, `--from FROM_DATE`, `--to TO_DATE`
: Same as for `list`.

**Other options**

`-p PROJECT`, `--project PROJECT`
: Restrict the report to the given project. Supports `?` and `*` wildcards.

`--round DURATION`
: Round timestamps before calculating durations. Format: `15m`, `1h`, etc.

---

### status

```
bartib status [-p PROJECT]
```

Show a status overview: the currently running activity, and time totals for today, the current week, and the current month.

**Options**

`-p PROJECT`, `--project PROJECT`
: Restrict totals to the given project.

---

### last

```
bartib last [-n NUMBER]
```

Display a numbered list of recently used project-and-description combinations, ordered by most recently started. The index shown can be passed to `bartib continue`.

**Options**

`-n NUMBER`, `--number NUMBER`
: Maximum number of entries to display. Defaults to `10`.

---

### projects

```
bartib projects [-c] [-n]
```

List all project names that appear in the activity log.

**Options**

`-c`, `--current`
: Show only projects with a currently running activity.

`-n`, `--no-quotes`
: Print project names without surrounding quotes.

---

### search

```
bartib search SEARCH_TERM
```

Search all activity descriptions and project names for the given term. Supports `?` and `*` wildcards.

**Arguments**

`SEARCH_TERM`
: The term to search for. Required.

---

### edit

```
bartib edit [-e EDITOR]
```

Open the activity log in a text editor. Falls back to the `EDITOR` environment variable if `-e` is not given.

**Options**

`-e EDITOR`
: Command used to launch the editor (e.g. `vim`, `nano`).

---

### check

```
bartib check
```

Parse the entire activity log and report any lines that cannot be read. Useful after manual edits.

---

### sanity

```
bartib sanity
```

Check the activity log for logical errors and print a warning for each one found. If no problems are found, prints `No unusual activities.`

Two conditions are checked:

- **Negative duration** — an activity whose end time is before its start time.
- **Overlapping activities** — an activity that starts before another activity has ended.

See [bartib-file-format.md](bartib-file-format.md) for more detail on these checks.

## ENVIRONMENT

`BARTIB_FILE`
: Path to the activity log file. Used when `-f` is not supplied. If neither `-f` nor `BARTIB_FILE` is set, bartib exits with an error.

`EDITOR`
: Default editor command used by `bartib edit` when `-e` is not given.

## FILES

`BARTIB_FILE` (or the value of `-f`)
: The activity log. A plain text file, one activity per line. Created automatically if it does not exist. See [bartib-file-format.md](bartib-file-format.md) for a description of the format.

## EXAMPLES

Start tracking work on a task:

```
bartib start -p "Important Project" -d "Urgent Task X"
```

Stop the running activity:

```
bartib stop
```

Stop at a specific time:

```
bartib stop -t 17:30
```

Continue the most recent activity:

```
bartib continue
```

Continue the third most recent activity with a different description:

```
bartib continue 3 -d "Follow-up work"
```

List today's activities:

```
bartib list --today
```

Report time spent last week:

```
bartib report --last_week
```

Report for a single project, rounding to the nearest 15 minutes:

```
bartib report --current_week -p "Important Project" --round 15m
```

Show overall status:

```
bartib status
```

Search for a term across all activities:

```
bartib search "urgent*"
```

Check the log for parse errors, then for logical errors:

```
bartib check
bartib sanity
```

## SEE ALSO

[bartib-file-format.md](bartib-file-format.md)

## AUTHORS

Nikolas Schmidt-Voigt &lt;nikolas.schmidt-voigt@posteo.de&gt;

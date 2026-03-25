# Bartib File Format

Bartib stores all activities in a plain text file, one activity per line. The file is human-readable and can be edited manually in any text editor.

## Line Structure

Each line represents one activity and follows this structure:

```
START_TIME | PROJECT | DESCRIPTION
```

For a stopped (completed) activity with an end time:

```
START_TIME - END_TIME | PROJECT | DESCRIPTION
```

Fields are separated by ` | ` (space, pipe, space). The description field is optional — a line with only start time and project is valid.

## Timestamp Format

Timestamps use ISO 8601 local time, without timezone information.

**Default (minute precision):**
```
YYYY-MM-DD HH:MM
```

**With `second-precision` compile feature:**
```
YYYY-MM-DD HH:MM:SS
```

## Examples

A currently running activity (no end time):
```
2021-02-16 16:14 | my project | writing documentation
```

A stopped activity:
```
2021-02-16 16:14 - 2021-02-16 18:23 | my project | writing documentation
```

An activity without a description:
```
2021-02-16 16:14 | my project
```

## Special Character Escaping

The pipe character `|` is used as a field delimiter, so it must be escaped inside project names and descriptions. The backslash `\` is the escape character.

| Character | Escaped form |
|-----------|--------------|
| `\|`      | literal pipe `|` in a field value |
| `\\`      | literal backslash `\` in a field value |

Example — a project named `client|work` and a description containing a backslash:
```
2021-02-16 16:14 - 2021-02-16 18:23 | client\|work | path: C:\\Users\\alice
```

This parses to:
- project: `client|work`
- description: `path: C:\Users\alice`

## Handling Precision Mismatches

When the compiled precision of a bartib binary differs from the precision used in the file, bartib handles it gracefully:

- **Minute-precision binary reads second-precision timestamps:** rounds to the nearest minute, prints a warning.
- **Second-precision binary reads minute-precision timestamps:** sets seconds to zero, prints a warning.

This means files can be shared across builds of different precision without data loss.

## Sanity Checks

The `bartib sanity` subcommand checks a file for logical errors and prints a warning for each one found. If no problems are detected it prints `No unusual activities.`

Before checking, all successfully parsed activities are sorted by start time. Lines that cannot be parsed are silently ignored.

Two conditions are flagged:

### Negative duration

An activity whose end time is earlier than its start time. This can happen if a line is edited manually and the timestamps are accidentally swapped or mistyped.

```
2021-02-16 18:23 - 2021-02-16 16:14 | my project | oops, end before start
```

### Overlapping activities

An activity that starts before a previous activity has ended. The check tracks the latest end time seen so far (across all prior activities sorted by start); if the current activity's start is earlier than that, it is reported as an overlap.

```
2021-02-16 09:00 - 2021-02-16 11:00 | project a | first task
2021-02-16 10:30 - 2021-02-16 12:00 | project b | overlaps with first task
```

For each flagged activity, the subcommand prints the description, start time, end time, and line number to help locate and fix the problem.

## Multiple Simultaneously Running Activities

The file format places no restriction on how many activities may be running at the same time — any number of lines without an end time is valid. This situation can arise when the file is edited manually.

Bartib's own commands always prevent accidental accumulation of running activities: `bartib start` and `bartib continue` stop all currently running activities before recording a new one. Likewise, `bartib stop` and `bartib cancel` act on *all* running activities at once, as does `bartib change`.

`bartib current` lists all running activities, so multiple entries will all be shown.

The `bartib sanity` subcommand does **not** flag multiple simultaneous running activities as an error. If that situation is unintentional it can be corrected by manually adding end times to the unwanted entries, or by running `bartib stop` to close all of them at once.

## File Behaviour

- Lines that cannot be parsed are silently skipped when reading activities, but are preserved as-is when the file is written back. This means comments or malformed lines are not lost.
- The file is not sorted; activities appear in the order they were recorded.
- The file path is configured via the `--file` / `-f` command-line flag or the `BARTIB_FILE` environment variable. The file is created automatically if it does not exist.

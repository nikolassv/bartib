# Set the available bartib commands
set -l commands change cancel check continue current edit help last list projects report start stop

# Don't use file completion
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "cancel" -d "cancels all currently running activities"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "change" -d "changes the current activity"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "check" -d "checks file and reports parsing errors"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "continue" -d "continues a previous activity"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "current" -d "lists all currently running activities"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "edit" -d "opens the activity log in an editor"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "help" -d "print the help message"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "last" -d "displays the descriptions and projects of recent activities"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "list" -d "list recent activities"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "projects" -d "list all projects"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "report" -d "reports duration of tracked activities"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "start" -d "starts a new activity"
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -f -a "stop" -d "stops all currently running activities"

# Toplevel options
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -s f -d "bartib task tracking file" -r -F
complete -c bartib -n "not __fish_seen_subcommand_from $commands" -s V -l version -d "print version information" -r

# Every command has the help message
complete -c bartib -n "__fish_use_subcommand" -s h -l help -d "print the help message"
complete -c bartib -n "__fish_seen_subcommand_from $commands" -s h -l help -d "print the help message"

# Functions
function __fish_complete_bartib_projects
    set -l output (bartib projects 2>/dev/null)
    string trim (string replace -a \" " " $output)
end

function __fish_complete_bartib_numbers
    set -l output (bartib last -n 100 2>/dev/null)

    set -l description_and_project (string match -r '(\\e\[4mDescription.*)(\\e\[4mProject.*)' -g $output)
    set -l description_length (math (string length $description_and_project[1]) - 9)
    set -l project_length (math (string length $description_and_project[2]) - 8)
    set -l description_project_re "\[(\d+)\][\p{Space}]*(.{$description_length})[\p{Space}]*(.{$project_length}).*"
    string trim --right (string match -r '(\d+\t.*)' -g (string replace -r $description_project_re '$1\t$3-> $2' $output))
end

function __fish_complete_bartib_descriptions
    set -l output (bartib last -n 100 2>/dev/null)

    set -l description_match (string match -r '(\\e\[4mDescription.*)\\e\[4mProject.*' -g $output)
    set -l description_length (math (string length $description_match) - 8)
    string replace -r (echo "\[(\d)\] ([[:ascii:]]{0,$description_length}).*") '$2' (string match -r '\[\d\].*' $output)
end

function __is_last_argument
    argparse --ignore-unknown 's/short=+' 'l/long=+' -- $argv

    set --local tokens (commandline --current-process --tokenize --cut-at-cursor)
    set --erase tokens[1]

    set -l output 1

    for t in $tokens
        if string match --quiet -- "-*" $t
            set output 1
        end
        if string match --quiet -- "--*" $t
            set output 1
        end

        for s in $_flag_s
            if string match --quiet -- "-$s" $t
                set output 0
            end
        end
        for l in $_flag_l
            if string match --quiet -- "--$l" $t
                set output 0
            end
        end
    end
    return $output
end

# Disable file completion after we have seen a subcommand
complete -c bartib -f


# "change" commands
complete -c bartib -n "__fish_seen_subcommand_from change" -s d -l description -d "the description of the new activity" -f
complete -c bartib -n "__fish_seen_subcommand_from change" -s p -l project -d "the project to which the new activity belongs" -f
complete -c bartib -n "__fish_seen_subcommand_from change" -s t -l time -d "the time for changing the activity status (HH:MM)" -r
complete -c bartib -n "__fish_seen_subcommand_from change; and __fish_seen_argument -s p -l project; and begin; __is_last_argument -s p -l project; or not __fish_seen_argument -s d -l description; end" -a "(__fish_complete_bartib_projects)" -f
complete -c bartib -n "__fish_seen_subcommand_from change; and __fish_seen_argument -s d -l description; and begin; __is_last_argument -s d -l description; or not __fish_seen_argument -s p -l project; end" -a "(__fish_complete_bartib_descriptions)" -f

# "continue" commands
complete -c bartib -n "__fish_seen_subcommand_from continue" -s d -l description -d "the description of the new activity" -f
complete -c bartib -n "__fish_seen_subcommand_from continue" -s p -l project -d "the project to which the new activity belongs" -f
complete -c bartib -n "__fish_seen_subcommand_from continue" -s t -l time -d "the time for changing the activity status (HH:MM)" -r
complete -c bartib -n "__fish_seen_subcommand_from continue; and __fish_seen_argument -s p -l project; and begin; __is_last_argument -s p -l project; or not __fish_seen_argument -s d -l description; end" -a "(__fish_complete_bartib_projects)" -f
complete -c bartib -n "__fish_seen_subcommand_from continue; and __fish_seen_argument -s d -l description; and begin; __is_last_argument -s d -l description; or not __fish_seen_argument -s p -l project; end" -a "(__fish_complete_bartib_descriptions)" -f
complete -c bartib -n "__fish_seen_subcommand_from continue; and not __fish_seen_argument -s d -l description; and not __fish_seen_argument -s p -l project" -a "(__fish_complete_bartib_numbers)" -f

# "edit" commands
complete -c bartib -n "__fish_seen_subcommand_from edit" -s e -l editor -d "the command to start your preferred editor" -r

# "last" commands
complete -c bartib -n "__fish_seen_subcommand_from last" -s n -l number -d "maximum number of lines to display"

# "list" commands
complete -c bartib -n "__fish_seen_subcommand_from list" -s d -l date -d "show activities of a certain date only" -r -f
complete -c bartib -n "__fish_seen_subcommand_from list" -l from -d "begin of date range (inclusive)" -r -f 
complete -c bartib -n "__fish_seen_subcommand_from list" -l to -d "end of date range (inclusive)" -r -f 
complete -c bartib -n "__fish_seen_subcommand_from list" -l current_week -d "show activities of the current week" 
complete -c bartib -n "__fish_seen_subcommand_from list" -l last_week -d "show activities of the last week" 
complete -c bartib -n "__fish_seen_subcommand_from list" -l no_grouping -d "do not group activities by date in list" 
complete -c bartib -n "__fish_seen_subcommand_from list" -l today -d "show activities of the current day" 
complete -c bartib -n "__fish_seen_subcommand_from list" -l yesterday -d "show yesterdays\" activities"
complete -c bartib -n "__fish_seen_subcommand_from list" -s n -l number -d "maximum number of activities to display"
complete -c bartib -n "__fish_seen_subcommand_from list" -s p -l project -d "do list activities for this project only" -f
complete -c bartib -n "__fish_seen_subcommand_from list; and __fish_seen_argument -s p -l project; and begin; __is_last_argument -s p -l project; or not __fish_seen_argument -s d -l description; end" -a "(__fish_complete_bartib_projects)" -f

# "report" commands
complete -c bartib -n "__fish_seen_subcommand_from report" -s d -l date -d "show activities of a certain date only" -r -f
complete -c bartib -n "__fish_seen_subcommand_from report" -l from -d "begin of date range (inclusive)" -r -f
complete -c bartib -n "__fish_seen_subcommand_from report" -l to -d "end of date range (inclusive)" -r -f
complete -c bartib -n "__fish_seen_subcommand_from report" -l current_week -d "show activities of the current week" 
complete -c bartib -n "__fish_seen_subcommand_from report" -l last_week -d "show activities of the last week" 
complete -c bartib -n "__fish_seen_subcommand_from report" -l today -d "show activities of the current day" 
complete -c bartib -n "__fish_seen_subcommand_from report" -l yesterday -d "show yesterdays\" activities" 
complete -c bartib -n "__fish_seen_subcommand_from report" -s n -l number -d "maximum number of activities to display" -f
complete -c bartib -n "__fish_seen_subcommand_from report" -s p -l project -d "do report activities for this project only" -f
complete -c bartib -n "__fish_seen_subcommand_from report; and __fish_seen_argument -s p -l project; and begin; __is_last_argument -s p -l project; or not __fish_seen_argument -s d -l description; end" -a "(__fish_complete_bartib_projects)" -f

# "start" commands
complete -c bartib -n "__fish_seen_subcommand_from start" -s d -l description -d "the description of the new activity" -f
complete -c bartib -n "__fish_seen_subcommand_from start" -s p -l project -d "the project to which the new activity belongs" -f
complete -c bartib -n "__fish_seen_subcommand_from start" -s t -l time -d "the time for changing the activity status (HH:MM)" -r
complete -c bartib -n "__fish_seen_subcommand_from start; and __fish_seen_argument -s p -l project; and begin; __is_last_argument -s p -l project; or not __fish_seen_argument -s d -l description; end" -a "(__fish_complete_bartib_projects)" -f
complete -c bartib -n "__fish_seen_subcommand_from start; and __fish_seen_argument -s d -l description; and begin; __is_last_argument -s d -l description; or not __fish_seen_argument -s p -l project; end" -a "(__fish_complete_bartib_descriptions)" -f

# "stop" commands
complete -c bartib -n "__fish_seen_subcommand_from stop" -s t -l time -d "the time for changing the activity status (HH:MM)"


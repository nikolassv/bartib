#!/bin/bash

function _bartib_completions()
{
    word="${COMP_WORDS[$COMP_CWORD]}"
    prev="${COMP_WORDS[$COMP_CWORD - 1]}"

    case "${prev}" in
        bartib)
            # COMPLETE COMMAND
            words="start stop current continue edit report list help cancel check last projects"
            COMPREPLY=($(compgen -W "$words" -- "$word"))    
        ;;
        -p)
            # COMPLETE PROJECT NAME
            local IFS=$'\n'
            ALL_PROJECTS=$(bartib projects)
            CANDIATE_PROJECTS=($(compgen -W "${ALL_PROJECTS[*]}" -- "$word"))

            if [ ${#CANDIATE_PROJECTS[*]} -eq 0 ]; then
                COMPREPLY=()
            else
                COMPREPLY=($(printf "\"%s\"\n" "${CANDIATE_PROJECTS[@]}"))
            fi
        ;;
        -d)
            # COMPLETE TASK
            local IFS=$'\n'
            # Get selected project and remove surrounding quotes
            local project="${COMP_WORDS[$COMP_CWORD - 2]}"
            project=$(echo "$project" | tr -d '"')
            # Get tasks for that project
            tasks=$(grep "| $project |" /opt/shared/bartib-hist | cut -d'|' -f 3 | sort | uniq | sed 's/ //' | sed -e 's/\(.*\)/"\1"/')

            candidate_tasks=($(compgen -W "${tasks[*]}" -- "$word"))

            if [ ${#candidate_tasks[*]} -eq 0 ]; then
                COMPREPLY=()
            else
                COMPREPLY=($(printf "\"%s\"\n" "${candidate_tasks[@]}"))
            fi
        ;;
        list|report)
            words="--today --help --last_week --current_week --yesterday --project"
            COMPREPLY=($(compgen -W "$words" -- "$word"))
        ;;
        *)
        ;;
    esac
}

complete -F _bartib_completions bartib

#!/bin/bash

_bartib_completions()
{
	local CWORD=${COMP_WORDS[COMP_CWORD]}
	ALL_PROJECTS=`bartib projects`

	local IFS=$'\n'
	CANDIATE_PROJECTS=($(compgen -W "${ALL_PROJECTS[*]}" -- "$CWORD"))

	if [ ${#CANDIATE_PROJECTS[*]} -eq 0 ]; then
		COMPREPLY=()
	else
		COMPREPLY=($(printf "\"%s\"\n" "${CANDIATE_PROJECTS[@]}"))
	fi
}

complete -F _bartib_completions bartib

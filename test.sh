#!/bin/sh
export TODO_LIST=.todo_list
export PATH=$(pwd):$PATH
set -ex
todo_
do_ Write tests
echo "Setup Linux CI\n• Setup Windows CI" | do_
todo_
did_ Write tests
todo_ | grep CI | did_
todo_

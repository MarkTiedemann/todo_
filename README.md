# todo_

**Minimal command-line todo list.**

| **Linux & Mac** | **Windows** |
| --------------- | ----------- |
| [![Travis Build Status](https://travis-ci.org/MarkTiedemann/todo_.svg?branch=master)](https://travis-ci.org/MarkTiedemann/todo_) | [![AppVeyor Build Status](https://ci.appveyor.com/api/projects/status/82o7yqy74pv3ca3i?svg=true)](https://ci.appveyor.com/project/MarkTiedemann/todo) |

## Quickstart

```diff
$ do_ Write tests
+ Write tests

$ do_ Setup CI
• Write tests
+ Setup CI

$ todo_
• Write tests
• Setup CI

$ did_ Write tests
- Write tests
• Setup CI

$ did_ Setup CI
- Setup CI
```

## Installation

**Download binaries:**

From GitHub [Releases](https://github.com/MarkTiedemann/todo_/releases).

**Or build from source:**

```sh
git clone https://github.com/MarkTiedemann/todo_
cd todo_
make
```

## FAQ

**> Why the trailing underscore?**

To avoid clashing with the `do` statement used in for-loops.

**> Where is the list saved?**

By default, it is saved in `~/.todo_List`.

**> How can I save the list in a different location?**

Set the `TODO_LIST` environment variable pointing to the new location. For example:

```sh
export TODO_LIST=~/Documents/TodoList.txt
```

**> How can I re-arrange the list?**

There is no built-in functionality for this in `todo_`. But since the todo list is persisted in a text file, you can simply edit `~/.todo_list` (or `$TODO_LIST`) in a text editor and re-arrange the lines.

**> How can I add multiple todos at the same time?**

By piping the todos to stdin. For example:

```diff
$ cat todos.txt
Setup Linux CI
Setup Windows CI

$ cat todos.txt | do_
+ Setup Linux CI
+ Setup Windows CI
```

**> How can I search for todos?**

There is no built-in search functionality in `todo_`. But you can easily `grep` the output of `todo_`. For example:

```sh
$ todo_ | grep CI
• Setup Linux CI
• Setup Windows CI
```

If you find yourself needing this functionality a lot, you can create a function for it. For example:

```sh
todo_g () { todo_ | grep "$*" }
```

**> How can I remove multiple todos at the same time?**

By piping the todos to stdin. For example:

```diff
$ todo_ | grep CI | did_
- Setup Linux CI
- Setup Windows CI
```

Note that leading `• ` will be stripped automatically.

If you find yourself needing this functionality a lot, you can create a function for it. For example:

```sh
did_g () { todo_ | grep "$*" | did_ }
```

**> How can I work with multiple lists?**

You can use the `TODO_LIST` environment variable to accomplish this. For example, by setting up functions to switch between your lists:

```sh
todo_home () { export TODO_LIST=~/.todo_home; todo_ }
todo_work () { export TODO_LIST=~/.todo_work; todo_ }
```

If you are regularly working with multiple lists, you may want to set the `TODO_PRINT_PATH` env var to see which list you are currently working with. For example:

```sh
$ export TODO_PRINT_PATH=1

$ todo_
~/.todo_home
• Buy groceries
• Mow the lawn
```

## License

MIT

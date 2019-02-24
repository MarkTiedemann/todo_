# todo_

**Minimal command-line todo list.**

```diff
$ todo_

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

$ todo_
```

## FAQ

**> Why the trailing underscore?**

To avoid clashing with the `do` statement used in for-loops.

**> Where is the list saved?**

By default, it is saved in `~/.todo_List`.

**> How can I save the list in a different location?**

Set the `TODO_LIST` env var pointing to the new location.

**> How can I rearrange the list?**

Edit `~/.todo_list` or `$TODO_LIST`.

**> How can I search for todos?**

You can `grep` the output of `todo_`.

**> How can I work with multiple lists?**

By using the `TODO_LIST` env var, for example:

```sh
alias todo_home="export TODO_LIST=~/.todo_home; todo_"
alias todo_work="export TODO_LIST=~/.todo_work; todo_"
```

If you are regularly working with multiple lists, you may want to set the `TODO_PRINT_PATH` env var to see which list you are currently working with.

## License

MIT

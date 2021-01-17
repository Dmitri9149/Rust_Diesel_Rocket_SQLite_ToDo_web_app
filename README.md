ToDo applicatiom using SQLite and Rust (Diesel, Rocket).

The general scheme is from here: https://erwabook.com/intro/index.html
See also diesel documentation: https://github.com/diesel-rs/diesel/tree/master/examples ; http://diesel.rs/guides/

To work with DB:

Wrong input -> some help:

`~>/~$ cd mytodo
~>/mytodo$ cargo run --bin todo add 'do somethin'
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/todo add 'do somethin'`
arg[0]-> target/debug/todo; args[1] -> add
subcommands:

new<title>: create new task

show: get the list of all tasks

delete: delete 'tasks' by stating 'task like string'; be carefull!
'learn C' and 'learn C++' will both deleted by pattern 'learn C'

done: mark a task as 'done', it requires to know the id of the task
that is why it is recommended to use 'show' first to get the id
~>/mytodo$ 

`
`
~>/mytodo$ cargo run --bin todo new 'do something'
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/todo new 'do something'`
arg[0]-> target/debug/todo; args[1] -> new
~>/mytodo$
`
`
~>/mytodo$ cargo run --bin todo show
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/todo show`
arg[0]-> target/debug/todo; args[1] -> show
TASKS
-----
id -> 1  title -> learn Rust   done -> pending
id -> 4  title -> learn Kubernetes   done -> pending
id -> 7  title -> learn Haskell   done -> pending
id -> 8  title -> learn C++    done -> pending
id -> 9  title -> learn C   done -> done
id -> 10  title -> learn PyTorch   done -> pending
id -> 11  title -> do something   done -> pending
~>/mytodo$

`
`
>/mytodo$ cargo run --bin todo done 10
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/todo done 10`
arg[0]-> target/debug/todo; args[1] -> done
This is 10
The task ''learn PyTorch' is marked as 'done'
The task with the id 10 was marked as 'done'
~>/mytodo$ cargo run --bin todo show
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/todo show`
arg[0]-> target/debug/todo; args[1] -> show
TASKS
-----
id -> 1  title -> learn Rust   done -> pending
id -> 4  title -> learn Kubernetes   done -> pending
id -> 7  title -> learn Haskell   done -> pending
id -> 8  title -> learn C++    done -> pending
id -> 9  title -> learn C   done -> done
id -> 10  title -> learn PyTorch   done -> done
id -> 11  title -> do something   done -> pending
~>/mytodo$ cargo run --bin todo delete 'learn C'
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/todo delete 'learn C'`
arg[0]-> target/debug/todo; args[1] -> delete
Deleted 2 posts
~>/mytodo$ cargo run --bin todo show
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/todo show`
arg[0]-> target/debug/todo; args[1] -> show
TASKS
-----
id -> 1  title -> learn Rust   done -> pending
id -> 4  title -> learn Kubernetes   done -> pending
id -> 7  title -> learn Haskell   done -> pending
id -> 10  title -> learn PyTorch   done -> done
id -> 11  title -> do something   done -> pending
~>/mytodo$ cargo run --bin todo done 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/todo done 1`
arg[0]-> target/debug/todo; args[1] -> done
This is 1
The task ''learn Rust' is marked as 'done'
The task with the id 1 was marked as 'done'
~>/mytodo$ 

`
The work is in progress.

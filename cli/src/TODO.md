## Developing plan
- prevent console window on spawning in windows (just flag doesn't work)
- define tasks as allowed in parallel
- add test for error: command in root level of scenario
- catch errors in concating strings like $d = "bla${a}bla"; (should be "bla{$a}bla")
- add simple assignation $a = $b
- add web-server for remote access
- dry-run: run and ignore terminal commands & show a map of executing
- add named arguments for functions @hash (track:(bla, bla); run:(bla))
- Task:has_meta - not implemented... it's dummy method
- a way to run tasks in parallel: `sibs * --lint` will run lint task for all components, which has it
- invalid UTF from command
- implement Cancellation on @exit function
- feature: signals and waiter... stop at some point of script until signal -> will allow sync parallel tasks
- missed commands/functions:
-- try / catch / check err
- add anti-deadlock
- Error MissedSemicolon doesn't give error report with error location. Canbe tested with signal.sibs. Remove 
semicollon and the end on "true".
- tracking time of some tasks gives a chart with performance changes (benchmarks)
- "?" as tolerance should be moved to Element layer

NO Errors:
`create-user --name={$user_name} --role={if $is_admin == "true" ["admin";]  ["user";]}`
$is_verified AND $has_permission => @proceed_with_action;
true != false doesn't work

- show trace like:
```
1 │         $c_str = "value_c_{$a_str}-{$b_str}";
12 │         $d_bool = @env::is_os(linux);
13 │         if @env::is_os(linux) [
14 │             :self:build("smth"; "prod1");
                                      ^^^^^
                                      Calling of task
15 │         ];
16 │     ];
17 │     build($input_a: {string}; $mode: dev | prod;) [
                                   ^^^^^^^^^^^^^^^^^
                                   ERROR: Value "prod1" doesn't match to allowed: dev | prod

18 │         $a_str = "value_a";
19 │         `ls -lsa`;
20 │     ];
```

- features:
-- detect type of project behind each component (nodejs, rust, ruby etc)
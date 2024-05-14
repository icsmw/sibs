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
- limit AnyValue only with supported types
- missed commands/functions:
-- break
-- loop
-- try / catch / check err


NO Errors:
`create-user --name={$user_name} --role={IF $is_admin == "true" ["admin";]  ["user";]}`
$is_verified AND $has_permission => @proceed_with_action;
true != false doesn't work

- show trace like:
```
1 │         $c_str = "value_c_{$a_str}-{$b_str}";
12 │         $d_bool = @env::is_os(linux);
13 │         IF @env::is_os(linux) [
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
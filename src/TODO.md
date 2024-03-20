## Developing plan
- support of comments "//"
- support of tokens extracting
- support mapping for different files (added with @import)
- define tasks as allowed in parallel
- add test for error: command in root level of scenario
- catch errors in concating strings like $d = "bla${a}bla"; (should be "bla{$a}bla")
- support modules for functions grouping; like @os.get
- add simple assignation $a = $b
- add web-server for remote access
- dry-run: run and ignore terminal commands & show a map of executing
- this is valid "@fs::path_join [$tmp_path $file_name];", this is NOT "@fs::path_join [$tmp_path; $file_name;];" or "@fs::path_join [$tmp_path ,$file_name];". Error should be shown.
- add named arguments for functions @hash track::(bla, bla) run::(bla)
- prevent dead-loop related to nested Element parsing
- a way to run tasks in parallel: `sibs * --lint` will run lint task for all components, which has it

NO Errors:
`create-user --name={$user_name} --role={IF $is_admin == "true" ["admin";]  ["user";]}`
$is_verified AND $has_permission => @proceed_with_action;
true != false doesn't work


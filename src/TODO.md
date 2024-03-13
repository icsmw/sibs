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

NO Errors:
`create-user --name={$user_name} --role={IF $is_admin == "true" ["admin";]  ["user";]}`
Doesn't give error                                                        ^^ missed ELSE

`convert image.jpg -pointsize 24 -fill blue -annotate +100+100 'Text with \`backtick\`
                                                                                     ^^ no closed ` (no errors if it's end of a line)

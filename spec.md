This is my own syntax, which I’ve developed for build system. It’s quite light, but still needs to be tested indeed. 

Okay, let’s discover all statements and definitions. 

Here is a list of all available entities:
```
pub enum ElTarget {
Function,
If,
Each,
First,
VariableAssignation,
Optional,
Reference,
PatternString,
VariableName,
Comparing,
Values,
Block,
Meta,
Command,
Task,
Component,
}

```

VariableName
Name of variable starts from $ and include alphabetic and numeric symbols, also “-” and “_”. Examples of variable names: 
$var
$var_a
$var-b

VariableAssignation,
Very classic way, using “=”. Examples:
$var = “string”;

as value can be:
    - Block,
    - First,
    - Function,
    - If,
    - PatternString,
    - Values,
    - Comparing,
    - VariableName,

Function
Name of function starts from @ and include alphabetic and numeric symbols, also “-” and “_”. Examples of function names: 
@get
@get_os
@os-get

function can have arguments in “(…)” and divided with “;”. For example:
@find(“something”)
@find(“one”; “two”)

As argument function can get:
- Values,
- Function,
- If,
- PatternString,
- Reference,
- Comparing,
- VariableName,
- Command,

Comparing
This is construction, which returns true/false. In can be:
left == right
left != right

As left and right can be:
- VariableName,
- Function,
- PatternString,
- Values,

For example
$var_a == “one”;
$var_b != “one”

PatternString
String closed with quotes, like: “this is my string”;
PatternString supports injections, which can be defined between {…}. For example:

“this is my function about{$subject} and something else”;

As injection can be used:
- VariableName,
- Function,
- If

Command
Same as PatternString but instead quotes uses tilda… For example `npm build app`;
As PatternString supports injections:

`npm run {$script_name}`;

As injection can be used:
- VariableName,
- Function,
- If

Optional
This is optional action. If condition returns true, action will be fired. Condition and action divided with “=>”: condition => action

For example:

@os linux => `ls -lsa`;

As condition can be used:
- Function,
- VariableName,
- Reference,
- Block,
- Comparing,

As action can be used:
- Function,
- Reference,
- VariableAssignation,
- Each,
- Block,
- First,
- PatternString,
- Command,

Values
This is collection of values. Like an array or vector. Values should be defined between (…) and divided with “;”. For example

(“one”; “two”; @get_three)

As value can be used:
- Command,
- Function,
- If,
- PatternString,
- Reference,
- Values,
- Comparing,
- VariableName,

Block
Block is a collection/list of others. Content of block locked between “[…]”. For example:

[
   @is_os(“windows) => `format c:`;
   $os_name = @get_os_name;
   IF $os_name == “windows” [
       @print(“damn”);
   ]
]


Inside block can be:
- Function,
- If,
- Each,
- First,
- VariableAssignation,
- Optional,
- Reference,
- PatternString,
- VariableName,
- Comparing,
- Values,
- Block,
- Meta,
- Command,

If statement
Very classic statement:

IF conditions [
   // actions to do
   // actually this is Block element
]

IF conditions [ … ] IF conditions [ …. ]

IF conditions [ … ] ELSE [ … ]

IF conditions [ … ] IF conditions [ …. ] ELSE [ … ] 

As for conditions it supports logical operators AND, OR

IF condition AND condition […]
IF condition OR condition […]

Conditions can be grouped with (…)

IF (condition AND condition) OR condition […]

As action If has Block element.

As condition can be used: 
- Comparing,
- Function


First
Statements starts with key-word FIRST and has after Block with actions. 

Example:

FIRST [
    @os(window) => "release{$version}-win.zip";
    @os(linux) => "release{$version}-linux.zip";
    @os(darwin) => "release{$version}-darwin.zip";
    @exit(1; "Unsupported OS: {@os}");
];

First runs actions one by one and returns the first value from some actions (not all actions return values)

Each

Iterate values

EACH($file; $files) [
    // do something for each $file
];

For now lets ignore rest: Component, Task and Meta… Not important for now.

Please confirm, is it clear or you have some questions? And please do not start generate a tests. Just confirm is it clear or not
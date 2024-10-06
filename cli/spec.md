This is my own syntax, which I’ve developed for build system. It’s quite light, but still needs to be tested indeed.

Okay, let’s discover all statements and definitions.

Here is a list of all available entities:

```
pub enum ElementRef {
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
Integer,
Boolean,
}

```

# Integer

Equal to rust `isize` type

# Boolean

Standard bool type: true / false

# VariableName

Name of variable starts from $ and include alphabetic and numeric symbols, also “-” and “\_”. Examples of variable names:
$var
$var_a
$var-b

# VariableAssignation

Very classic way, using “=”. Examples:
$var = “string”;

as value can be: - Block, - First, - Function, - If, - PatternString, - Values, - Comparing, - VariableName, - Integer, - Boolean,

# Function

Name of function starts from @ and include alphabetic and numeric symbols, also “-” and “\_”. Examples of function names:
@get
@env::os
@env::is_os-get

function can have arguments in “(…)” and divided with “;”. For example:
@find(“something”)
@find(“one”; “two”; 123; true; false)

As argument function can get:

-   Values,
-   Function,
-   If,
-   PatternString,
-   Reference,
-   Comparing,
-   VariableName,
-   Command,
-   Integer,
-   Boolean,

# Comparing

This is construction, which returns true/false. In can be:
left == right
left != right
left < right
left > right

As left and right can be:

-   VariableName,
-   Function,
-   PatternString,
-   Values,
-   Integer,
-   Boolean,

For example
$var_a == “one”;
$var_b != “one”
$var_b != 42
$var > 42
$var < 24

# PatternString

String closed with quotes, like: “this is my string”;
PatternString supports injections, which can be defined between {…}. For example:

“this is my function about{$subject} and something else”;

As injection can be used:

-   VariableName,
-   Function,
-   If

# Command

Same as PatternString but instead quotes uses tilda… For example `npm build app`;
As PatternString supports injections:

`npm run {$script_name}`;

As injection can be used:

-   VariableName,
-   Function,
-   If

# Optional

This is optional action. If condition returns true, action will be fired. Condition and action divided with “=>”: condition => action

For example:

@env::is_os linux => `ls -lsa`;

As condition can be used:

-   Function,
-   VariableName,
-   Reference,
-   Block,
-   Comparing,

As action can be used:

-   Function,
-   Reference,
-   VariableAssignation,
-   Each,
-   Block,
-   First,
-   PatternString,
-   Command,
-   Integer,
-   Boolean,

# Values

This is collection of values. Like an array or vector. Values should be defined between (…) and divided with “;”. For example

(“one”; “two”; @get_three)

As value can be used:

-   Command,
-   Function,
-   If,
-   PatternString,
-   Reference,
-   Values,
-   Comparing,
-   VariableName,
-   Integer,
-   Boolean,

# Block

Block is a collection/list of others. Content of block locked between “[…]”. For example:

[
@is_os(“windows) => `format c:`;
$os_name = @get_os_name;
if $os_name == “windows” [
@print(“damn”);
]
]

Inside block can be:

-   Function,
-   If,
-   Each,
-   First,
-   VariableAssignation,
-   Optional,
-   Reference,
-   PatternString,
-   VariableName,
-   Comparing,
-   Values,
-   Block,
-   Meta,
-   Command,
-   Integer,
-   Boolean,

# If statement

Very classic statement:

if conditions [
// actions to do
// actually this is Block element
]

if conditions [ … ] if conditions [ …. ]

if conditions [ … ] else [ … ]

if conditions [ … ] if conditions [ …. ] else [ … ]

As for conditions it supports logical operators AND, OR

if condition && condition […]
if condition || condition […]

Conditions can be grouped with (…)

if (condition && condition) || condition […]

Nested groups are supported too

if (condition && condition && (condition || condition)) || condition […]

As action If has Block element.

As condition can be used:

-   Comparing,
-   Function
-   VariableName

# First

Statements starts with key-word FIRST and has after Block with actions.

Example:

FIRST [
@env::is_os(window) => "release{$version}-win.zip";
@env::is_os(linux) => "release{$version}-linux.zip";
@env::is_os(darwin) => "release{$version}-darwin.zip";
@exit(1, "Unsupported OS: {@env::is_os}");
];

First runs actions one by one and returns the first value from some actions (not all actions return values)

# Each

Iterate values

each($file; $files) [
// do something for each $file
];

For now lets ignore rest: Component, Task and Meta… Not important for now.

Please confirm, is it clear or you have some questions? And please do not start generate a tests. Just confirm is it clear or not

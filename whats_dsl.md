# What's DSL?

There seems to be no clear definition for the term DSL. Both Makefile and Ant are DSLs, but Makefile has its own grammar, while Ant describes build files in XML. It seems that it is not necessary to have a unique grammar or to be Turing complete and capable of complex calculations to be a DSL.

First, I will explain my understanding of what language is. Language refers to what is executed in the form below:
```Rust
// It needs to be sequencial
for item in code{
	// Processing branches depending on the kind of the item
	match item{ /* do something */ }
}

// or

for item in code{
	// Dynamic dispatch
	item.run(&mut context);
}
```
When the code is written in text before conversion, the format used to write the text is called a language.

Data refers to what is used in the form below:
```Rust
function(data)
```
Code is a subset of data. I think the essence of language is that data determine how processing is done.

In Munyo language, you can write data and code naturally, but it's more specialized to write code.

If a language can be used for programming, I think it should be called a programming language, and if it has a specific purpose, it should be called a domain-specific language. They shouldn't be mutually exclusive.

## How should logic be described in Munyo language?

I would like to suggest how to write logic in the Munyo language here.

In the Munyo language, you can write something like this:
```Rust
set x 10
set y 20
label loop_start
add x y
goto loop_start
```
However, I think this approach is too dificult.

It is also possible to embed languages such as Lua in the Munyo language. However, in the Munyo language, '\\' and ‘|’ are special characters, so you will need to swap them with other characters(before parsing Munyo and before executing Lua script).
```
>>Lua
io.write("Please enter the value in meters: \n")
meter = io.read()
answer = meter * 3.28
print( meter .. " meters is " .. answer .. " feet.")

|| '\' and '|' are swapped to '@' and '`'
>@MunyoValue`name value`name2 value2
```
I think basically you don't need to embed scripting languages.

You can use Rust functions through the match statement, so you can use Rust for logic.

Since it is difficult to interoperate when described in two languages in separated files, a mechanism is needed to check whether the function is used only once by examining all Munyo files.
```Rust
// You can use the keys of this HashMap to check whether it's used only once.
one_time_functions.insert(
    "enter_the_number_in_meters_and_convert_it_to_feet".to_string(), 
    call_the_rust_function);

// This can be used to check whether they are used more than once.
normal_functions.insert(
    "a_function_that_is_called_multiple_times".to_string(), 
    call_another_rust_function);

for item in code{
	if normal_functions.contains_key(&item.typename){
		//...
	} else if one_time_functions.contains_key(&item.typename){
		//...
	} else match &item.typename{
		//...
	}
}
```

## Why you need DSLs

When you write data in Rust, the compilation time becomes longer and the size of the executable file becomes larger. Generally, when executing code that constructs data in a compiled language, the execution speed is often significantly slower than reading and analyzing text file to construct data.
```Rust
// Directly writing data in compiled languages is slow
let data = Data{ a : 10, b : 20, children : vec![ Data{ a : 30, ...}, ...]}
```
Since code is also data, problems can occur if it is written too long. Besides, in something like a game script, functions such as “stop execution temporarily and resume execution when the A button is pressed” and “jump globally depending on the selected option” are required, which is difficult to achieve with general-purpose programming languages.

Let's see how to implement these functions in Munyo.
```
|| file_a.munyo
|| # means "display text message" in this DSL.
# Press A button.
Stop
# Choose A or B.
Select
	Case A|goto label_a
	Case B|goto file_b/label_b
Label label_a
# You chose A.

|| file_b.munyo
Label label_b
# You chose B.
```
First, you need to collect all the labels from all the Munyo files, and remember the positions of the labels. The position is:
```Rust
struct Position{
	filename : String,
	indexes : Vec<usize>,
}
```
Munyo files are translated to Vec\<Item>, and Items can contain children, which is also Vec\<Item>, so you need indexes to determine the position of an Item. (Maybe this DSL only needs the first index...)

While running this DSL, the runner must track the position it executes to resume when it stopped. The runner also needs to execute from the given position to implement goto(and stop).

So, I think there is a difference between logic that should be written in DSL and logic that should be written in Rust, and it is a difficult problem to distinguish between them.

## Are natural languages languages?

In my difinition, languages are executed with match, dynamic dispatch, or something similar to that(hashtables for matching, CPUs that examine the opcode and branch...).

Natural languages also take this form:
```Rust
for word in text{ ... }
```
Currently, we don't know how human brains work, but we know how AI works. In the AI world, a word is a vector.
```Rust
for word in text{
	execute(word, &mut neurons)
}
```
No match/dynamic dispatch/etc., so formally, a natural language is not a language but data.

However, a word vector is something like this:
```
word "in"
    ↓
a the of  in to at...
0  0   0  1  0  0...
```
A word vector is essentially an index of a word list, and AI performs completely different processing based on that index. I think this is essentially a match statement. So, in my definition, natural languages are languages, at least in the AI world.
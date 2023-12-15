# What's DSL?

There seems to be no clear definition for the term DSL. Both Makefile and Ant are DSLs, but Makefile has its own grammar, while Ant describes build files in XML. It seems that it is not necessary to have a unique grammar or to be Turing complete and capable of complex calculations to be a DSL.

First, I will explain my understanding of what language is. Language refers to what is ultimately executed in the form below:
```Rust
for item in code{
	match item{ /* do something */ }
}

// or

for item in code{
	// Object oriented style
	item.run(context);
}
```
When the code is written in text, the format used to write the text is called a language.

Data refers to what is used in the form below:
```Rust
function(data)
```
Code is a subset of data.

In Munyo language, you can write data and code naturally, but it's more specialized to write code.

If a language can be used for general programming, I think it should be called a programming language, and if it has a specific purpose, it should be called a domain-specific language.

## How should logic be described in Munyo language?

I would like to suggest how to write logic in Munyo language here.

In Munyo language, you can write something like this:
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
>@MunyoValue`name value`name2 value2
```
I think basically you don't need to embed scripting languages.

You can use Rust functions through the match statement, so you can use Rust for logic.

Since it is difficult to interoperate when described in two languages in separated files, a mechanism is needed to check whether the function is used only once by examining all Munyo files.
```Rust
// You can use the keys of this HashMap to check whether it's used only once.
one_time_functions.insert(
    "enter_the_number_in_meters_and_convert_it_to_feet", 
    call_the_rust_function);

// This can be used to check whether they are used more than once.
normal_functions.insert(
    "a_function_that_is_called_multiple_times", 
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
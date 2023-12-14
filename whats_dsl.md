# What's DSL

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

On the other hand, data refers to what is used in the form below:
```Rust
function(data)
```

Munyo can write
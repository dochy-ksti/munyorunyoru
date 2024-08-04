## [html_samples](https://github.com/dochy-ksti/munyorunyoru/tree/master/munyo-sample/src/samples/html_samples)

Samples which converts Munyo to HTML. The conversion is simple because Munyo and HTML has very similar structures. Besides, HTML is a visually attractive format, so it's chosen for samples. What we do in the samples is essentially handwriting HTML, so they are not practical examples.

### [html_sample_1](https://github.com/dochy-ksti/munyorunyoru/tree/master/munyo-sample/src/samples/html_samples/sample1)

A sample of a language processor to convert Munyo to HTML using match statement. [tags.rs](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample1/tags.rs)

My definition of the language processor and the language is explained [here.](https://github.com/dochy-ksti/munyorunyoru/blob/master/whats_dsl.md)

It also shows "Line Continuation" to write multiline text([sample1.munyo](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample1/sample1.munyo)) and "RestOf" to capture the text of the rest of the line. [tags.rs](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample1/tags.rs)

### [html_sample_2](https://github.com/dochy-ksti/munyorunyoru/tree/master/munyo-sample/src/samples/html_samples/sample2)

In HTML, we write text directly, like "&lt;P&gt; direct text &lt;/p&gt;". To do that in Munyo, ">>" is used in this sample. [sample2.munyo](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample2/sample2.munyo)

The default-type "Text" and the empty-line-type "BR" is defined in the line ">>Text | BR". See [lang_spec.txt](https://github.com/dochy-ksti/munyorunyoru/blob/master/lang_spec.txt) for more details.

You can write multiline text without line-continuation because every line has default type "Text", but non-text items become a bit verbose.

Like Markdown, empty lines has meanings because empty-line-type "BR" is defined and it's converted to the &lt;BR&gt; tag. [output.html](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample2/output.html) 

HTML source text is useful for illustrative purposes, but you need web browsers to visually understand HTMLs. Download the sources and open them with your browser.

### [html_sample_3](https://github.com/dochy-ksti/munyorunyoru/tree/master/munyo-sample/src/samples/html_samples/sample3)

Creating new tags by adding the tags to the language processor. [tags.rs](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample3/tags.rs)

The tags "Alice" and "Bob" make writing the conversations of them easier. [sample3.munyo](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample3/sample3.munyo)

### [html_sample_4](https://github.com/dochy-ksti/munyorunyoru/tree/master/munyo-sample/src/samples/html_samples/sample4)

With MunyoItem, you can use Munyo without "serde". That means you can use every tag without specifying the data structure of the tag. [untyped.rs](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample4/untyped.rs), [untyped.munyo](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/html_samples/sample4/untyped.munyo)

MunyoItem isn't constrained with Rust syntax, so you can use "#" as a typename, for example. But when errors occur, you can't get the line numbers of them. If you want the line numbers of errors and want to ignore Rust syntax, you need to use [from_str_with_metabuilder](https://github.com/dochy-ksti/munyorunyoru/blob/master/src/lang/from_str_with_metabuilder.rs). But currently, the materials about how to use it are unavailable.

## [poke_sample](https://github.com/dochy-ksti/munyorunyoru/tree/master/munyo-sample/src/samples/poke_sample)

Explained in the [readme.md](https://github.com/dochy-ksti/munyorunyoru/) of the main crate.

This is only meant for writing data efficiently, so it's a practical example, I think.

## [custom_new_sample](https://github.com/dochy-ksti/munyorunyoru/blob/master/munyo-sample/src/samples/custom_new_sample.rs)

When you implement TupleVisitor, you can accept arguments more freely without implementing parser. I don't think this has practical usages.

## [poke_move_sample](https://github.com/dochy-ksti/munyorunyoru/tree/master/munyo-sample/src/samples/poke_move_sample)

Example about how to parse the arguments freely with your custom parser. You can let a line have any number of arguments whose types are automatically detected by the representations of the arguments.

With this method, you can accept any arguments with your custom parser, and get the line numbers of errors, but typenames must follow Rust identifier naming syntax. If you want to ignore it, you need to use [from_str_with_metabuilder](https://github.com/dochy-ksti/munyorunyoru/blob/master/src/lang/from_str_with_metabuilder.rs).
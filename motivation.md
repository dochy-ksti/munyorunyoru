Reading needs to write correctly, but data often become unreadable.

This is very old Japanese emperor list.
```json5
{
	name : "Nintoku",
	children :[
		{
			name : "Richu",
			children : [
				{
					name : "Iwasakanoichinobenoomoha"
					children : [
						{
							name : "Kenzou"
							born : 450,
							died : 487,
						}
						{
							name : "Ninken"
							born : 449,
							died : 498
						},
					],
					born : "unknown",
					died : 455,
				}
			],
			born:336,
			died:405
		},
		{
			name : "Hanzei",
			born:336,
			died:410,
		},
		{
			name : "Ingyou",
			children :[
				{
					name : "Ankou",
					born : 401,
					died : 456,
				},
				{
					name : "Yuryaku",
					children : [
						{
							name : "Seinei",
							born : 444,
							died : 484,
						}
					],
					born : 418,
					died : 479,
				}
			]
			born : 376,
			died : 453,
		}
	],
	born : 290,
	died : 399,
}
```
When you are looking at nested data, your head gets filled with that data, and when you come out of the nest, you forget what the original data was. So data must not be placed after children.
```json5
{
	name : "Nintoku",
	born : 290,
	died : 399,
	children :[
		{
			name : "Richu",
			born:336,
			died:405
			children : [
				{
					name : "Iwasakanoichinobenoomoha",
					born : "unknown",
					died : 455,
					children : [
						//...
```
For the same reason, an item cannot have two nested data.
```json5
{
	name : "Nintoku",
	born : 290,
	died : 399,
	poems :[
		"沖へには　小船(をぶね)連(つら)らく　くろざやの　まさづ子吾妹(わぎも)　国へ下らす",
		"おしてるや　難波の崎よ　出で立ちて　我が国見れば　淡島(あはしま)　自凝(おのごろ)島　檳榔(あぢまさ)の　島も見ゆ　放(さけ)つ島見ゆ",
		"山県(やまがた)に　蒔ける青菜も　吉備人と　共にし摘めば　楽しくもあるか",
		//...
	],
	children :[
		{
			name : "Richu",
			born:336,
			died:405
			children : [
				//...
			],
		},
		//...
	],
```
Two nested data make the data incomprehensible, so an item should have zero or one children. If an item can contain only one children at most, the name "children" is verbose.

Nested large data is difficult to understand, so programmers divide the entire nest to fit on one screen and use indentation to visually understand the structure of the data. From that perspective, using one line to one variable is very redundant. The amount of data that can be displayed on one screen is greatly reduced, making it difficult to understand the structure of the data.

Let's see the first data converted to Munyo.
```
>>Tennou
Nintoku 290-399
	Richu 336-405
		Iwasakanoichinobenoomoha ?-455
			Kenzou 450-487
			Ninken 449-498
	Hanzei 336-410
	Ingyou 376-453
		Ankou 401-456
		Yuryaku 418-479
			Seinei 444-484
```
Very easy to read, isn't it? It's also very easy to write.
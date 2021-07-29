# HTMLisp

## What is this?

A compiler that takes in lisp style html and outputs normal html

## Why is this?

To annoy a few people

## How do I use this?

* CD into cloned repo
* Compile: `cargo build --release`
* Copy executable to `/usr/bin` (`sudo cp target/release/htmlisp /usr/bin`)
* Run: `htmlisp -i <path to htmlisp input file> -o <path to html output file>` 

## Example:

(example.htmlisp)
```lisp
(html
    (head
        (meta :charset "UTF-8")
        (meta :name "viewport" :content "width=device-width, initial-scale=1"))
    (body
        (h1 "Hello World")
        (p "This is a paragraph")))
```

compiled using `htmlisp --input example.htmlisp --output example.html` will produce

(example.html)
```html
<html><head><meta charset="UTF-8"></meta><meta name="viewport" content="width=device-width, initial-scale=1"></meta></head><body><h1>Hello World</h1><p>This is a paragraph</p></body></html>
```

or with `htmlisp --prettify --input example.htmlisp --output`

(example.html)
```html
<html>
	<head>
		<meta charset="UTF-8"></meta>
		<meta name="viewport" content="width=device-width, initial-scale=1"></meta>
	</head>
	<body>
		<h1>
			Hello World
		</h1>
		<p>
			This is a paragraph
		</p>
	</body>
</html>
```
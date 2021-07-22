#### PIP DEPENDENCY REQUIRED (`pip install python-lorem`) ####

import random
import subprocess
import time
import lorem

start = """(html
(head
    (meta :charset "utf-8")
    (meta :name "viewport" :content "width=device-width, initial-scale=1")
    (title "Testing"))
(body :style "font-family: sans-serif;\""""
end = "))"
containers = [
    "article",
    "aside",
    "div",
    "footer",
    "header",
    "main",
    "nav",
    "section",
] + [None] * 2

text_content = [
    "blockquote",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "p",
    "pre",
] + [None] * 4

inline_text = ["a", "b", "code", "em",
               "i", "q", "span", "strong"] + [None] * 10

text = text_content + inline_text
any_elem = containers + text


def main():
    num = 10

    for i in range(0, num):
        inner = "".join([gen_container() for _ in range(50)])
        test = start + inner + end
        with open(f"./input/random{i}.htmlisp", "w") as file:
            file.write(test)

    results = [0] * num
    for i in range(0, num):
        start_time = time.time_ns()
        subprocess.run(
            [
                "../target/release/htmlisp",
                f"input/random{i}.htmlisp",
                f"output/random{i}.html",
            ],
            stderr=subprocess.STDOUT,
        )
        end_time = time.time_ns()
        results[i] = end_time - start_time

    avg = sum(results) / num
    print("Average ms:", avg / 1000000)


def gen_container(depth=1) -> str:
    tag = random.choice(containers)
    if tag == None:
        return "\n" + "\t" * depth + f'"{lorem.get_paragraph()}"'

    string = "\n" + "\t" * depth + f"({tag}"
    for i in range((5 - depth) * 3):
        string += "\n"
        if bool(random.getrandbits(1)) and depth < 4:
            string += gen_container(depth + 1)
        else:
            string += gen_text_content(depth + 1)

    return string + ")"


def gen_text_content(depth: int) -> str:
    tag = random.choice(text_content)
    if tag == None:
        return "\t" * depth + f'"{lorem.get_paragraph()}"'

    string = "\n" + "\t" * depth + f'({tag}' + "\n"
    string += "\n".join([gen_inline_text(depth + 1)
                        for _ in range((6 - depth) * 5)]) + ")"
    return string


def gen_inline_text(depth: int) -> str:
    tag = random.choice(inline_text)
    if tag == None:
        return "\t" * depth + f'" {lorem.get_sentence()}"'

    return "\t" * depth + f'({tag} " {lorem.get_sentence()}")'


if __name__ == "__main__":
    main()

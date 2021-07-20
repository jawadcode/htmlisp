#### PIP DEPENDENCY REQUIRED (`pip install python-lorem`) ####

import random
import subprocess
import lorem

start = """(html
(head
    (meta :charset "utf-8")
    (meta :name "viewport" :content "width=device-width, initial-scale=1")
    (title "Testing"))
(body :style "font-family: sans-serif;\""""
end = '))'
containers = ['article', 'aside', 'div', 'footer',
              'header', 'main', 'nav', 'section'] + [None] * 1
text_content = ['blockquote', 'button', 'div', 'hr', 'h1', 'h2',
                'h3', 'h4', 'h5', 'h6', 'input', 'p', 'pre'] + [None] * 5
inline_text = ['a', 'b', 'br', 'code', 'em',
               'i', 'q', 'span', 'strong'] + [None] * 10
text = text_content + inline_text
any_elem = containers + text


def main():
    for i in range(0, 10):
        inner = "".join([gen_container() for _ in range(10)])
        test = start + inner + end
        with open(f"./input/random{i}.htmlisp", "w") as file:
            file.write(test)

        subprocess.run([
            "../target/release/htmlisp",
            f"input/random{i}.htmlisp",
            f"output/random{i}.html"
        ], stderr=subprocess.STDOUT)


def gen_container(depth=1) -> str:
    tag = random_choice(containers)
    if tag == None:
        return "\n" + "\t" * depth + f'"{lorem.get_paragraph()}"'

    string = "\n" + "\t" * depth + f'({tag}' + "\n"
    if bool(random.getrandbits(1)) and depth < 3:
        string += "\n".join([gen_container(depth + 1) for _ in range(depth+4)])
    else:
        string += "\n".join([gen_text(depth + 1) for _ in range(depth*3)])

    return string + ")"


def gen_text(depth: int) -> str:
    tag = random_choice(text)
    if tag == None:
        return "\t" * depth + f'"{lorem.get_sentence()}"'
    else:
        return "\t" * depth + f'({tag} "{lorem.get_sentence()}")'


def random_choice(lst: list[str]) -> str:
    rand_num = random.randint(1, len(lst)) - 1
    return lst[rand_num]


if __name__ == "__main__":
    main()

import re
import sys

file_name = sys.argv[1]

with open(file_name) as file:
    file_lines = file.readlines()

def evaluate(string):
    if len(string.split(' ')) == 1: return int(string)
    eval = 1
    if not '(' in string:
        eval_strings = string.split(' * ')
        for mult_eval in eval_strings:
            add_eval = 0
            for add_eval_str in mult_eval.split(' + '):
                add_eval += evaluate(add_eval_str)
            eval *= add_eval
    else:
        parens_open = -1
        parens_count = 0
        for (i, s) in enumerate(string):
            if s == '(':
                parens_count += 1
                if parens_open == -1:
                    parens_open = i
            elif s == ')':
                parens_count -= 1
                if parens_count == 0:
                    eval_string = string[parens_open+1:i]
                    string = string[0:parens_open] + str(evaluate(eval_string)) + string[i+1:]
                    return evaluate(string)
    return eval

tot = 0
for line in file_lines:
    tot += evaluate(line)
print(tot)
import re
with open('./test.txt') as file:
    lines =[re.search(r'(nop|acc|jmp) (\+|-)(\d+)', line.strip()).groups() for line in file.readlines()]

def get_deltas_jump(direction, delta):
    return (0, int(delta) if direction == '+' else -int(delta))

def get_deltas_acc(direction, delta):
    return (int(delta) if direction == '+' else -int(delta), 1)

def execute_line(line, can_switch):
    (operation, direction, delta) = line

    if (operation == 'nop'): 
        switch = get_deltas_jump(direction, delta) if can_switch else False
        return (0, 1, switch)
    elif (operation == 'acc'): return (*get_deltas_acc(direction, delta), False)
    elif (operation == 'jmp'): 
        switch = (0, 1) if can_switch else False
        return (*get_deltas_jump(direction, delta), switch)


i = 0
acc = 0
executed = set()

def spawn_program(i, acc, executed):
    switched_at = None
    switched = set()
    while i < len(lines):
        
        (acc_delta, line_delta, switch) = execute_line(lines[i], i not in switched and not switched_at)
        if switch: 
            switched_at = (i, acc, executed)
            (acc_delta, line_delta) = switch
        else:
            executed.add(i)
        acc += acc_delta
        i += line_delta

        if i in executed and switched_at:
            (i, acc, executed) = switched_at
            switched.add(i)
            switched_at = None
            

    if (i not in executed):
        print(f'Success: {acc}')

spawn_program(0, 0, set())
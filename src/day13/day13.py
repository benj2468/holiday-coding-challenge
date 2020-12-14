from functools import reduce

with open('./input.txt') as file:
    lines = file.readlines()


leave_ts_input = int(lines[0])
buses_iter = lines[1].split(',')

def part_one(leave_ts, buses):
    minimum = None
    for bus in buses:
        if bus == 'x': continue
        bus_ts = int(bus)
        wait_time = bus_ts - (leave_ts % bus_ts)

        if minimum == None:
            minimum = (wait_time, bus_ts)
        if wait_time < minimum[0]:
            minimum = (wait_time, bus_ts)

    print(minimum[0] * minimum[1])

def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod
 
 
 
def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1


def part2(buses):
    n = []
    a = []
    tot = len(buses) + 1
    for (i,bus) in enumerate(buses):
        if bus == 'x':
            continue
        n.append(tot-i)
        a.append(int(bus))
    print(n,a)
    print(chinese_remainder(a,n) - tot)

part2(buses_iter)







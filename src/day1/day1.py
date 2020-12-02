k = 2020

sig = []
with open('src/day1/input.txt') as file:
    sig = list(map(lambda x: int(x), file.read().strip().split('\n')))

sig.sort()

def twos(sig, k):
    p = 0
    q = len(sig) - 1
    while 0 <= p < q < len(sig):
        s = sig[p] + sig[q]
        if s == k:
            return (p, q)
        if s > k:
            q -= 1
            p -= 1
        if s < k or p < 0:
            p += 1


def threes(sig, k):
    for r in range(len(sig)):
        two = twos(sig, k-sig[r])
        if two and not r in two:
            (p, q) = two
            assert(sig[p] + sig[q] + sig[r] == k)
            return (r, p, q)

def display(sig, l):
    vals = list(map(lambda i: sig[i], l))
    prod = 1
    for v in vals: prod *= v
    print(f"Values: {vals}\nProduct: {prod}")

print('Twos:')
double = twos(sig, k)
display(sig, double)
print('\nThrees:')
triple = threes(sig, k)
display(sig, triple)
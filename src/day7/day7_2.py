import re

with open('./input.txt') as file:
    line = file.readline()

    bags = {}
    while line:

        groups = line.strip().split(' ')
        bag = groups[0] + ' ' + groups[1]
        sub_bags = []
        i = 6
        while i + 1 < len(groups):
            sub_bags.append((groups[i-1] + ' ' + groups[i], int(groups[i-2])))
            i += 4
        
        if (sub_bags != ['other bags.']):
            bags[bag] = sub_bags

        line = file.readline()

my_bag = 'shiny gold'
def get_embedded_bags(cur_bag):
    embedded_bags = 0
    for (bag, count) in bags[cur_bag]:
        embedded_bags += count
        embedded_bags += (count * get_embedded_bags(bag))
    return embedded_bags
    

print(get_embedded_bags(my_bag))
        



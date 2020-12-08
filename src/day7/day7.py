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
            sub_bags.append(groups[i-1] + ' ' + groups[i])
            i += 4
        
        if (sub_bags != ['other bags.']):
            for sub_bag in sub_bags:
                if not sub_bag in bags:
                    bags[sub_bag] = []
                bags[sub_bag].append(bag)

        line = file.readline()

my_bag = 'shiny gold'
def get_containers(prev_containers, cur_bag):
    if cur_bag not in bags:
        return set()
    containers = prev_containers
    for bag in bags[cur_bag]:
        if bag not in containers:
            containers.add(bag)
            for sub_bag in get_containers(containers, bag):
                containers.add(sub_bag)
    return containers

print(len(get_containers(set(), my_bag)))


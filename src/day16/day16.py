import sys
from typing import Dict, List, Tuple, Generator
import re

file_name = sys.argv[1]

with open(file_name) as file:
    lines = file.readlines()

class Train:

    def __init__(self):
        self.trainRestriction = {}
        self.nearby_tickets: List[Ticket] = []
        self.my_ticket: Ticket = None
        self.field_order = []

    def get_restrictions(self):
        return self.trainRestriction
    
    def _add_restriction(self, restriction: Dict[str, Tuple[Tuple]]):
        self.trainRestriction = {**self.trainRestriction, **restriction}

    def _add_ticket(self, values: List[int]):
        ticket_obj = Ticket(self, values)
        if not self.my_ticket:
            self.my_ticket = ticket_obj
        else:
            self.nearby_tickets.append(ticket_obj)
    
    def process_line(self, line: str):
        restriction_re = r'(.+): (\d+)-(\d+) or (\d+)-(\d+)'
        ticket_re = r'(\d+),'

        restrictions = re.findall(restriction_re, line)
        if len(restrictions): # Is a restriction line
            for r in restrictions:
                self._add_restriction({
                    r[0]: ((int(r[1]), int(r[2])), (int(r[3]), int(r[4])))
                })
        elif re.match(ticket_re, line):
            self._add_ticket(map(lambda x: int(x), line.strip().split(',')))

    def get_error_rate(self):
        error_rate = 0
        for ticket in self.nearby_tickets:
            error_rate += ticket.check_validity()
        return error_rate

    def calc_field_order(self):
        valid_by_index = {}
        valid_by_key = {}
        true_positions = {}
        for ticket in filter(lambda x: x.check_validity() == 0, self.nearby_tickets):
            ticket_valid_by_index, ticket_valid_by_key = ticket.get_valid_fields()

            for (i, valid_at_index) in enumerate(ticket_valid_by_index):
                if not i in valid_by_index: valid_by_index[i] = valid_at_index
                valid_by_index[i].intersection_update(valid_at_index)

            for (key, valid_at_key) in ticket_valid_by_key.items():
                if not key in valid_by_key: valid_by_key[key] = valid_at_key
                valid_by_key[key].intersection_update(valid_at_key)

        changing = True
        while changing:
            changing = False
            for (i, valid_at_index) in valid_by_index.items():
                if len(valid_by_index[i]) == 1:
                    changing = True
                    key = valid_by_index[i].pop()
                    true_positions[i] = key
                    valid_by_key[key] = set()
                    for value in valid_by_index.values():
                        if key in value: value.remove(key)
            for (key, valid_at_key) in valid_by_key.items():
                if len(valid_by_key[key]) == 1:
                    changing = True
                    position = valid_by_key[key].pop()
                    true_positions[position] = key
                    valid_by_index[position] = set()
                    for value in valid_by_key.values():
                        if position in value: value.remove(position)
        self.field_order = true_positions

    def get_product(self, match_str: str):
        prod = 1

        for (i, field) in self.field_order.items():
            if re.match(match_str, field): 
                prod *= self.my_ticket.get_value_at_index(i)
        return prod



class Ticket:
    def __init__(self, train: Train, values: Generator):
        self.values = list(values)
        self.train = train

    def check_validity(self) -> int:
        for value in self.values:
            valid = False
            for (key, ranges) in self.train.get_restrictions().items():
                for (start, end) in ranges:
                    if value >= start and value <= end:
                        valid = True
            if not valid:
                return value
        return 0

    def get_valid_fields(self):
        valid_by_index = []
        valid_by_key = {}
        for (i, value) in enumerate(self.values):
            valid_at_index = set()
            for (key, ranges) in self.train.get_restrictions().items():
                 for (start, end) in ranges:
                    if value >= start and value <= end:
                        valid_at_index.add(key)
                        if not key in valid_by_key: valid_by_key[key] = set()
                        valid_by_key[key].add(i)
            valid_by_index.append(valid_at_index)
        return valid_by_index, valid_by_key

    def get_value_at_index(self, i: int):
        return self.values[i]

    
train = Train()

for line in lines:
   train.process_line(line)

train.calc_field_order()

print(train.get_product('departure'))

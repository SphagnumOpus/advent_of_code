#/usr/bin/env python3
# Advent of Code 2021 day 7
# Author:  Bill Moss

import argparse
import functools
parser = argparse.ArgumentParser(description="2021 Day 7 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

crab_positions=list(map(int,file.read().split(',')))  

def fuel_cost(a_crab_positions,a_h_position):
    return(sum((list(map(lambda a: abs(a_h_position-a),a_crab_positions)))))

all_costs = [(i,fuel_cost(crab_positions,i)) for i in range(min(crab_positions),max(crab_positions)+1)]
reduced=functools.reduce(lambda a,b: a if a[1]<b[1] else b,all_costs)
print('Part 1 solution is',reduced[1])

fuel_cost_iter=0
fuel_cost_amt=0
fuel_table=[]
for i in range(0,max(crab_positions)+1): 
    fuel_cost_amt+=fuel_cost_iter  
    fuel_table.extend([fuel_cost_amt]) #lookup table for part 2 fuel costs
    fuel_cost_iter+=1

def part2_fuel_cost(a_crab_position,a_h_position):
    return(sum((list(map(lambda a: fuel_table[abs(a_h_position-a)],a_crab_position)))))

all_costs = [(i,part2_fuel_cost(crab_positions,i)) for i in range(min(crab_positions),max(crab_positions)+1)]
reduced=functools.reduce(lambda a,b: a if a[1]<b[1] else b,all_costs)
print('Part 2 solution is',reduced[1])
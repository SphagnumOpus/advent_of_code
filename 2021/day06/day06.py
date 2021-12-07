#/usr/bin/env python3
# Advent of Code 2021 day 6
# Author:  Bill Moss

import argparse
parser = argparse.ArgumentParser(description="2021 Day 6 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

orig_lfish_list=list(map(int,file.read().split(',')))  

orig_lfish_histo=[orig_lfish_list.count(0),
        orig_lfish_list.count(1),
        orig_lfish_list.count(2),
        orig_lfish_list.count(3),
        orig_lfish_list.count(4),
        orig_lfish_list.count(5),
        orig_lfish_list.count(6),
        orig_lfish_list.count(7),
        orig_lfish_list.count(8)]
lfish_histo=orig_lfish_histo
for i in range(80):
    birthing_fish=lfish_histo[0]
    lfish_histo=lfish_histo[1:]
    lfish_histo.extend([birthing_fish])
    lfish_histo[6]+=birthing_fish

print('Part 1 answer is',(sum(lfish_histo)))

lfish_histo=orig_lfish_histo
for i in range(256):
    birthing_fish=lfish_histo[0]
    lfish_histo=lfish_histo[1:]
    lfish_histo.extend([birthing_fish])
    lfish_histo[6]+=birthing_fish

print('Part 2 answer is',(sum(lfish_histo)))



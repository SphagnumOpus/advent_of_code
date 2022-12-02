#/usr/bin/env python3
# Advent of Code 2021 day 8
# Author:  Bill Moss

import argparse
import functools
parser = argparse.ArgumentParser(description="2021 Day 8 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

lines=file.read().splitlines()
parse_input=[y.split(' | ') for y in lines]

def count_1478(string):
    return(sum(map(lambda x:1 if len(x) in [2,3,4,7] else 0,string[1].split())))   #count up the 2,3,4,7 entries

print ('Part 1 answer is:',sum(list(map(count_1478,parse_input))))

char_map = {'abcefg':0,'cf':1,'acdeg':2,'acdfg':3,'bcdf':4,'abdfg':5,'abdefg':6,'acf':7,'abcdefg':8,'abcdfg':9}

def alphabatize(strlist): #takes a list of strings and returns same strings, each one alphabatized
    x=(list(map(lambda x: "".join(sorted(x)),strlist)))
    return x

def without(str,subtract_str):  #return str removing characters in substract_str
    return(str.translate({ord(x): None for x in "".join(subtract_str)}))

def create_xlate(readings):  #given a set of ten readings, create a dictionary translating value to proper value  (not elegant!)
    map={'a':'abcdefg','b':'abcdefg','c':'abcdefg','d':'abcdefg','e':'abcdefg','f':'abcdefg','g':'abcdefg'}   #as we start, all mappings are possible
    element = list(filter(lambda x:len(x) == 2, readings))[0] #only two element reading is a 1... get that knowledge
    map['c']=element
    map['f']=element
    for i in ['a','b','d','e','g']:
        map[i]=without(map[i],element)     #remove elements of 1 from all non-1 segments
    element = list(filter(lambda x:len(x) == 3, readings))[0] #only two element reading is a 7... get that knowledge
    element = without(element,map['c'])     #which element is NOT in one but is in seven?
    map['a']=element
    for i in ['b','d','e','g']:
        map[i]=without(map[i],element)     #remove elements determined to be a from all non-1 non-a segments
    element = list(filter(lambda x:len(x) == 4, readings))[0] #only two element reading is a 4... get that knowledge
    element = without(element,map['f'])   # isolate the elements than could be b or d
    map['b']=element
    map['d']=element
    for i in ['e','g']:
        map[i]=without(map[i],element)    #remove elements determined to be a from all non-1 non-a segments
    element = list(filter(lambda x:len(x) == 6 and not((map['c'][0] in x) and (map['c'][1])in x),readings))[0] #five element without both parts of 1 (number 6)
    if map['c'][0:1] in element:
        map['f']=map['c'][0:1]
        map['c']=map['c'][-1]
    else:
        map['f']=map['c'][-1]
        map['c']=map['c'][0:1]
    element = list(filter(lambda x:len(x) == 6 and not((map['e'][0] in x) and (map['e'][-1])in x),readings))[0] #six element without seg 'e' and 'g' (number 9)
    if map['e'][0:1] in element:
        map['g']=map['e'][0:1]
        map['e']=map['e'][-1]
    else:
        map['g']=map['e'][-1]
        map['e']=map['e'][0:1]
    element = list(filter(lambda x:len(x) == 6 and not((map['d'][0:1] in x) and (map['d'][-1])in x),readings))[0] #six element without seg 'b' and 'd' (number 0)
    if map['d'][0] in element:
        map['b']=map['d'][0:1]
        map['d']=map['d'][-1]
    else:
        map['b']=map['d'][-1]
        map['d']=map['d'][0:1]
    invmap={v:k for k,v in map.items()}   #now invert the map for proper lookup
    return(invmap)

def xlate(strlist,table): #given a set of segment strings, translate them using the translate table
    ret = []
    for i in strlist:
        ret.append("".join([table.get(j) for j in i]))
    return(ret)


def output_digits(arg1):        #takes a single line of parse_input formatted data and returns a list of the four numbers in the readout
    xlate_table = create_xlate(alphabatize(arg1[0].split()))   #creates translation table for crossed segments to correct segment
    #return(list(map(char_map.get,xlate(alphabatize(['cf','abcefg','abcefg','abcefg']),xlate_table))))
    return(list(map(char_map.get,alphabatize(xlate(alphabatize(arg1[1].split()),xlate_table)))))

def output_value(arg1):         #takes a single line of parse_input formatted data and returns the numeric value of the readout
    return(sum(list(map(lambda x: x[1]*10**(3-x[0]),enumerate(output_digits(arg1))))))

print ('part 2 answer is:',sum(list(map(output_value,parse_input))))

exit()

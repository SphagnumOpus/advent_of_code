#/usr/bin/env python3
# Advent of Code 2021 day 10
# Author:  Bill Moss

import argparse
import functools
parser = argparse.ArgumentParser(description="2021 Day 10 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

lines=file.read().splitlines()




close_dict={'(':')','[':']','{':'}','<':'>'}

def syntax_score(line): #create a score for a syntax line given part 1 rules
    global close_dict
    scores={')':3,']':57,'}':1197,'>':25137}
    stack=[]
    for i in line:
        if i in close_dict.keys():
            stack.append(i)
        else:
            if len(stack) > 0:
                opener=stack.pop(-1)
            else:
                return(0)  #incomplete line
            if i in close_dict.values() and i != close_dict[opener]:
                
                return(scores[i],0)
    #score incomplete lines for pt2 purposes
    #calculate part 2 score for this incomplete line
    
    pt2_scores={')':1,']':2,'}':3,'>':4}
    acc=0
    while len(stack)>0:
        acc=acc*5
        acc += pt2_scores[close_dict[stack.pop(-1)]]
    
    return(0,acc)  #reached end of line without an illegal

print('Part 1 answer is:',sum([syntax_score(i)[0] for i in lines]))
pt2_result=sorted(list(filter(lambda x:x>0,[syntax_score(i)[1] for i in lines])))
print('part 2 answer is:',pt2_result[int(len(pt2_result)/2)])
            
exit()

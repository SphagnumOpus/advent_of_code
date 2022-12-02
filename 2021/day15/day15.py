#/usr/bin/env python3
# Advent of Code 2021 day 15
# Author:  Bill Moss

import argparse
import functools
import itertools
from operator import add

parser = argparse.ArgumentParser(description="2021 Day 15 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')


lines=file.read().splitlines()
def make_riskmap(buf):
    #riskmap is:
    #x,y,risk,tot_risk_to_spot,opened_flag,closed_flag     where x,y are the coordinate of the current spot.
    return([[(x,y,int(buf[y][x]),None,False,False)for y in range(len(buf))]for x in range(len(buf[0]))])
def is_node_open(rm,point):
    return(rm[point[0]][point[1]][4])

def is_node_closed(rm,point):
    return(rm[point[0]][point[1]][5])

def open_node(rm,point):
    p=rm[point[0]][point[1]]
    rm[p[0]][p[1]]=(p[0],p[1],p[2],p[3],True,False)
    return()

def close_node(rm,point):
    p=rm[point[0]][point[1]]
    rm[p[0]][p[1]]=(p[0],p[1],p[2],p[3],False,True)
    return()

def risk_to_node(rm,point):
    return(rm[point[0]][point[1]][3])

def node_risk(rm,point):
    return(rm[point[0]][point[1]][2])

def set_risk_to_node_slow(rm,point,value):
    #original slow version of setting risk
    p=rm[point[0]][point[1]]
    rm[p[0]][p[1]]=(p[0],p[1],p[2],value,p[4],p[5])
    return()

def filter_nodes(x):
    return(x[4])
    
def reduce_nodes(x,y):
    return(x if x[3]<y[3] else y)

def adjacent_nodes(rm,point):   #given map and an x and y tuple, list of tuples for legal adjacent points to search
    highx = len(rm[0])-1
    highy = len(rm)-1
    ret=[]
    for i,j in [(0,1),(1,0),(-1,0),(0,-1)]:
        if i+point[0] >= 0 and i+point[0] <= highx:
            if j+point[1]>=0 and j+point[1]<= highy:
                candidate_point=(i+point[0],j+point[1])
                if not is_node_closed(rm,candidate_point):
                    ret.append(candidate_point)
    return(ret)

def tuple_is_open(t):
    return(t[4])

def get_next_node_slow(rm):
    #original get next node... too slow
    open_nodes=list(filter(filter_nodes,functools.reduce(add,rm)))
    lowest_node=list(functools.reduce(reduce_nodes,open_nodes))
    return((lowest_node[0],lowest_node[1]))

def get_lowest_queue_entry(queue):
    item=next(filter(lambda x:len(x)>0,queue))  #get the lowest non-empty item
    ret=item[0]
    queue[ret[3]]=item[1:]
    return(ret)


def get_next_node(rm,queue):
    this_queue_entry=get_lowest_queue_entry(queue)
    while (not this_queue_entry[4]) or this_queue_entry[3] != rm[this_queue_entry[0]][this_queue_entry[1]][3]:
        #keep getting queue entries until we get one that isn't stale (i.e. for a non-open item or from a previous value for the risk path)
        this_queue_entry=get_lowest_queue(queue)
    return((this_queue_entry[0],this_queue_entry[1]))

def add_to_queue(queue,node):
    queue[node[3]].append(node)   #append this to queue sorted by risk value
    return()

def set_risk_to_node(rm,queue,point,value):
    p=rm[point[0]][point[1]]
    rm[p[0]][p[1]]=(p[0],p[1],p[2],value,p[4],p[5])
    add_to_queue(queue,rm[p[0]][p[1]])
    return()

def make_queue(rm):
    return([[] for i in range((len(rm)+len(rm[0]))*9)])



def dijkstra_path(rm,start,end):
    q=make_queue(rm)
    open_node(rm,start)
    set_risk_to_node(rm,q,start,0)

    while not is_node_closed(rm,end):
        current_node = get_next_node(rm,q)
        total_risk=risk_to_node(rm,current_node)
        for i in adjacent_nodes(rm,current_node):
            open_node(rm,i)
            if risk_to_node(rm,i) is None or total_risk+node_risk(rm,i) < risk_to_node(rm,i):
                set_risk_to_node(rm,q,i,total_risk+node_risk(rm,i))
        close_node(rm,current_node)
    return(risk_to_node(rm,end))


            
    return(0)

pt1_riskmap=make_riskmap(lines)
pt1_answer=dijkstra_path(pt1_riskmap,(0,0),(len(pt1_riskmap[0])-1,len(pt1_riskmap)-1))
print('Part 1 answer is ',pt1_answer)



#prepare part 2 version of map
def mapper(c,offset):
    return(['0','1','2','3','4','5','6','7','8','9','1','2','3','4','5','6','7','8','9','1','2','3','4'][c+offset])

def wide_line(buf,offset):    #creates a five wide char buf of integers upped as per part 2 offset 
    ret=""
    for i in range(5):
        for j in buf:
           ret= ret + mapper(int(j),i+offset)
    return(ret)

pt2_lines=[]
for i in range(5):
    for y in range(len(lines)):
        pt2_lines.append(wide_line(lines[y],i))

pt2_riskmap=make_riskmap(pt2_lines)
pt2_answer=dijkstra_path(pt2_riskmap,(0,0),(len(pt2_riskmap[0])-1,len(pt2_riskmap)-1))
print('Part 2 answer is ',pt2_answer)

exit()

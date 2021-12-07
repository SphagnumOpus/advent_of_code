#/usr/bin/env python3
# Advent of Code 2021 day 5
# Author:  Bill Moss

import argparse
parser = argparse.ArgumentParser(description="2021 Day 5 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')


def parse_lines(str_buf):
    p=str_buf.split(' ')
    line_beginning=tuple(map(int,p[0].split(',')))
    line_end=tuple(map(int,p[2].split(',')))
    return (line_beginning,line_end)

line_list=list(map(parse_lines,file.read().splitlines()))  #create a list of all the lines defined as lists of ((x1,y1),(w2,y2)) where x1,y1 is start and x2,y2 is end

def parse_line_list(line_tuple):
    min_x=min(line_tuple[0][0],line_tuple[1][0])
    max_x=max(line_tuple[0][0],line_tuple[1][0])
    min_y=min(line_tuple[0][1],line_tuple[1][1])
    max_y=max(line_tuple[0][1],line_tuple[1][1])
    if min_x == max_x:   #horizontal line
        return([(line_tuple[0][0],i) for i in range (min_y,max_y+1)])
    elif min_y == max_y:   #verticle  line
        return([(i, line_tuple[0][1]) for i in range (min_x,max_x+1)])
    else:  #it's diagonal
        if (line_tuple[0][0]==min_x and line_tuple[0][1] == min_y) or (line_tuple[1][0] == min_x and line_tuple[1][1] == min_y):
            #if min x  min y are both part of same coordinate line slopes UP
            slope = 1
            y_count = min_y
        else:   
            #otherwise line slopes down
            slope = -1
            y_count = max_y
        ret_list=[]
        for i in range (min_x,max_x+1):
            ret_list.extend([(i,y_count)])
            y_count+=slope
        return(ret_list)





lines_as_points_list=list(map(parse_line_list,line_list))  #create lists of  point lists for each line

def add_points_to_dict(a_dict,a_pointlist):
    for i in a_pointlist:
        prev=a_dict.get(i)
        if prev == None:
            a_dict[i]=1
        else:
            a_dict[i]=prev+1

point_count_dict={}
    
def filter_dict(x):
    if x > 1:
        return (True)
    return(False)

for i in lines_as_points_list:
    if (i[0][0] == i[1][0]) or (i[0][1] == i[1][1]):
        add_points_to_dict(point_count_dict,i)   #add the points of horizontal or vertical lines
print ('Part 1 answer is ',len(list(filter(filter_dict,point_count_dict.values()))))
for i in lines_as_points_list:
    if (i[0][0] != i[1][0]) and (i[0][1] != i[1][1]): 
        add_points_to_dict(point_count_dict,i)  #add the points of diagonal lines
print ('Part 2 answer is ',len(list(filter(filter_dict,point_count_dict.values()))))


#/usr/bin/env python3
# Advent of Code 2021 day 22
# Author:  Bill Moss

import argparse
from enum import Enum
import copy

parser = argparse.ArgumentParser(description="2021 Day 22 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')
lines=file.read().splitlines()

class Instr(Enum):
    ON = 1
    OFF = 2

def Reboot_Steps_Create(buf,is_part_1):
    ret=[]
    for i in buf:
        use_new_row=True
        newrow=[]
        ssplit=i.split(" ")
        newrow.append(Instr.ON if ssplit[0]=='on' else Instr.OFF)
        for j in ssplit[1].split(','):
            coords=list(map(int,j.split('=')[1].split('..')))
            if use_new_row and is_part_1:
                coords[0]=max(-50,coords[0])
                coords[1]=min(50,coords[1])
                use_new_row=True if coords[0]<=coords[1] else False   #eliminate items completely outside -50 to 50 for part 1
            newrow.append(coords)
        if use_new_row:
            ret.append(newrow)
    return(ret)

def Reboot_Steps_Item_Process(r,new_item):
    ret=[]
    for i in r:
        this_piece=copy.deepcopy(i)
        They_Overlap=True
        for j in range(1,4):
            They_Overlap=They_Overlap and this_piece[j][0]<=new_item[j][1] and this_piece[j][1]>=new_item[j][0]
        if They_Overlap:
            for j in range(1,4):
                if this_piece[j][0]<new_item[j][0]:
                    new_piece=copy.deepcopy(this_piece)
                    new_piece[j][1]=new_item[j][0]-1
                    this_piece[j][0]=new_item[j][0]
                    ret.append(new_piece)
                if this_piece[j][1]>new_item[j][1]:
                    new_piece=copy.deepcopy(this_piece)
                    new_piece[j][0]=new_item[j][1]+1
                    this_piece[j][1]=new_item[j][1]
                    ret.append(new_piece)
        else:
            ret.append(this_piece)
    #we've processed the prev_item down until it is equal to or smaller than the current new item.  Just output the new item if it ISN'T an off.
    if new_item[0] is Instr.ON:
        ret.append(new_item.copy())
    return(ret)

def Reboot_Steps_Process(s):
    return_list=[]
    return_list.append(s[0])   #start with return_list being the first item
    for i in range(1,len(s)):   #process each subsequent item against the current return list
        return_list=Reboot_Steps_Item_Process(return_list,s[i])
    return(return_list)

def Reboot_Steps_Item_Volume(item):
    return((item[1][1]-item[1][0]+1)*(item[2][1]-item[2][0]+1)*(item[3][1]-item[3][0]+1))

It_Is_Part_1=True
for i in range(1,3):
    answer=sum(map(Reboot_Steps_Item_Volume,Reboot_Steps_Process(Reboot_Steps_Create(lines,It_Is_Part_1))))
    print('Part ',i,' answer is',answer)
    It_Is_Part_1=False
exit()
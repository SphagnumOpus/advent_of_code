#/usr/bin/env python3 
import argparse
parser = argparse.ArgumentParser(description="2021 Day 2 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
args = parser.parse_args()

file = open (args.file,'r')

raw_array = list(file.read().splitlines())

def compute_tuple(x):
    if x.split()[0] == 'forward': 
        ret = (int(x.split()[1]),0)
    elif x.split()[0] == 'up':
        ret = (0,int(x.split()[1]))
    elif x.split()[0] == 'down':
        ret = (0,-1 * int(x.split()[1]))
    else:
        print('Error... invalid direction ',x.split()[0])
        exit(1)
    return(ret)

tuple_array = [compute_tuple(y) for y in raw_array]
sum_array = [sum(i) for i in zip(*tuple_array)]

print ('Part 1 answer is',sum_array[0]*sum_array[1] * -1) #multiply by -1 because we want DEPTH

cur_pos=(0,0)
cur_aim=0
for i in tuple_array:
    cur_aim+=i[1]
    cur_pos=(cur_pos[0]+i[0],cur_pos[1]+(i[0]*cur_aim))


print ('Part 2 answer is',cur_pos[0]*cur_pos[1] * -1) #multiply by -1 because we want DEPTH





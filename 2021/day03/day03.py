#/usr/bin/env python3

import argparse
parser = argparse.ArgumentParser(description="2021 Day 3 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')

def split_chars(x):
    return [int(char) for char in x]

tuple_array = list(map (split_chars,file.read().splitlines()))  #create list of ints with 0/1 values
if args.debug:
    print ('tuple_array =',tuple_array)

def create_sum_array(x):
    return([sum(i) for i in zip(*x)]) #sum up the 1/0 numbers in each position of the list

sum_array = create_sum_array(tuple_array)
if args.debug:
    print('sum_array = ',sum_array)

list_len=len(tuple_array)
if args.debug:
    print('list_len = ',list_len)

gamma_rate = 0
epsilon_rate = 0

power_of_2 = int(1<<(len(sum_array) - 1))
for i in sum_array:
    if i > (list_len/2):
        gamma_rate += power_of_2
    else:
        epsilon_rate += power_of_2
    power_of_2 = power_of_2>>1

if args.debug:
    print('gamma_rate =',gamma_rate,' epsilon rate = ',epsilon_rate)

print('Part 1 answer is', gamma_rate * epsilon_rate)


def narrow_down(in_tuple,in_index,in_flag):
    if len(in_tuple) == 1:
            return(in_tuple)
    else:
            new_sum_array = create_sum_array(in_tuple)
            #print('in_index =', in_index)
            #print('new_sum_array[in_index]=',new_sum_array[in_index])
            if new_sum_array[in_index] >= (len(in_tuple)/2):
                current_majority = 1
            else:
                current_majority = 0

            new_tuple = []
            for i in in_tuple:
                if (current_majority == i[in_index]) == in_flag:
                    new_tuple.append(i)
            #print('return values',narrow_down(new_tuple,in_index+1,in_flag))
            return(narrow_down(new_tuple,in_index+1,in_flag))

oxy_tuple = narrow_down(tuple_array,0,True)[0]
co2_tuple = narrow_down(tuple_array,0,False)[0]

oxy_sensor = 0
co2_sensor = 0
power_of_2 = int(1<<(len(oxy_tuple) - 1))
for i in range (len(oxy_tuple)):
    oxy_sensor += oxy_tuple[i] * power_of_2 
    co2_sensor += co2_tuple[i] * power_of_2 
    power_of_2 = power_of_2>>1

print ('Part 2 answer is', oxy_sensor * co2_sensor)

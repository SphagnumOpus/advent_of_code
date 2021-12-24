#/usr/bin/env python3
# Advent of Code 2021 day 21
# Author:  Bill Moss

import argparse
import functools
import itertools


parser = argparse.ArgumentParser(description="2021 Day 21 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')






lines=file.read().splitlines()

starting_game=[]
for i in lines:
    starting_game.append([int(i.split()[4]),0])

starting_game.append(0)  #second element is whose turn is next



def die_create():
    return([0,0])  #deterministic die starts with no rolls and on poised to roll a 1

def die_roll(d):
    d[0]+=1
    d[1]+=1
    if d[0]>100:
        d[0]-=100
    return(d[0])

ddie = die_create()

def next_turn(x):
    return((x+1)%2)

def player_score(g,x):
    return(g[x][1])

def take_a_turn(x,y):
    next_space=(((x[0]+y)-1)%10)+1
    return([next_space,x[1]+next_space])

def die_roll_count(x):
    return(x[1])

def game_set_whose_turn(g,x):
    game[2]=x
    return()

def game_get_turn(g):
    return(g[2])

def game_take_a_turn(g,r):
    ret=g.copy()
    ret[g[2]]= take_a_turn(g[g[2]],r)
    ret[2]=next_turn(g[2])
    return(ret)

def game_leading_score(g):
    return(max(g[0][1],g[1][1]))

def game_winning_player(g):
    return(0 if g[0][1]>g[1][1] else 1)

def game_losing_player(g):
    return(0 if g[0][1]<g[1][1] else 1)

game=starting_game.copy()
game_set_whose_turn(game,0)  #two players take turns  we set up to be poised for first move to be player 0

while game_leading_score(game) < 1000:
    game=game_take_a_turn(game,die_roll(ddie)+die_roll(ddie)+die_roll(ddie))

print ('Part 1 answer is', player_score(game,game_losing_player(game)) * die_roll_count(ddie))

#first get the dice roll distributions for three 3 sided dice rolls into gauss_tbl
gauss_tbl={}
for i in range(1,4):
    for j in range(1,4):
        for k in range(1,4):
            gauss_tbl[i+j+k]=gauss_tbl.setdefault(i+j+k,0)+1


game=starting_game.copy()   #start another game

def play_quantum_game(t,g):
    ret = [0,0]
    if game_leading_score(g) >= 21:
        ret[game_winning_player(g)]+=1
    else:
        for roll,num_of_universes in t.items():
            tmp_result=play_quantum_game(t,game_take_a_turn(g,roll))
            ret= [(tmp_result[i]*num_of_universes)+ret[i] for i in range(2)]
    return(ret)


        


outcomes=play_quantum_game(gauss_tbl,game)
print('Part 2 answer is',max(outcomes[0],outcomes[1]))




exit()


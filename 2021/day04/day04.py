#/usr/bin/env python3

import argparse
parser = argparse.ArgumentParser(description="2021 Day 4 Solution")
parser.add_argument("-f", "--file", type=str, help="input file")
parser.add_argument("-d", "--debug", help="debug mode",action="store_true") 
args = parser.parse_args()

file = open (args.file,'r')


file_string_list=file.read().splitlines()
drawn_numbers_list=list(map(int,file_string_list[0].split(',')))

board_list=[]
board_match_list=[]
def build_boards(binput,blist,bmatches):  #build the two board structures from the file input format
    board_number=-1
    for bline in binput:
        if bline=='':
            board_number+=1
            blist.append([])
            bmatches.append([0 for i in range(11)]) #ten for rows and columns, last element to hold sum of matched numbers
        else:
            blist[board_number].extend(list(map(int,bline.split())))
    return()

build_boards(file_string_list[1:],board_list,board_match_list)

def find_drawn_number(blist,bmatches,drawn_number,bwonlist):
    return_value = -1
    for i in range(0,len(blist)):
        if not bwonlist[i]:
            if (drawn_number in blist[i]) and (not bwonlist[i]):
                this_index=blist[i].index(drawn_number)
                bmatches[i][10]+=drawn_number   #add found number to list of squares not to count at end
                bmatches[i][5 + (this_index%5)] += 1      #row it's in
                bmatches[i][int(this_index/5)] += 1      #col it's in
                if max(bmatches[i][:-1]) == 5:
                    bwonlist[i]=True
                    return_value = i
    return(return_value)


number_of_winners = 0
board_won_list=[]
board_won_list.extend([False for i in range(len(board_list))])
for i in range(0,len(drawn_numbers_list)):
    winning_board = find_drawn_number(board_list,board_match_list,drawn_numbers_list[i],board_won_list) 
    if winning_board >= 0: #and (not board_won_list[winning_board]):
        number_of_winners += 1
        #found the number
        if number_of_winners == 1:
            print ('Part 1 answer is ',drawn_numbers_list[i] * (sum(board_list[winning_board]) - board_match_list[winning_board][10]))
        else:
            if not(False in board_won_list):
            #if number_of_winners == len(board_won_list):
                print ('Part 2 answer is ',drawn_numbers_list[i] * (sum(board_list[winning_board]) - board_match_list[winning_board][10]))
                break






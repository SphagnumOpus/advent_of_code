// Advent of code 2023 day 02
// Author:  Bill Moss
using System;


namespace Day02NS
{

    struct Bag {
        public int[] cubes;

        public Bag(){
            cubes=new int[3];
        }
    };
    struct Game {
        public int gameNum;
        public List<Bag> bagList;
        public Game(){
            bagList=new List<Bag>();
        }
    };

    class Day02
    {
        static string[] ColorNames= {"red","green","blue"};
        static int[] Part1Bag={12,13,14};
        static void Main(string[] args)
        {
            //Assume argv[1] is a filename and open it, and read it into a buffer
            List<string> inputBuffer = new List<string>();
            List<Game> gameList = new List<Game>();
            using (var inputFile= new StreamReader(args[0]))
            {
                string? buf = inputFile.ReadLine();
                while(buf is not null)
                {
                    inputBuffer.Add(buf);
                    gameList.Add(buildGame(buf));
                    buf = inputFile.ReadLine();
                }
            }

            int part1Sum=0;
            int part2Sum=0;
            foreach (var t in gameList){
                part1Sum += PlayPart1(t);
                part2Sum += PlayPart2(t);
            }
            System.Console.Write("Part 1 answer is:");
            System.Console.WriteLine(part1Sum);
            System.Console.Write("Part 2 answer is:");
            System.Console.WriteLine(part2Sum);

        }

        static int PlayPart1(Game inGame){
            foreach (var t in inGame.bagList){
                foreach (var (x,y) in t.cubes.Zip(Part1Bag)){
                    if (x>y) {
                        return 0;
                    }                   
                }
            }
            return inGame.gameNum;
        }

        static int PlayPart2(Game inGame){
            Bag maxbag=new Bag();
            foreach (var t in inGame.bagList){
                for (int i=0;i<3;i++){
                    if (t.cubes[i]>maxbag.cubes[i]){
                        maxbag.cubes[i]=t.cubes[i];
                    }
                }
                
            }
            return maxbag.cubes[0]*maxbag.cubes[1]*maxbag.cubes[2];
        }
        static Game buildGame(string inString){
            Game returnVal = new Game();
            string[] GameSection=inString.Split(':');
            string[] GameSectionSplit=GameSection[0].Split(' ');
            returnVal.gameNum=Int32.Parse(GameSectionSplit[1]);
            foreach (var x in GameSection[1].Split(';')){
                Bag b=new Bag();
                foreach (var y in x.Split(',')){
                    string[] z=y.Split(' ',StringSplitOptions.RemoveEmptyEntries);
                    int color = Array.FindIndex(ColorNames, x => x.Contains(z[1]));
                    b.cubes[color] += Int32.Parse(z[0]);
                }
                returnVal.bagList.Add(b);
            }
            return returnVal;
        }      
    }
}
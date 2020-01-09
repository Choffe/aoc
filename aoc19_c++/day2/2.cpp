#include <iostream>
#include <iomanip>
#include <fstream>

using namespace std;
int main()
{
    int x;
    int program[1000];
    int original[1000];
    ifstream inFile;
    char tmp[120];
    inFile.open("input.txt");

    if (!inFile) {
        cout << "Unable to open file\n";
        exit(1);
    }
    int total_length = 0;
    while (inFile.getline(tmp, 1000, ',')) {
        original[total_length++] = stoi(tmp);
    }
    inFile.close();

    for (int noun = 0; noun <= 99; noun++) {
        for (int verb = 0; verb <= 99; verb++) {
                for (int i = 0; i < total_length; i++) {
                    program[i] = original[i];
                }
                program[1] = noun;
                program[2] = verb;

                int pc = 0;
                while (pc < total_length) {
                    int op = program[pc];
                    if (op == 99) {
                        break;
                    }
                    int res;
                    switch (op) {
                        case 1:
                        program[program[pc+3]] = program[program[pc+2]] + program[program[pc+1]];
                        pc+= 4;
                        break;
                        case 2:
                        program[program[pc+3]] = program[program[pc+2]] * program[program[pc+1]];
                        pc+= 4;
                        break;
                    }
                }
                if (program[0] == 19690720) {
                    cout << "DONE\n";
                    cout << noun * 100 + verb << endl;
                    exit(0);
                }

        }
    }
    cout << "failed";
    cout << program[0];
    cout << endl;
    return 0;
}
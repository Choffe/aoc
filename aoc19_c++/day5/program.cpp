#include <iostream>
#include <iomanip>
#include <fstream>

using namespace std;
int main()
{
    int x;
    int program[1000] = {0};
    int original[1000] = {0};
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

    for (int i = 0; i < total_length; i++) {
        program[i] = original[i];
    }
    int p[5];

    int input_array[10] = {0};
    input_array[0] = 5;
    int output_array[1000] = {0};
    int input_index = 0;
    int output_index = 0;

    int pc = 0;
    while (pc < total_length) {
        int op = program[pc] % 100;
        p[0] = (program[pc] / 100) % 10;
        p[1] = (program[pc] / 1000) % 10;
        p[2] = (program[pc] / 10000) % 10;
        p[3] = (program[pc] / 100000) % 10;
        cout << op << endl;;
        if (op == 99) {
            break;
        }
        switch (op) {
            case 1:
            // cout << (!p[1] ? program[program[pc+2]] : program[pc+2]) << " + " << (!p[0] ? program[program[pc+1]] : program[pc+1]) << endl;
            program[program[pc+3]] = (!p[1] ? program[program[pc+2]] : program[pc+2]) + (!p[0] ? program[program[pc+1]] : program[pc+1]);
            pc+= 4;
            break;
            case 2:
            program[program[pc+3]] = (!p[1] ? program[program[pc+2]] : program[pc+2]) * (!p[0] ? program[program[pc+1]] : program[pc+1]);
            pc+= 4;
            break;
            case 3:
            program[program[pc+1]] = input_array[input_index++];
            // cout << "input : " << program[pc+1] << endl;
            pc+= 2;
            break;
            case 4:
            output_array[output_index++] = (!p[0] ? program[program[pc+1]] : program[pc+1] );
            cout << "output : " << output_array[output_index-1] << endl;
            pc+= 2;
            break;
            case 5:
            if ((!p[0] ? program[program[pc+1]] : program[pc+1]) != 0) {
                pc = (!p[1] ? program[program[pc+2]] : program[pc+2]);
            } else {
                pc += 3;
            }
            break;
            case 6:
            if ((!p[0] ? program[program[pc+1]] : program[pc+1]) == 0) {
                pc = (!p[1] ? program[program[pc+2]] : program[pc+2]);
            } else {
                pc += 3;
            }
            break;
            case 7:
            if ((!p[0] ? program[program[pc+1]] : program[pc+1]) < (!p[1] ? program[program[pc+2]] : program[pc+2])) {
                program[program[pc+3]] = 1;
            } else {
                program[program[pc+3]] = 0;
            }
            pc += 4;
            break;
            case 8:
            if ((!p[0] ? program[program[pc+1]] : program[pc+1]) == (!p[1] ? program[program[pc+2]] : program[pc+2])) {
                program[program[pc+3]] = 1;
            } else {
                program[program[pc+3]] = 0;
            }
            pc += 4;
            break;
            default :
            cout << "op " << op  <<" default, exiting" << endl;
            pc = total_length;
            break;
        }
    }

    cout << "done ";
    if (output_index > 0 ) {
        cout << output_array[output_index - 1];
    }
    cout << endl;
    return 0;
}
#include <iostream>
#include <iomanip>
#include <fstream>

using namespace std;

class Computer {
    public:
    long long int program[10000] = {0};
    long long int input_array[10000] = {0};
    long long int input_index = 0;
    long long int write_input_index = 0;
    long long int total_length = 0;
    long long int pc = 0;
    long long int output_array[10000] = {0};
    long long int output_index = 0;
    long long int relative_base = 0;

    Computer(long long int original[10000], long long int length) {
        total_length = length;
        for (int i = 0; i < total_length; i++) {
            program[i] = original[i];
        }
    }

    long long int run() {
        int p[5];
        long long int first = 0;
        long long int second = 0;
        while (pc < total_length) {
            int op = program[pc] % 100;
            p[0] = (program[pc] / 100) % 10;
            p[1] = (program[pc] / 1000) % 10;
            p[2] = (program[pc] / 10000) % 10;
            p[3] = (program[pc] / 100000) % 10;
            // cout << "op " << op << endl;
            // cout << "instruction : " << program[pc] << endl;
            if (op == 99) {
                break;
            }
            switch (op) {
                case 1:
                // cout << (!p[1] ? program[program[pc+2]] : program[pc+2]) << " + " << (!p[0] ? program[program[pc+1]] : program[pc+1]) << endl;
                if (p[0] == 0) {
                    first = program[program[pc+1]];
                } else if (p[0] == 1) {
                    first = program[pc+1];
                } else if (p[0] == 2) {
                    first = program[program[pc+1] + relative_base];
                }
                if (p[1] == 0) {
                    second = program[program[pc+2]];
                } else if (p[1] == 1) {
                    second = program[pc+2];
                } else if (p[1] == 2) {
                    second = program[program[pc+2] + relative_base];
                }
                if (p[2] == 0) {
                    program[program[pc+3]] = first + second;
                } else {
                    program[program[pc+3]+relative_base] = first + second;
                }
                pc+= 4;
                break;
                case 2:
                if (p[0] == 0) {
                    first = program[program[pc+1]];
                } else if (p[0] == 1) {
                    first = program[pc+1];
                } else if (p[0] == 2) {
                    first = program[program[pc+1] + relative_base];
                }
                if (p[1] == 0) {
                    second = program[program[pc+2]];
                } else if (p[1] == 1) {
                    second = program[pc+2];
                } else if (p[1] == 2) {
                    second = program[program[pc+2] + relative_base];
                }
                if (p[2] == 0) {
                    program[program[pc+3]] = first * second;
                } else {
                    program[program[pc+3]+relative_base] = first * second;
                }
                pc+= 4;
                break;
                case 3:
                if (p[0] == 0) {
                    program[program[pc+1]] =  input_array[input_index++];
                } else {
                    program[program[pc+1] + relative_base] = input_array[input_index++];
                }
                cout << "input : " << input_array[input_index-1] << endl;
                pc+= 2;
                break;
                case 4:
                if (p[0] == 0) {
                    first = program[program[pc+1]];
                } else if (p[0] == 1) {
                    first = program[pc+1];
                } else if (p[0] == 2) {
                    first = program[program[pc+1] + relative_base];
                }
                output_array[output_index++] = first;
                cout << "output : " << output_array[output_index-1] << endl;
                pc+= 2;
                break;
                case 5:
                if (p[0] == 0) {
                    first = program[program[pc+1]];
                } else if (p[0] == 1) {
                    first = program[pc+1];
                } else if (p[0] == 2) {
                    first = program[program[pc+1] + relative_base];
                }
                if (p[1] == 0) {
                    second = program[program[pc+2]];
                } else if (p[1] == 1) {
                    second = program[pc+2];
                } else if (p[1] == 2) {
                    second = program[program[pc+2] + relative_base];
                }
                if (first != 0) {
                    pc = second;
                } else {
                    pc += 3;
                }
                break;
                case 6:
                if (p[0] == 0) {
                    first = program[program[pc+1]];
                } else if (p[0] == 1) {
                    first = program[pc+1];
                } else if (p[0] == 2) {
                    first = program[program[pc+1] + relative_base];
                }
                if (p[1] == 0) {
                    second = program[program[pc+2]];
                } else if (p[1] == 1) {
                    second = program[pc+2];
                } else if (p[1] == 2) {
                    second = program[program[pc+2] + relative_base];
                }
                if (first == 0) {
                    pc = second;
                } else {
                    pc += 3;
                }
                break;
                case 7:
                if (p[0] == 0) {
                    first = program[program[pc+1]];
                } else if (p[0] == 1) {
                    first = program[pc+1];
                } else if (p[0] == 2) {
                    first = program[program[pc+1] + relative_base];
                }
                if (p[1] == 0) {
                    second = program[program[pc+2]];
                } else if (p[1] == 1) {
                    second = program[pc+2];
                } else if (p[1] == 2) {
                    second = program[program[pc+2] + relative_base];
                }
                if (first < second) {
                    if (p[2] == 0) {
                        program[program[pc+3]] = 1;
                    } else {
                        program[program[pc+3]+relative_base] = 1;
                    }
                } else {
                    if (p[2] == 0) {
                        program[program[pc+3]] = 0;
                    } else {
                        program[program[pc+3]+relative_base] = 0;
                    }
                }
                pc += 4;
                break;
                case 8:
                if (p[0] == 0) {
                    first = program[program[pc+1]];
                } else if (p[0] == 1) {
                    first = program[pc+1];
                } else if (p[0] == 2) {
                    first = program[program[pc+1] + relative_base];
                }
                if (p[1] == 0) {
                    second = program[program[pc+2]];
                } else if (p[1] == 1) {
                    second = program[pc+2];
                } else if (p[1] == 2) {
                    second = program[program[pc+2] + relative_base];
                }
                // cout << first << " == " << second << endl;
                if (first == second) {
                    if (p[2] == 0) {
                        program[program[pc+3]] = 1;
                    } else {
                        program[program[pc+3]+relative_base] = 1;
                    }
                } else {
                    if (p[2] == 0) {
                        program[program[pc+3]] = 0;
                    } else {
                        program[program[pc+3]+relative_base] = 0;
                    }
                }
                pc += 4;
                break;
                case 9:
                if (p[0] == 0) {
                    first = program[program[pc+1]];
                } else if (p[0] == 1) {
                    first = program[pc+1];
                } else if (p[0] == 2) {
                    first = program[program[pc+1] + relative_base];
                }
                relative_base += first;
                pc += 2;
                break;
                default :
                cout << "op " << op  <<" default, exiting" << endl;
                pc = total_length;
                break;
            }
        }

        // if (output_index > 0 ) {
        //     cout << output_array[output_index - 1];
        // }
        // cout << output_array[0] << endl;
        // input_array[input_index + 1] = output_array[0];
        // output_index = 0;

        return output_array[output_index - 1];
    }
};


int main()
{

    long long int x;
    // long long int program[1000] = {0};
    long long int original[10000] = {0};
    ifstream inFile;
    char tmp[120];
    inFile.open("input.txt");

    if (!inFile) {
        cout << "Unable to open file\n";
        exit(1);
    }
    long long int total_length = 0;
    while (inFile.getline(tmp, 10000, ',')) {
        original[total_length++] = stoi(tmp);
    }
    inFile.close();


    Computer boost(original, total_length);

    boost.input_array[0] = 2;
    boost.run();

    cout << "done";
    cout << endl;
    return 0;
}
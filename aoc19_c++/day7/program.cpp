#include <iostream>
#include <iomanip>
#include <fstream>

using namespace std;

class Amp {
    public:
    int program[1000] = {0};
    int input_array[1000] = {0};
    int input_index = 0;
    int write_input_index = 0;
    int total_length = 0;
    Amp *next_amp = NULL;
    int pc = 0;
    int output_array[1000] = {0};
    int output_index = 0;

    Amp(int original[1000], int length, int phase, Amp *next) {
        input_array[write_input_index++] = phase;
        total_length = length;
        for (int i = 0; i < total_length; i++) {
            program[i] = original[i];
        }
        next_amp = next;
    }

    int run() {
        int p[5];

        while (pc < total_length) {
            int op = program[pc] % 100;
            p[0] = (program[pc] / 100) % 10;
            p[1] = (program[pc] / 1000) % 10;
            p[2] = (program[pc] / 10000) % 10;
            p[3] = (program[pc] / 100000) % 10;
            // cout << op << endl;
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
                // program[program[pc+1]] = input;
                program[program[pc+1]] = input_array[input_index++];
                // cout << "input : " << input_array[input_index-1] << endl;
                pc+= 2;
                if (input_index > write_input_index) {
                    cout << "reading unwritten data " << endl;
                }
                break;
                case 4:
                output_array[output_index++] = (!p[0] ? program[program[pc+1]] : program[pc+1] );
                // cout << "output : " << output_array[output_index-1] << endl;
                pc+= 2;
                next_amp->input_array[next_amp->write_input_index++] = output_array[output_index-1];
                next_amp->run();
                // next_amp->run((!p[0] ? program[program[pc+1]] : program[pc+1] ));
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

        // if (output_index > 0 ) {
        //     cout << output_array[output_index - 1];
        // }
        // cout << output_array[0] << endl;
        // input_array[input_index + 1] = output_array[0];
        // output_index = 0;

        return output_array[output_index - 1];
    }
};

int thrust(int original[1000], int total_length, int input_array[10]) {

    int program[1000] = {0};
    int output_array[10] = {0};
    int input_index = 0;
    int output_index = 0;

    for (int amp = 0; amp < 5; amp++) {
        for (int i = 0; i < total_length; i++) {
            program[i] = original[i];
        }
        int p[5];


        int pc = 0;
        while (pc < total_length) {
            int op = program[pc] % 100;
            p[0] = (program[pc] / 100) % 10;
            p[1] = (program[pc] / 1000) % 10;
            p[2] = (program[pc] / 10000) % 10;
            p[3] = (program[pc] / 100000) % 10;
            // cout << op << endl;
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
                // cout << "input : " << input_array[input_index-1] << endl;
                pc+= 2;
                break;
                case 4:
                output_array[output_index++] = (!p[0] ? program[program[pc+1]] : program[pc+1] );
                // cout << "output : " << output_array[output_index-1] << endl;
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

        // if (output_index > 0 ) {
        //     cout << output_array[output_index - 1];
        // }
        // cout << output_array[0] << endl;
        input_array[input_index + 1] = output_array[0];
        output_index = 0;
    }
    return output_array[0];
}


int main()
{
    int x;
    // int program[1000] = {0};
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

    int max_output = 0;
    int max_input = 0;
    for (int i = 5; i < 10; i++) {
        for (int j = 5; j < 10; j++) {
            for (int k = 5; k < 10; k++) {
                for (int l = 5; l < 10; l++) {
                    for (int m = 5; m < 10; m++) {
                        if (i == j || i == k || i == l || i == m) {
                            continue;
                        }
                        if (j == k || j == l || j == m) {
                            continue;
                        }
                        if (k == l || k == m) {
                            continue;
                        }
                        if (l == m) {
                            continue;
                        }
                        // int input_array[10] = {0};
                        // input_array[0] = i;
                        // input_array[2] = j;
                        // input_array[4] = k;
                        // input_array[6] = l;
                        // input_array[8] = m;
                        // int output = thrust(original, total_length, input_array);
                        // if (output > max_output) {
                        //     max_output = output;
                        //     max_input = 10000* i + 1000*j + 100 * k + 10 * l + m;
                        // }
                        Amp e(original, total_length, m, NULL);
                        Amp d(original, total_length, l, &e);
                        Amp c(original, total_length, k, &d);
                        Amp b(original, total_length, j, &c);
                        Amp a(original, total_length, i, &b);
                        e.next_amp = &a;
                        a.input_array[a.write_input_index++] = 0;
                        a.run();
                        int output = e.output_array[e.output_index-1];
                        if (output > max_output) {
                            max_output = output;
                            max_input = 10000* i + 1000*j + 100 * k + 10 * l + m;
                        }

                    }
                }
            }
        }
    }

    cout << "done\noutput: " << max_output << endl << "phase: " << max_input;
    cout << endl;
    return 0;
}
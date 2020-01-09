#include <iostream>
#include <iomanip>
#include <fstream>
#include <vector>

using namespace std;

enum {
    UP,
    RIGHT,
    DOWN,
    LEFT,
};
enum {
    EMPTY,
    WALL,
    BLOCK,
    HORIZONTAL_PADDLE,
    BALL
};

class Node {
    public :
    int x = 0;
    int y = 0;
    int tile = EMPTY;

    Node(int _x, int _y, int _tile) {
        x = _x;
        y = _y;
        tile = _tile;
    }

    int equals(Node other){
        return (x == other.x && y == other.y);
    }
};

class Computer {
    public:
    long long int program[10000] = {0};
    long long int input_array[10000] = {0,0,0,0,-1,-1,-1,-1,-1};
    long long int input_index = 0;
    long long int write_input_index = 0;
    long long int total_length = 0;
    long long int pc = 0;
    long long int output_array[10000] = {0};
    long long int output_index = 0;
    long long int relative_base = 0;
    int x_coord = 0;
    int y_coord = 0;
    int direction = UP;
    static const int height = 20;
    static const int width = 40;
    int screen[height][width] = {0};
    int slow = 0;


    Computer(long long int original[10000], long long int length) {
        total_length = length;
        for (int i = 0; i < total_length; i++) {
            program[i] = original[i];
        }
        int t = 67;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        t = 99;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t = 119;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t = 145;
        input_array[t++] = -1;
        t = 195;
        input_array[t++] = 1;
        input_array[t++] = 1;
        t = 300;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t = 400;
        input_array[t++] = 1;
        input_array[t++] = 1;
        t = 450;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t = 530;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        t = 590;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t = 650;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;

        t = 680;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        t = 700;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t = 780;
        input_array[t++] = 1;
        input_array[t++] = 1;
        t = 850;
        input_array[t++] = 1;
        input_array[t++] = 1;
        t = 950;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;

        t = 1050;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;

        t = 1190;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;

        t = 1263;
        for (int i = 0; i < 25; i++)
            input_array[t++] = -1;
        t = 1320;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t = 1340;
        for (int i = 0; i < 25; i++)
            input_array[t++] = 1;


        t = 1371;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;

        t = 1390;
        for (int i = 0; i < 20; i++)
            input_array[t++] = -1;
    
        t = 1421;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;
        input_array[t++] = -1;

        t+= 5;
        input_array[t++] = 1;
        input_array[t++] = 1;
        input_array[t++] = 1;

        t+= 15;
        for (int i = 0; i < 17; i++)
            input_array[t++] = 1;
        t= 1485;
        for (int i = 0; i < 18; i++)
            input_array[t++] = -1;
        t= 1541;
        for (int i = 0; i < 18; i++)
            input_array[t++] = 1;
        t= 1571;
        for (int i = 0; i < 10; i++)
            input_array[t++] = 1;
        for (int i = 0; i < 26; i++)
            input_array[t++] = -1;
        t= 1611;
        input_array[t++] = 1;
        t= 1641;
        for (int i = 0; i < 13; i++)
            input_array[t++] = 1;
        t= 1670;
        for (int i = 0; i < 4; i++)
            input_array[t++] = -1;
        t= 1700;
        for (int i = 0; i < 15; i++)
            input_array[t++] = 1;
        t= 1800;
        for (int i = 0; i < 25; i++)
            input_array[t++] = -1;
        t= 1840;
        for (int i = 0; i < 14; i++)
            input_array[t++] = 1;
        t= 1890;
        for (int i = 0; i < 4; i++)
            input_array[t++] = -1;
        t= 1930;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t= 2020;
        input_array[t++] = -1;
        input_array[t++] = -1;
        t= 2040;
        for (int i = 0; i < 12; i++)
            input_array[t++] = 1;
        t= 2060;
        for (int i = 0; i < 22; i++)
            input_array[t++] = -1;

        slow = 2060;


    }
    void print_screen(void) {
        cout << "Move index " << input_index << endl;

        for(int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                if (screen[y][x] == BALL) {
                    cout << 'O';
                } else if (screen[y][x] == EMPTY) {
                    cout << ' ';
                } else if (screen[y][x] == WALL) {
                    cout << '#';
                } else if (screen[y][x] == HORIZONTAL_PADDLE) {
                    cout << '_';
                } else if (screen[y][x] == BLOCK) {
                    cout << 'x';
                }
            }
            cout << endl;
        }
    }

    long long int run() {
        int p[5];
        long long int first = 0;
        long long int second = 0;
        int output = 0;
        int _x = 0, _y = 0, _tile = 0;
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
                // input_array[input_index] = 0;
                // for (Node &node : hull) {
                //     if (node.x == x_coord && node.y == y_coord) {
                //         input_array[input_index] = node.color;
                //     }
                // }
                print_screen();
                if (input_index > slow) {
                    char c;
                    cin.get(c);
                }
                // if (c == 'j') {
                //     input_array[input_index] = -1;
                // } else if (c == 'l') {
                //     input_array[input_index] = 1;
                // } else {
                //     input_array[input_index] = 0;
                // }
                if (p[0] == 0) {
                    program[program[pc+1]] = input_array[input_index++];
                } else {
                    program[program[pc+1] + relative_base] = input_array[input_index++];
                }
                // cout << "input : " << input_array[input_index-1] << endl;
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
                if (output_index >= 3) {
                    if (output_array[0] == -1 && output_array[1] == 0) {
                        cout << "Player score is " << output_array[2] << endl;
                    }else {
                        if (output_array[0] < 0 || output_array[0] >= 50) {
                            cout << "error " <<endl;
                            return -1;
                        }
                        if (output_array[1] < 0 || output_array[2] >= 50) {
                            cout << "error " <<endl;
                            return -1;
                        }
                        if (output_array[1] < 0 || output_array[1] >= 50) {
                            cout << "error " <<endl;
                            return -1;
                        }
                        screen[output_array[1]][output_array[0]] = output_array[2];
                    }
                    output_index = 0;
                    // print_screen();
                }
                // cout << "output : " << output_array[output_index-1] << endl;

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
        return 0;
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
        original[total_length++] = stoll(tmp);
    }
    inFile.close();

    original[0] = 2;

    Computer paint(original, total_length);

    cout << paint.run() << endl;

    cout << "done";
    cout << endl;
    return 0;
}
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
    BLACK,
    WHITE
};

class Node {
    public :
    int x = 0;
    int y = 0;
    int color = BLACK;

    Node(int _x, int _y, int _color) {
        x = _x;
        y = _y;
        color = _color;
    }

    int equals(Node other){
        return (x == other.x && y == other.y);
    }
};

void rotate_right(int& direction) {
    direction += 1;
    direction %= (LEFT + 1);
}
void rotate_left(int& direction) {
    direction -= 1;
    direction += (LEFT + 1);
    direction %= (LEFT + 1);
}

void move(int& x, int& y, int& direction) {
    if (direction == UP) {
        y += 1;
    } else if (direction == DOWN) {
        y -= 1;
    } else if (direction == LEFT) {
        x -= 1;
    } else if (direction == RIGHT) {
        x += 1;
    }
}
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
    int x_coord = 0;
    int y_coord = 0;
    int direction = UP;
    vector<Node> hull;


    Computer(long long int original[10000], long long int length) {
        total_length = length;
        for (int i = 0; i < total_length; i++) {
            program[i] = original[i];
        }
        hull.push_back(Node(x_coord, y_coord, WHITE));
    }

    long long int run() {
        int p[5];
        long long int first = 0;
        long long int second = 0;
        int output = 0;
        while (pc < total_length) {
            int found = 0;
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
                input_array[input_index] = 0;
                for (Node &node : hull) {
                    if (node.x == x_coord && node.y == y_coord) {
                        input_array[input_index] = node.color;
                    }
                }
                if (p[0] == 0) {
                    program[program[pc+1]] = input_array[input_index];
                } else {
                    program[program[pc+1] + relative_base] = input_array[input_index];
                }
                cout << "input : " << input_array[input_index] << endl;
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
                if (output == 0) {
                    for (Node &node : hull) {
                        if (node.x == x_coord && node.y == y_coord) {
                            node.color = first;
                            found = 1;
                        }
                    }
                    if (!found) {
                        hull.push_back(Node(x_coord, y_coord, first));
                    }
                    // cout << "paint (" << x_coord << "," << y_coord << "): " << (first == 1 ? "white" : "black") << endl;
                    output++;
                } else {
                    // cout << "dir :" << direction << endl;
                    if (first == 1) {
                        rotate_right(direction);
                        // cout << "rotate right" << endl;
                    } else {
                        rotate_left(direction);
                        // cout << "rotate left" << endl;
                    }
                    // cout << "dir :" << direction << endl;
                    move(x_coord, y_coord, direction);
                    // cout << "new coord "<< x_coord << "," << y_coord  << endl;
                    output = 0;
                }
                output_array[output_index] = first;
                cout << "output : " << output_array[output_index] << endl;
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
        //157 is to low
        return hull.size();
    }

    void print()
    {
        cout << "printing : \n";
        int max_x = 0;
        int min_x = 0;
        int max_y = 0;
        int min_y = 0;
        int canvas[10][50] = {0};

        for (Node &n : hull) {
            if (n.x > max_x) {
                max_x = n.x;
            }
            if (n.y > max_y) {
                max_y = n.y;
            }
            if (n.x < min_x) {
                min_x = n.x;
            }
            if (n.y < min_y) {
                min_y = n.y;
            }
            canvas[n.y+5][n.x] = n.color == BLACK ? 1 : 0;
        }
        // cout << max_x << " " << max_y << endl;
        // cout << min_x << " " << min_y << endl;

        for (int i = 0; i < 10; i++) {
            for (int j = 0; j < 50; j++) {
                cout << canvas[i][j];
            }
            cout << endl;
        }
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


    Computer paint(original, total_length);

    cout << paint.run() << endl;

    paint.print();

    cout << "done";
    cout << endl;
    return 0;
}
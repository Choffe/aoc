#include <fstream>
#include <iostream>
#include <sstream>
#include <set>

using namespace std;

class Node{
    public:
    string name;
    string parent;
    mutable Node *parent_ptr;
    mutable int santa_steps = -1;
    mutable int my_steps = -1;

    Node(string node_name, string parent_node){
        name = node_name;
        parent = parent_node;
        parent_ptr = NULL;
    }

    int equal(Node other) {
        return name == other.name;
    }
};

bool node_compare(Node lhs, Node rhs) {
    return lhs.name < rhs.name;
};

int calc_nbr_parents(Node *node) {
    if (node->parent_ptr == NULL) {
        return 0;
    } else {
        return 1 + calc_nbr_parents(node->parent_ptr);
    }

}

void trace_santa(Node *node, int steps) {
    if (node->name == "COM") {
        return;
    } else {
        node->santa_steps = steps;
        trace_santa(node->parent_ptr, steps + 1);
    }
}

int trace_me(Node *node, int steps) {
    cout << "searching " << node->name << endl;
    if (node->name == "COM") {
        cout << "BEGIN\n";
        return -1000000;
    } else if (node->santa_steps != -1){
        cout << "santas been here " << node->name << endl;
        return node->santa_steps + steps;
    } else {
        node->my_steps = steps;
        return trace_me(node->parent_ptr, steps + 1);
    }
}

int main() {
    bool(*fn_pt)(Node,Node) = node_compare;
    set<Node, bool(*)(Node,Node)> node_set (fn_pt);
    ifstream ifs("input.txt");
    if (!ifs.is_open()) {
        cout << "Can't open file\n";
        exit(1);
    }
    char line[100];
    while (ifs.getline(line, 100)) {
        istringstream sstream;
        sstream.str(line);
        string parent;
        string name;
        getline(sstream, parent, ')');
        getline(sstream, name);
        // cout << parent << " ) ";
        // cout << name << endl;
        Node n(name, parent);
        node_set.insert(n);
        if (parent == "COM") {
            cout << "COM" << endl;
            Node n(parent, "");
            node_set.insert(n);
        }
    }
    ifs.close();

    for (set<Node, bool(*)(Node,Node)>::iterator it = node_set.begin(); it != node_set.end(); it++) {
        Node test((*it).parent, "");
        auto found = node_set.find(test);
        if (found != node_set.end()) {
            (*it).parent_ptr = (Node*)&(*found);
        }
        // if ((*it).parent_ptr != NULL) {
        //     cout << (*it).parent_ptr->name << " ) " << (*it).name << endl;
        // }
    }

    // int sum = 0;
    // for (Node n : node_set) {
    //     sum += calc_nbr_parents(&n);
    //     // cout << calc_nbr_parents(&n) << endl ;
    // }
    // cout << sum << endl;

    Node santas("SAN", "");
    auto found = node_set.find(santas);
    if (found != node_set.end()) {
        cout << "Tracing santa : \n";
        trace_santa((Node*)&(*found), 0);
    }
    Node me("YOU", "");
    auto found_me = node_set.find(me);
    if (found_me != node_set.end()) {
        cout << "Tracing me : \n";
        cout << trace_me((Node*)&(*found_me), 0) - 2 << endl;
    }

}
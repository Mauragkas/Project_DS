#include <iostream>
#include <string.h>
#include <fstream>
#include <sstream>
#include <vector>

using namespace std;

#define MOD 11

struct Data {
    string Direction;
    int Year;
    string Date;
    string Weekday;
    string Country;
    string Commodity;
    string Transport_Mode;
    string Measure;
    long int Value;
    long int Cumulative;
};

struct Node {
    Data data;
    Node *next;
};

vector<Node*> init() {
    vector<Node*> nodes;
    for (int i = 0; i < MOD; i++) {
        nodes.push_back(NULL);
    }
    return nodes;
}

Node *createNode(Data data) {
    Node *newNode = new Node;
    newNode->data = data;
    newNode->next = NULL;
    return newNode;
}

// search for a node by date 
Node *searchNode(vector<Node*> nodes, string date) {
    // calculate hash key
    int key = 0;
    for (int i = 0; i < date.length(); i++) {
        // get the ascii value of the character and add it to the key
        key += (int)date[i];
    }
    key %= MOD;

    // if the key is not in the vector, return NULL
    if (nodes[key] == NULL) {
        return NULL;
    } else {
        // if the key is in the vector, search the linked list
        Node *temp = nodes[key];
        while (temp != NULL) {
            if (temp->data.Date == date) {
                return temp;
            }
            temp = temp->next;
        }
        return NULL;
    }
}

// delete a node by date
vector<Node*> deleteNode(vector<Node*> nodes, string date) {
    // calculate hash key
    int key = 0;
    for (int i = 0; i < date.length(); i++) {
        // get the ascii value of the character and add it to the key
        key += (int)date[i];
    }
    key %= MOD;

    // if the key is not in the vector, return the vector
    if (nodes[key] == NULL) {
        return nodes;
    } else {
        // if the key is in the vector, search the linked list
        Node *temp = nodes[key];
        Node *prev = NULL;
        while (temp != NULL) {
            if (temp->data.Date == date) {
                // if the node is the first node in the linked list
                if (prev == NULL) {
                    nodes[key] = temp->next;
                } else {
                    prev->next = temp->next;
                }
                delete temp;
                return nodes;
            }
            prev = temp;
            temp = temp->next;
        }
        return nodes;
    }
}

// edit a node by date
vector<Node*> editNode(vector<Node*> nodes, string date) {
    // calculate hash key
    int key = 0;
    for (int i = 0; i < date.length(); i++) {
        // get the ascii value of the character and add it to the key
        key += (int)date[i];
    }
    key %= MOD;

    // if the key is not in the vector, return the vector
    if (nodes[key] == NULL) {
        return nodes;
    } else {
        // if the key is in the vector, search the linked list
        Node *temp = nodes[key];
        while (temp != NULL) {
            if (temp->data.Date == date) {
                // edit the node
                cout << "Enter the new value: ";
                cin >> temp->data.Value;
                return nodes;
            }
            temp = temp->next;
        }
        return nodes;
    }
}

vector<Node*> insert(vector<Node*> nodes, Data data) {
    Node *newNode = createNode(data);

    // calculate hash key
    int key = 0;
    for (int i = 0; i < data.Date.length(); i++) {
        // get the ascii value of the character and add it to the key
        key += (int)data.Date[i];
    }
    key %= MOD;

    // if the key is not in the vector, add it
    if (nodes[key] == NULL) {
        nodes[key] = newNode;
    } else {
        // if the key is in the vector, add the node to the end of the linked list
        Node *temp = nodes[key];
        while (temp->next != NULL) {
            temp = temp->next;
        }
        temp->next = newNode;
    }

    return nodes;
}

vector<Node*> read_data(string filename) {
    vector<Node*> nodes = init();
    ifstream file(filename);
    if (!file.is_open()) {
        cout << "Error opening file" << endl;
        return nodes;
    }
    string line;
    getline(file, line); // skip the first line
    while (getline(file, line)) {
        Data temp;
        string temp_array[10];

        stringstream ss(line);
        string token;

        bool insideQuotes = false;
        int i = 0;
        
        while (getline(ss, token, ',')) {
            if (token.front() == '\"' && !insideQuotes) {
                insideQuotes = true;
            }
        
            if (token.back() == '\"' && insideQuotes) {
                insideQuotes = false;
            }
        
            if (insideQuotes) {
                temp_array[i - 1] += "," + token;
            } else {
                temp_array[i] = token;
                i++;
            }
        }

        temp.Direction = temp_array[0];
        temp.Year = stoi(temp_array[1]);
        temp.Date = temp_array[2];
        temp.Weekday = temp_array[3];
        temp.Country = temp_array[4];
        temp.Commodity = temp_array[5];
        temp.Transport_Mode = temp_array[6];
        temp.Measure = temp_array[7];
        temp.Value = stol(temp_array[8]);
        temp.Cumulative = stol(temp_array[9]);

        nodes = insert(nodes, temp);
    }
    file.close();
    return nodes;
}

void print_data(Node *node) {
    cout << node->data.Direction << ", ";
    cout << node->data.Year << ", ";
    cout << node->data.Date << ", ";
    cout << node->data.Weekday << ", ";
    cout << node->data.Country << ", ";
    cout << node->data.Commodity << ", ";
    cout << node->data.Transport_Mode << ", ";
    cout << node->data.Measure << ", ";
    cout << node->data.Value << ", ";
    cout << node->data.Cumulative << endl;
}

int main() {
    vector<Node*> nodes = read_data("effects.csv");

    while (true) {
        cout << "\n--------------------------------" << endl;
        cout << " MENU " << endl;
        cout << "--------------------------------" << endl;
        cout << "1. Search by date" << endl;
        cout << "2. Edit by date" << endl;
        cout << "3. Delete by date" << endl;
        cout << "0. Exit" << endl;
        cout << "enter your choice: ";
        int choice;
        cin >> choice;

        switch (choice) {
            case 0:
                exit(0);
            case 1: {
                cout << "Enter the date: ";
                string date;
                cin >> date;
                Node *temp = searchNode(nodes, date);
                if (temp == NULL) {
                    cout << "No data found" << endl;
                } else {
                    print_data(temp);
                }
                break;
            }
            case 2: {
                cout << "Enter the date: ";
                string date;
                cin >> date;
                nodes = editNode(nodes, date);
                break;
            }
            case 3: {
                cout << "Enter the date: ";
                string date;
                cin >> date;
                nodes = deleteNode(nodes, date);
                break;
            }
            default:
                cout << "Invalid choice" << endl;
        }
    }
    
    return 0;
}
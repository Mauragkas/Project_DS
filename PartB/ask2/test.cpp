#include <iostream>
#include <string.h>
#include <fstream>
#include <sstream>
#include <vector>

using namespace std;

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
    Node *left;
    Node *right;
};

int convert_date_to_days(const string& date_str) {
    int month, day, year;
    sscanf(date_str.c_str(), "%d/%d/%d", &day, &month, &year);
    return (year - 2010) * 365 + (month - 1) * 30 + day;
};

// create a new node
Node* createNode(Data data) {
    Node *newNode = new Node;
    newNode->data = data;
    newNode->left = NULL;
    newNode->right = NULL;
    return newNode;
}

Node* insert(Node* node, Data data) {
    if (node == NULL) {
        return createNode(data);
    }
    // if (convert_date_to_days(data.Date) < convert_date_to_days(node->data.Date)) {
    if (data.Value < node->data.Value) {
        node->left = insert(node->left, data);
    } else {
        node->right = insert(node->right, data);
    }
    return node;
}

Node *nodeWithMaxValue(Node *node) {
    Node *current = node;
    while (current->right != NULL)
        current = current->right;
    return current;
}

Node* nodeWithMinValue(Node* node) {
    Node* current = node;
    while (current && current->left != NULL) {
        current = current->left;
    }
    return current;
}

int getNumberOfNodes(Node* root) {
    if (root == NULL) {
        return 0;
    }
    return getNumberOfNodes(root->left) + getNumberOfNodes(root->right) + 1;
}

void printDates(Node *root, int value) {
    if (root == NULL) {
        return;
    }
    if (root->data.Value == value) {
        cout << root->data.Direction << ", "
            << root->data.Year << ", "
            << root->data.Date << ", "
            << root->data.Weekday << ", "
            << root->data.Country << ", "
            << root->data.Commodity << ", "
            << root->data.Transport_Mode << ", "
            << root->data.Measure << ", "
            << root->data.Value << ", "
            << root->data.Cumulative << endl;
    }
    printDates(root->left, value);
    printDates(root->right, value);
}

Node* read_data(string filename) {
    Node *root = NULL;
    ifstream file(filename);
    if (!file.is_open()) {
        cout << "Error opening file" << endl;
        exit(1);
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

        root = insert(root, temp);
    }
    file.close();
    return root;
}

int main() {
  // Node *root = read_data("effects.csv");
  Node *root = read_data("effects.csv");

  cout << "Number of nodes: " << getNumberOfNodes(root) << endl;

  while (true) {
        cout << "\n--------------------------------" << endl;
        cout << " MENU " << endl;
        cout << "--------------------------------" << endl;
        cout << "1. find the date/dates with the Max value " << endl;
        cout << "2. find the date/dates with the Min value " << endl;
        cout << "3. exit " << endl;
        cout << "enter your choice: ";
        int choice;
        cin >> choice;
        switch (choice) {
            case 1: {
                Node *maxNode = nodeWithMaxValue(root);
                cout << "\n\tDate with Max value: " << maxNode->data.Date << endl;
                printDates(root, maxNode->data.Value);
                break;
            }
            case 2: {
                Node *minNode = nodeWithMinValue(root);
                cout << "\n\tDate with Min value: " << minNode->data.Date << endl;
                printDates(root, minNode->data.Value);
                break;
            }
            case 3: {
                exit(0);
            }
            default: {
                cout << "\n\tInvalid choice" << endl;
                break;
            }
        }
    }
    
  return 0;
}
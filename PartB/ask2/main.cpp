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
    int height;
};

// Calculate height
int height(Node *node) {
    if (node == NULL)
        return 0;
    return node->height;
}

int max(int a, int b) {
    return (a > b) ? a : b;
}

// New node creation
Node *newNode(Data data) {
    Node *node = new Node();
    node->data = data;
    node->left = NULL;
    node->right = NULL;
    node->height = 1;
    return (node);
}

// Rotate right
Node *rightRotate(Node *y) {
    Node *x = y->left;
    Node *T2 = x->right;

    x->right = y;
    y->left = T2;

    y->height = max(height(y->left), height(y->right)) + 1;
    x->height = max(height(x->left), height(x->right)) + 1;

    return x;
}

// Rotate left
Node *leftRotate(Node *x) {
    Node *y = x->right;
    Node *T2 = y->left;

    y->left = x;
    x->right = T2;

    x->height = max(height(x->left), height(x->right)) + 1;
    y->height = max(height(y->left), height(y->right)) + 1;

    return y;
}

// Get balance factor of each node
int getBalance(Node *node) {
    if (node == NULL)
        return 0;
    return height(node->left) - height(node->right);
}

// Insert node
Node *insertNode(Node *node, Data data) {
    if (node == NULL)
        return (newNode(data));
    if (data.Value < node->data.Value)
        node->left = insertNode(node->left, data);
    else if (data.Value > node->data.Value)
        node->right = insertNode(node->right, data);
    else
        return node;

    // Update the balance factor of each node and balance the tree
    node->height = 1 + max(height(node->left), height(node->right));
    int balance = getBalance(node);
    if (balance > 1) {
        if (data.Value < node->left->data.Value) {
            return rightRotate(node);
        } else if (data.Value > node->left->data.Value) {
            node->left = leftRotate(node->left);
            return rightRotate(node);
        }
    }
    if (balance < -1) {
        if (data.Value > node->right->data.Value) {
            return leftRotate(node);
        } else if (data.Value < node->right->data.Value) {
            node->right = rightRotate(node->right);
            return leftRotate(node);
        }
    }
    return node;
}

// Node with minimum value
Node *nodeWithMinValue(Node *node) {
    Node *current = node;
    while (current->left != NULL)
        current = current->left;
    return current;
}

Node *nodeWithMaxValue(Node *node) {
    Node *current = node;
    while (current->right != NULL)
        current = current->right;
    return current;
}

Node *read_data(string filename) {
    Node *root = NULL;
    ifstream file(filename);
    if (!file.is_open()) {
        cout << "Error opening file" << endl;
        exit(1);
    }
    string line;
    getline(file, line);
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

        root = insertNode(root, temp);
        
    }
    file.close();
    return root;
}

void preorder(Node *root) {
    if (root == NULL) {
        return;
    }
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
    preorder(root->left);
    preorder(root->right);
}

void printDates(Node *root, string date) {
    if (root == NULL) {
        return;
    }
    if (root->data.Date == date) {
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
    printDates(root->left, date);
    printDates(root->right, date);
}

int getLength(Node *root) {
    if (root == NULL) {
        return 0;
    }
    return 1 + getLength(root->left) + getLength(root->right);
}

int main() {
    Node *root = read_data("effects.csv");

    cout << "Number of nodes: " << getLength(root) << endl;
    
    while (true) {
        cout << "\n--------------------------------" << endl;
        cout << " MENU " << endl;
        cout << "--------------------------------" << endl;
        cout << "1. find the date with the Max value " << endl;
        cout << "2. find the date with the Min value " << endl;
        cout << "3. exit " << endl;
        cout << "enter your choice: ";
        int choice;
        cin >> choice;
        switch (choice) {
            case 1: {
                Node *maxNode = nodeWithMaxValue(root);
                cout << "\n\tDate with Max value: " << maxNode->data.Date << endl;
                printDates(root, maxNode->data.Date);
                break;
            }
            case 2: {
                Node *minNode = nodeWithMinValue(root);
                cout << "\n\tDate with Min value: " << minNode->data.Date << endl;
                printDates(root, minNode->data.Date);
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
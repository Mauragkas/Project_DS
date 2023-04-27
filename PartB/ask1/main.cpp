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

void inoder(Node *root) {
    if (root == NULL) {
        return;
    }
    inoder(root->left);
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
    inoder(root->right);
}

Node* insert(Node* node, Data data) {
    if (node == NULL) {
        return createNode(data);
    }
    if (convert_date_to_days(data.Date) < convert_date_to_days(node->data.Date)) {
    // if (data.Value < node->data.Value) {
        node->left = insert(node->left, data);
    } else {
        node->right = insert(node->right, data);
    }
    return node;
}

Node* minValueNode(Node* node) {
    Node* current = node;
    while (current && current->left != NULL) {
        current = current->left;
    }
    return current;
}

Node* deleteNode(Node* root, Data data) {
    if (root == NULL) {
        return root;
    }

    int date = convert_date_to_days(data.Date);
    int root_date = convert_date_to_days(root->data.Date);

    if (date < root_date) {
        root->left = deleteNode(root->left, data);
    } else if (date > root_date) {
        root->right = deleteNode(root->right, data);
    } else {
        if (root->left == NULL) {
            Node *temp = root->right;
            delete root;
            return temp;
        } else if (root->right == NULL) {
            Node *temp = root->left;
            delete root;
            return temp;
        }
        Node* temp = minValueNode(root->right);
        root->data = temp->data;
        root->right = deleteNode(root->right, temp->data);
    }
    return root;
}

Node* search(Node* root, Data data) {
    int date = convert_date_to_days(data.Date);
    int root_date = convert_date_to_days(root->data.Date);
    if (root == NULL || date == root_date ) {
        return root;
    }
    if (date < root_date) {
        return search(root->left, data);
    }
    return search(root->right, data);
}

void editNode(Node* root, Data data) {
    Node* temp = search(root, data);
    if (temp != NULL) {
        temp->data = data;
    }
}

int getNumberOfNodes(Node* root) {
    if (root == NULL) {
        return 0;
    }
    return getNumberOfNodes(root->left) + getNumberOfNodes(root->right) + 1;
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

    while (true) {
        cout << "\n--------------------------------" << endl;
        cout << " MENU " << endl;
        cout << "--------------------------------" << endl;
        cout << "1. Inorder traversal" << endl;
        cout << "2. Search" << endl;
        cout << "3. Edit" << endl;
        cout << "4. Delete" << endl;
        cout << "0. Exit" << endl;
        cout << "enter your choice: ";
        int choice;
        cin >> choice;
        switch (choice) {
            case 1:
                inoder(root);
                break;
            case 2: {
                Data temp;
                cout << "Enter the date to search: ";
                cin >> temp.Date;
                Node* result = search(root, temp);
                if (result != NULL) {
                    cout << "Found: " << endl;
                    cout << result->data.Direction << ", "
                        << result->data.Year << ", "
                        << result->data.Date << ", "
                        << result->data.Weekday << ", "
                        << result->data.Country << ", "
                        << result->data.Commodity << ", "
                        << result->data.Transport_Mode << ", "
                        << result->data.Measure << ", "
                        << result->data.Value << ", "
                        << result->data.Cumulative << endl;
                } else {
                    cout << "Not found" << endl;
                }
                break;
            }
            case 3: {
                Data temp;
                cout << "Enter the date to edit: ";
                cin >> temp.Date;
                Node* result = search(root, temp);
                if (result != NULL) {
                    cout << "Found: " << endl;
                    cout << result->data.Direction << ", "
                        << result->data.Year << ", "
                        << result->data.Date << ", "
                        << result->data.Weekday << ", "
                        << result->data.Country << ", "
                        << result->data.Commodity << ", "
                        << result->data.Transport_Mode << ", "
                        << result->data.Measure << ", "
                        << result->data.Value << ", "
                        << result->data.Cumulative << endl;
                    cout << "Enter new data: " << endl;
                    cout << "Value: ";
                    cin >> temp.Value;
                    editNode(root, temp);
                    cout << "Edited" << endl;
                } else {
                    cout << "Not found" << endl;
                }
                break;
            }
            case 4: {
                Data temp;
                cout << "Enter the date to delete: ";
                cin >> temp.Date;
                Node* result = search(root, temp);
                if (result != NULL) {
                    cout << "Found: " << endl;
                    cout << result->data.Direction << ", "
                        << result->data.Year << ", "
                        << result->data.Date << ", "
                        << result->data.Weekday << ", "
                        << result->data.Country << ", "
                        << result->data.Commodity << ", "
                        << result->data.Transport_Mode << ", "
                        << result->data.Measure << ", "
                        << result->data.Value << ", "
                        << result->data.Cumulative << endl;
                    root = deleteNode(root, temp);
                    cout << "Deleted" << endl;
                } else {
                    cout << "Not found" << endl;
                }
                break;
            }
            case 0:
                exit(0);
            default:
                cout << "Invalid choice" << endl;
        }
    }
    return 0;
}
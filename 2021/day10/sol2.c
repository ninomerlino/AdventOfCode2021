#include<stdio.h>
#include<stdbool.h>
#include<stdlib.h>
#include<string.h>

#define MAX_INT 2147483647

typedef char State;
typedef long Points;

typedef struct StackElement{
    State c;
    struct StackElement* next;
} StackElement;

typedef StackElement *Stack;

typedef Points *Darr;

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

Stack newElement(State value){
    Stack s = malloc(sizeof(StackElement));
    s->c = value;
    s->next = NULL;
    return s;
}

void Stack_push(Stack* head, State value){
    Stack curr = *head;
    if(curr == NULL){
        *head = newElement(value);
    }else{
        while (curr->next != NULL){
            curr = curr->next;
        }
        curr->next = newElement(value);
    }
}

State Stack_pop(Stack *head){
    State c;
    Stack curr = *head, tmp;
    if(*head == NULL){
        return '\1';
    }else if (curr->next == NULL)
    {
        c = curr->c;
        free(curr);
        *head = NULL;
    }else{
        while (curr->next->next != NULL){
            curr = curr->next;
        }
        tmp = curr->next;
        c = tmp->c;
        curr->next = NULL;
        free(tmp);
    }
    return c;
}

int Stack_size(Stack head){
    int size = 0;
    while (head != NULL){
        size++;
        head = head->next;
    }
    return size;
}

void Stack_free(Stack *head){
    Stack curr = *head, tmp;
    while (curr != NULL){
        tmp = curr;
        curr = curr->next;
        free(tmp);
    }
    *head = NULL;    
}

void Darr_append(Darr *arr, int *size, Points value){
    if(*arr != NULL){
        (*arr) = realloc((*arr), sizeof(Points)*((*size)+1));
        (*arr)[(*size)] = value;
        (*size)++;
    }else{
        (*arr) = malloc(sizeof(Points));
        (*arr)[0] = value;
        (*size) = 1;
    }
}

bool analizeState(Stack *sstack, State curr_state){
    switch (curr_state){
    case '(':
    case '[':
    case '{':
    case '<':
        Stack_push(sstack, curr_state);
        return true;
    case ')':
        if(Stack_pop(sstack) == '(')
            return true;
        return false;
    case ']':
        if(Stack_pop(sstack) == '[')
            return true;
        return false;
    case '}':
        if(Stack_pop(sstack) == '{')
            return true;
        return false;
    case '>':
        if(Stack_pop(sstack) == '<')
            return true;
        return false;
    default:
        return true;//ignore any other char
    }
}

Points autocomplete(const char* line){//takes a line and returns the invalid score
    Stack stack = NULL;
    Points points = 0;
    for (size_t i = 0, len = strlen(line); i < len; i++){
        if(!analizeState(&stack, line[i]))
            return points;
    }
    while(stack != NULL){
        points *= 5;
        switch(Stack_pop(&stack)){
            case '(':
                points +=1;
                break;
            case '[':
                points +=2;
                break;
            case '{':
                points +=3;
                break;
            case '<':
                points +=4;
                break;
        }
    }
    Stack_free(&stack);
    return points;
}

void selectionSort(Points *arr, const int size){//cause i don't wanna implement a more complex sort just for this
    int pmin;
    Points min, tmp;
    for (int i = 0; i < size; i++){
        min = -1;
        for (int j = i; j < size; j++){
            if(arr[j] < min || min == -1){
                min = arr[j];
                pmin = j;
            }
        }
        tmp = arr[i];
        arr[i] = arr[pmin];
        arr[pmin] = tmp;
    }
}

Points solve(const char* filename){
    FILE* file = fopen(filename, "r");
    char* buffer = NULL;
    size_t buff_size = 0;
    Darr arr = NULL;
    int arr_size = 0;
    Points tmp;

    if(file == NULL)raiseError("Cannot open file");

    while (getline(&buffer, &buff_size, file) != EOF){
        tmp = autocomplete(buffer);
        if(tmp){
            Darr_append(&arr, &arr_size, tmp);
        }
    }
    
    if(ferror(file))raiseError("Error while reading file");
	fclose(file);

    if(arr == NULL)raiseError("No incomplete string found");
    selectionSort(arr, arr_size);
    for (int i = 0; i < arr_size; i++)printf(" %ld ", arr[i]);
    tmp = arr[(arr_size/2)];
    free(arr);
    return tmp;
}

int main(){
    Points sol = solve("input.txt");
    printf("The solution is %ld\n", sol);
    return 0;
}

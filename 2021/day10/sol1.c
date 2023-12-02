#include<stdio.h>
#include<stdbool.h>
#include<stdlib.h>
#include<string.h>

typedef char State;

typedef struct StackElement{
    State c;
    struct StackElement* next;
} StackElement;

typedef StackElement *Stack;

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
        free(head);
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
        return true;
    }
}

int isInvalid(const char* line){//takes a line and returns the invalid score
    Stack stack = NULL;
    for (size_t i = 0, len = strlen(line); i < len; i++){
        if(!analizeState(&stack, line[i])){
            switch(line[i]){
                case ')':
                    return 3;
                case ']':
                    return 57;
                case '}':
                    return 1197;
                case '>':
                    return 25137;
            }
        }
    }
    Stack_free(&stack);
    return 0;
}

int solve(const char* filename){
    FILE* file = fopen(filename, "r");
    char* buffer = NULL;
    size_t buff_size = 0;
    int sum = 0;

    if(file == NULL)raiseError("Cannot open file");

    while (getline(&buffer, &buff_size, file) != EOF){
        sum += isInvalid(buffer);
    }
    
    if(ferror(file))raiseError("Error while reading file");
	fclose(file);

    return sum;
}

int main(){
    int sol = solve("input.txt");
    printf("The solution is %d\n", sol);
    return 0;
}

#include<stdio.h>
#include<stdlib.h>

typedef char fish;
typedef struct School{
    fish* fishes;
    size_t size;
} School;

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

void append(School *s, fish f){
    if(s->fishes != NULL){
        s->fishes = realloc(s->fishes, sizeof(fish)*(s->size+1));
        s->fishes[s->size] = f;
        s->size++;
    }else{
        s->fishes = malloc(sizeof(fish));
        s->fishes[0] = f;
        s->size = 1;
    }
}

void removeAt(School *s, size_t index){
    char* newArray = malloc(sizeof(fish)*(s->size-1));
    for (size_t i = 0, j = 0; i < s->size; i++, j++)
        if(i != index)newArray[j] == s->fishes[i];
        else j--;
    free(s->fishes);
    s->fishes = newArray;
    s->size--;
}

void remvoveALL(School *s){
    free(s->fishes);
    s->fishes = NULL;
    s->size = 0;
}

void nextDay(School* s){
    size_t oldsize = s->size;
    for (size_t i = 0; i < oldsize; i++)
    {
        if(s->fishes[i] == 0){
            append(s, 8);
            s->fishes[i] = 6;
        }else{
            s->fishes[i]--;
        }
    }   
}


void getInput(const char* filename, School* s){
	FILE* f = fopen(filename, "r");
    int age;
    char c;
    s->fishes = NULL;
    s->size = 0;
	if(f == NULL)raiseError("Cannot open file");

    while (fscanf(f, "%d%c", &age, &c) != EOF){
        append(s, age);
    }
    printf("Counted a total of %ld fishes\n", s->size);
	if(ferror(f))raiseError("Error while reading file");
	fclose(f);
}

void printSchool(School* s){
    putchar('[');
    for(size_t i = 0; i < s->size; i++)
    {
        if(i == 20){
            printf("...");
            break;
        }
        printf("%d", s->fishes[i]);
        if(i != s->size-1)putchar(',');

    }
    printf("]\n");
    return;
}

int main(){
    School school;
    getInput("input.txt", &school);
    for (int day = 0; day < 80; day++)
    {
        nextDay(&school);
        printf("Day %3d there are %6ld fishes now\n", day+1, school.size);
    }
    remvoveALL(&school);
    return 0;
}
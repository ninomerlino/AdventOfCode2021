//dr. strangesol or how i learned to optimize and improve my solution
#include<stdio.h>
#include<stdlib.h>
#include<stdint.h>

typedef uint64_t  fishsize;
typedef fishsize School[9];

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

void nextDay(School s){
    fishsize new = s[0];
    for (int i = 0; i < 8; i++)s[i]=s[i+1];
    s[8] = new;
    s[6] += new;
}


void getInput(const char* filename, School s){
	FILE* f = fopen(filename, "r");
    int age;
    char c;
	if(f == NULL)raiseError("Cannot open file");

    while (fscanf(f, "%d%c", &age, &c) != EOF){
        s[age]++;
    }
	if(ferror(f))raiseError("Error while reading file");
	fclose(f);
}

fishsize sumSchool(School s){
    fishsize sum = 0;
    for (int i = 0; i < 9; i++){
        sum += s[i];
    }
    return sum;
}

int main(){
    School school;
    for (size_t i = 0; i < 9; i++)school[i] = 0;
    printf("Day %3d there are %lu fishes now\n", 0, sumSchool(school));
    getInput("input.txt", school);
    for (int day = 0; day < 256; day++)
    {
        nextDay(school);
        printf("Day %3d there are %9lu fishes now\n", day+1, sumSchool(school));
    }
    return 0;
}
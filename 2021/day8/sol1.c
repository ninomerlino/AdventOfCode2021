#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>
#include<string.h>

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

int readfile(const char* filename){
    FILE* file = fopen(filename, "r");
    bool output = false;
    int count = 0, length, read = 0;
    char tmp = '\0', str[8];

    if(file == NULL)raiseError("Cannot open file");

    while (fscanf(file, "%7s", str) != EOF)
    {
        if(strcmp(str, "|") == 0){
            output = true;
            continue;
        }
        if(read == 4){
            read = 0;
            output = false;
        }
        if(!output)continue;
        length = strlen(str);
        if(length == 2 || length == 3 || length == 4 || length == 7){
            count++;
            printf("[%s] ", str);
        }
        read++;
    }
    
    if(ferror(file))raiseError("Error while reading file");
	fclose(file);
    return count;
}

int main(){
    int sol = readfile("input.txt");
    printf("Solution = %d\n", sol);
    return 0;
}
#include<stdio.h>
#include<stdbool.h>
#include<stdlib.h>
#define X 0
#define Y 1
#define msize 1000

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

void UpdateTable(int table[msize][msize], int x1, int x2, int y1, int y2){
    int ix = 1, iy = 1;
    if(y1 > y2)iy = -1;
    if(x1 > x2)ix = -1;
    while (y1 != y2 || x1 != x2)
    {
        table[y1][x1]++;
        if(y1 != y2)y1 += iy;
        if(x1 != x2)x1 += ix;
    }
    table[y2][x2]++;
}

void getInput(const char* filename, int table[msize][msize]){
	FILE* f = fopen(filename, "r");
    int a,b,c,d;

	if(f == NULL)raiseError("Cannot open file");

    while (fscanf(f, "%d,%d -> %d,%d\n", &a, &b, &c, &d) != EOF){
        UpdateTable(table, a, c, b, d);
    }

	if(ferror(f))raiseError("Error while reading file");
	fclose(f);
}

int countI(int table[msize][msize]){
    int i = 0;
    for (size_t y = 0; y < msize; y++)
        for (size_t x = 0; x < msize; x++)
            if(table[y][x] > 1)i++;
    return i;
}

int main(){
    int table[msize][msize];

    for (int i = 0; i < msize; i++)
        for (int j = 0; j < msize; j++)
            table[i][j] = 0;//init all table to 0
    
    getInput("input.txt", table);//get  lines
    printf("Solution is : %d\n", countI(table));
    return 0;
}
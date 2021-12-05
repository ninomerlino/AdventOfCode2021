#include<stdio.h>
#include<stdbool.h>
#include<stdlib.h>
#define X 0
#define Y 1
#define msize 1000

typedef int Point[2];
typedef struct Line
{
    Point start;
    Point end;
} Line;


void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

int getInput(const char* filename, Line** l){
	FILE* f = fopen(filename, "r");
    Line* lines;
    size_t size = 0, cento = 100;
    char filec;
    int a,b,c,d;


	if(f == NULL)raiseError("Cannot open file");

    while (!feof(f))
    {
        filec = getc(f);
        if(filec == '\n')size++;
    }

    lines = malloc(sizeof(Line) * size);
    fseek(f, 0, SEEK_SET);

    for (size_t i = 0; i < size; i++)
    {
        fscanf(f, "%d,%d -> %d,%d\n", &a, &b, &c, &d);
        lines[i].start[X] = a;
        lines[i].start[Y] = b;
        lines[i].end[X] = c;
        lines[i].end[Y] = d;
    }
    

	if(ferror(f))raiseError("Error while reading file");
	fclose(f);
    *l = lines;
    return size;
}

int countI(int table[msize][msize]){
    int i = 0;
    for (size_t y = 0; y < msize; y++)
        for (size_t x = 0; x < msize; x++)
            if(table[y][x] > 1)i++;
    return i;
}

void DrawH(int table[msize][msize], Line l){
    int inc = 1, x = l.start[X], y = l.start[Y];
    if(l.start[X] > l.end[X])inc = -1;
    while (x != l.end[X])
    {
        table[y][x]++;
        x += inc;
    }
    table[y][l.end[X]]++;
}

void DrawV(int table[msize][msize], Line l){
    int inc = 1, y = l.start[Y], x = l.start[X];
    if(l.start[Y] > l.end[Y])inc = -1;
    while (y != l.end[Y])
    {
        table[y][x]++;
        y += inc;
    }
    table[l.end[Y]][x]++;
}

void DrawD(int table[msize][msize], Line l){
    int ix = 1, iy = 1;
    int y = l.start[Y], x = l.start[X];
    if(l.start[Y] > l.end[Y])iy = -1;
    if(l.start[X] > l.end[X])ix = -1;
    while (y != l.end[Y] && x != l.end[X])
    {
        table[y][x]++;
        y += iy;
        x += ix;
    }
    table[l.end[Y]][l.end[X]]++;
}

void Print(int table[msize][msize]){
    FILE* f = fopen("out.txt", "w");
        for (int i = 0; i < msize; i++){
            for (int j = 0; j < msize; j++)fprintf(f, "%d", table[j][i]);
            putc('\n',f);
        }
    fclose(f);
}

int main(){
    Line* lines;
    int size;
    int table[msize][msize];

    for (int i = 0; i < msize; i++)
        for (int j = 0; j < msize; j++)
            table[i][j] = 0;//init all table to 0
    
    size = getInput("input.txt", &lines);//get  lines

    for (int i = 0; i < size; i++)//draw all lines on table
    {
        if(lines[i].start[Y] == lines[i].end[Y])
            DrawH(table, lines[i]);
        else if(lines[i].start[X] == lines[i].end[X])
            DrawV(table, lines[i]);
        else
            DrawD(table,  lines[i]);
    }
    printf("Solution is : %d with %d lines\n", countI(table), size);

    free(lines);
    return 0;
}
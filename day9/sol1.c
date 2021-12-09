#include<stdio.h>
#include<stdlib.h>

typedef struct Table{
    int* value;
    int rows;
    int cols;
} Table;

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

void getInputSize(FILE* f, int* col, int* row){
    char* buff = NULL;
    int b_size = 0;
    *col = 0;
    *row = 0;

    while (getc(f) != '\n')(*col)++;//count col size
    (*row)++;//count first line
    while (getline(&buff, &b_size, f) != EOF)(*row)++;
    fseek(f, 0, SEEK_SET);
    free(buff);
}

int readfile(const char* filename, Table *matrix){
    FILE* file = fopen(filename, "r");
    int col_size, row_size;
    int* m;

    if(file == NULL)raiseError("Cannot open file");

    getInputSize(file, &col_size, &row_size);
    m = malloc(sizeof(int)*col_size*row_size);
    for (int row = 0; row < row_size; row++)
    {
        for (int col = 0; col < col_size; col++)
        {
            m[row*col_size + col] = getc(file) - 48;
        }
        getc(file);//ignore \n
    }
    matrix->value = m;
    matrix->cols = col_size;
    matrix->rows = row_size;

    if(ferror(file))raiseError("Error while reading file");
	fclose(file);
}

int lowPoint(Table* t, int r, int c){
    int value = t->value[r*t->cols + c];
    if(r-1 >= 0 && t->value[(r-1)*t->cols + c] <= value)return 0;
    if(r+1 < t->rows && t->value[(r+1)*t->cols + c] <= value)return 0;
    if(c-1 >= 0 && t->value[r*t->cols + (c-1)] <= value)return 0;
    if(c+1 < t->cols && t->value[r*t->cols + (c+1)] <= value)return 0;
    printf("[%d]", value);
    return 1;
}

int main(){
    Table table;
    int sum = 0;
    
    readfile("input.txt", &table);
    for (int row = 0; row < table.rows; row++)
    {
        for (int col = 0; col < table.cols; col++){
            if(lowPoint(&table, row, col))sum = sum + 1 + table.value[row*table.cols + col];
        }
    }
    printf("Sum of risk is %d\n", sum);
    return 0;
}

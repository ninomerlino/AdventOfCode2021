#include<stdio.h>
#include<stdbool.h>
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

bool lowPoint(Table* t, int r, int c){
    int value = t->value[r*t->cols + c];
    if(r-1 >= 0 && t->value[(r-1)*t->cols + c] <= value)return false;
    if(r+1 < t->rows && t->value[(r+1)*t->cols + c] <= value)return false;
    if(c-1 >= 0 && t->value[r*t->cols + (c-1)] <= value)return false;
    if(c+1 < t->cols && t->value[r*t->cols + (c+1)] <= value)return false;
    printf("[%d]", value);
    return true;
}

int sizeOfBasin(Table* t, bool* checked, int r, int c){
    int size;
    if(t->value[r*t->cols + c] == 9 || checked[r*t->cols + c])return 0;

    size = 1;
    checked[r*t->cols + c] = true;
    if(r-1 >= 0)size += sizeOfBasin(t, checked, r-1, c);
    if(r+1 < t->rows)size += sizeOfBasin(t, checked, r+1, c);
    if(c-1 >= 0)size += sizeOfBasin(t, checked, r, c-1);
    if(c+1 < t->cols)size += sizeOfBasin(t, checked, r, c+1);

    return size;
}

void isMax(int m[3], int value){
    if(value > m[2]){
        m[2] = value;
        if(value > m[1]){
            m[2] = m[1];
            m[1] = value;
            if(value > m[0]){
                m[1] = m[0];
                m[0] = value;
            }
        }
    }
}

int main(){
    Table table;
    bool *checked;
    int sum = 0;
    int max[3] = {0};
    
    readfile("input.txt", &table);
    checked = malloc(sizeof(bool)*table.cols*table.rows);
    for (int i = 0; i < table.cols*table.rows; i++)checked[i] = false;
    

    for (int row = 0; row < table.rows; row++)
    {
        for (int col = 0; col < table.cols; col++){
            if(lowPoint(&table, row, col))isMax(max, sizeOfBasin(&table, checked, row, col));
        }
    }
    printf("Sum of risk is %d\n", max[0]*max[1]*max[2]);
    return 0;
}

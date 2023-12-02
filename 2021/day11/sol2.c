#include<stdio.h>
#include<stdbool.h>
#include<stdlib.h>

#define Matrix_at(m,c,r) m.lighttable[(r)*(m.cols) + (c)]

typedef int LightLevel;
typedef int FlashCount;

typedef struct Matrix{
    LightLevel* lighttable;
    int cols;
    int rows;
} Matrix;

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

Matrix getInput(const char* filename){
    FILE* file = fopen(filename, "r");
    Matrix m;

    if(file == NULL)raiseError("Cannot open file");

    getInputSize(file, &(m.cols), &(m.rows));
    m.lighttable = malloc(sizeof(LightLevel)*m.cols*m.rows);
    for (int i = 0; i < m.rows; i++){
        for (int j = 0; j < m.cols; j++)
        {
            Matrix_at(m,j,i) = getc(file)-48;
        }
        getc(file);//ignore \n
    }
    
    if(ferror(file))raiseError("Error while reading file");
	fclose(file);

    return m;
}

FlashCount step(Matrix m){
    FlashCount flash = 0;
    bool flashed = true;
    for (int i = 0; i < m.rows; i++){//1
        for (int j = 0; j < m.cols; j++)
        {
            Matrix_at(m,j,i)++;
        }
    }
    while(flashed){
        flashed = false;
        for (int i = 0; i < m.rows; i++){//2
            for (int j = 0; j < m.cols; j++)
            {
                if(Matrix_at(m,j,i) > 9){
                    flashed = true;
                    flash++;
                    Matrix_at(m,j,i) = 0;
                    if(i-1 >= 0 && Matrix_at(m,j,i-1) != 0)Matrix_at(m,j,i-1)++;//up
                    if(j-1 >= 0 && Matrix_at(m,j-1,i) != 0)Matrix_at(m,j-1, i)++;//left
                    if(i+1 < m.rows && Matrix_at(m,j,i+1) != 0)Matrix_at(m,j,i+1)++;//down
                    if(j+1 < m.cols && Matrix_at(m,j+1,i) != 0)Matrix_at(m,j+1,i)++;//right
                    if(i-1 >= 0 && j-1 >= 0 && Matrix_at(m,j-1,i-1) != 0)Matrix_at(m,j-1, i-1)++;//upleft
                    if(i-1 >= 0 && j+1 < m.cols && Matrix_at(m,j+1,i-1) != 0)Matrix_at(m,j+1, i-1)++;//upright
                    if(i+1 < m.rows && j-1 >= 0 && Matrix_at(m,j-1,i+1) != 0)Matrix_at(m,j-1, i+1)++;//downleft
                    if(i+1 < m.rows && j+1 < m.cols && Matrix_at(m,j+1,i+1) != 0)Matrix_at(m,j+1, i+1)++;//downright
                }
            }
        }
    }
    return flash;
}

bool isSim(Matrix m){
    for (int i = 0; i < m.rows; i++){//1
        for (int j = 0; j < m.cols; j++)
        {
            if(Matrix_at(m,j,i) != 0)return false;
        }
    }
    return true;
}

int main(){
    Matrix m = getInput("input.txt");
    int stepcount = 0;

    while(!isSim(m)){
        stepcount++;
        step(m);
        printf("------Step%3d------\n", stepcount);
        for (int i = 0; i < m.rows; i++){//1
        for (int j = 0; j < m.cols; j++)
        {
            printf("%d",Matrix_at(m,j,i));
        }
        printf("\n");
    }
    };
    
    printf("The step is %d\n", stepcount);
    return 0;
}


#include<stdio.h>
#include<stdlib.h>
#define MAX_INT 2147483647



void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}


void getInput(const char* filename, int** pos, size_t *pos_size, int *max, int *min){
	FILE* f = fopen(filename, "r");
    int* crabs;
    size_t size;
    int tmp;
    char poopy;
    *max = 0;
    *min = MAX_INT;

	if(f == NULL)raiseError("Cannot open file");

    while (!feof(f)){
        if(getc(f) == ',')size++;
    }
    size++;

    fseek(f, 0, SEEK_SET);
    crabs = malloc(sizeof(int)*size);

    for (size_t i = 0; i < size; i++){
        fscanf(f, "%d%c", &tmp,&poopy);
        crabs[i] = tmp;
        if(tmp>*max)*max = tmp;
        if(tmp<*min)*min = tmp;
    }

    *pos = crabs;
    *pos_size = size;

	if(ferror(f))raiseError("Error while reading file");
	fclose(f);
}

int Σ(int x){
    int sum = 0;
    for (int i = 0; i <= x; i++)
        sum+=i;
    return sum;
}

int calculateFuelCompsumption(int* c, size_t s, int p){
    int sum = 0, dif;
    for (size_t i = 0; i < s; i++)
    {
        dif = p - c[i];
        if(dif < 0)dif *= -1;
        sum += Σ(dif);
    }
    return sum;
}

int main(){
    int *crabs;
    int max, min;
    size_t csize;
    //ints and positio
    int perfect_pos, fuel_comsumption = MAX_INT;

    getInput("input.txt", &crabs, &csize, &max, &min);
    
    for (int i = min, tmp; i <= max; i++){
        tmp = calculateFuelCompsumption(crabs, csize, i);
        if (tmp < fuel_comsumption)
        {
            perfect_pos = i;
            fuel_comsumption = tmp;
        }
    }
    printf("You have %ld crabs and they all move in position %d consuming %d fuel\n", csize, perfect_pos, fuel_comsumption);

    return 0;
}
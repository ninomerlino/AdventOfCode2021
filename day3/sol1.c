#include<stdio.h>
#include<string.h>

int findGammaEpsilon(const char* filename, int* gamma, int* epsilon){
	FILE* f = fopen(filename, "r");
	//daystuff
	int sampleCount = 0;
	int onemap[12] = {0};
	char value[13];

	if(f == NULL){
		printf("Cannot open file %s\n", filename);
		return -1;
	}

	while(fscanf(f, "%s\n", value) != EOF){
		if(ferror(f)){
			return -1;
			printf("Error while reading file pos : %ld\n", ftell(f));
		}
		for (int i = 0; i < 12; i++)
			if(value[i] == '1')
				onemap[i]++;//check for every bit if is one
			else
				onemap[i]--;
	}
	fclose(f);
	//analazing data
	for (int i = 11; i >= 0; i--)
		if(onemap[11 - i] > 0){//11-i cause the first element of array is the most significant digit
			*gamma += (1 << i);// if the one in pos i are more than half of the sample then gamma += 2^i
		}
	*epsilon = *gamma ^ 4095;
	printf("Read found\n\tGamma = %d\n\tEpsilon = %d\n", *gamma, *epsilon);
	return 0;
}

int main(){
	int g = 0, e = 0;
	if(findGammaEpsilon("input.txt", &g, &e) != 0)
		printf("Error with count fun\n");
	else
		printf("The solution is :%d\n", g*e);
	return 0;
}
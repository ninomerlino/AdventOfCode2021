#include<stdio.h>
#include<string.h>

int count(const char* filename, int* pos, int* depth){
	FILE* f = fopen(filename, "r");
	//daystuff
	int value = 0;
	char command[10];

	if(f == NULL){
		printf("Cannot open file %s\n", filename);
		return -1;
	}

	while(fscanf(f, "%s %d\n", command, &value) != EOF){
		if(ferror(f)){
			return -1;
			printf("Error while reading file pos : %ld\n", ftell(f));

		}
		//daystuff
		if(strcmp("forward", command) == 0){
			*pos += value;
		}else if(strcmp("up", command) == 0){
			*depth -= value;
		}else if(strcmp("down", command) == 0){
			*depth += value;
		}
	}
	fclose(f);
	return 0;
}

int main(){
	int pos = 0, depth = 0;
	if(count("moving.txt", &pos, &depth) != 0)
		printf("Error with count fun\n");
	else
		printf("The solution is :%d\n", pos*depth);
	return 0;
}
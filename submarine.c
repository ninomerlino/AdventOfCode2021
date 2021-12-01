#include<stdio.h>

int count(const char* filename){
	FILE* f = fopen(filename, "r");
	int count = 0, curr = 0, prev = -1;

	if(ferror(f))return -1;

	while(fscanf(f, "%d", &curr) != EOF){
		if(ferror(f)) -1;
		else if(prev != -1 && curr > prev)count++;
		printf("Curr = %d\n", curr);
		prev = curr;
	}
	return count;
}

int main(){
	int sol = count("sub.txt");
	if(sol < 0)
		printf("Error with count fun\n");
	else
		printf("The solution is :%d", sol);
	return 0;
}
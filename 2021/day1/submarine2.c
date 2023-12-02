#include<stdio.h>

int count(const char* filename){
	FILE* f = fopen(filename, "r");
	int count = 0, m1 = -1, m2 = -1, m3 = -1, sum = -1;

	if(ferror(f))return -1;

	while(fscanf(f, "%d", &m3) != EOF){
		if(ferror(f))return -1;
		if(m1 != -1){
			if(sum != -1 && (m1+m2+m3) > sum)count++;
			sum = m1 + m2 + m3;
		}
		m1 = m2;
		m2 = m3;
	}
	fclose(f);
	return count;
}

int main(){
	int sol = count("sub.txt");
	if(sol < 0)
		printf("Error with count fun\n");
	else
		printf("The solution is :%d\n", sol);
	return 0;
}
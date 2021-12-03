#include<stdio.h>
#include<stdlib.h>
#include<string.h>

struct sList{
	char value[13];
	struct sList* next;
};
typedef struct sList* List;

List newNode(const char* newValue){
	List l = malloc(sizeof(struct sList));
	l->next = NULL;
	strcpy(l->value, newValue);
	return l;
}

List readFile(const char* filename){
	FILE* f = fopen(filename, "r");
	List head = NULL, curr;
	char value[13];

	if(f == NULL){
		printf("Cannot open file %s\n", filename);
		return NULL;
	}
	while(fscanf(f, "%s\n", value) != EOF){
		if(ferror(f)){
			return NULL;
			printf("Error while reading file pos : %ld\n", ftell(f));
		}
		if (head == NULL){
			head = newNode(value);
			curr = head;
		}else{
			curr->next = newNode(value);
			curr = curr->next;
		}
	}
	fclose(f);
	return head;
}

void raiseError(const char* message){
	printf("%s\n", message);
	exit(1);
}

void freeList(List head){
	List tmp;
	while(head != NULL){
		head = head->next;
		tmp = head;
		free(tmp);
	}
}

int convert(const char str[13]){
	int tot = 0;
	for (int i = 11; i >= 0; i--)
		if(str[11 - i] == '1'){//11-i cause the first element of array is the most significant digit
			tot += (1 << i);// if the one in pos i are more than half of the sample then gamma += 2^i
		}
	return tot;
}
//for debug
void PrintList(List head){
	putchar('[');
	while(head != NULL){
		printf("%d", convert(head->value));
		if(head->next != NULL){
			putchar(',');
		}else{
			putchar(']');
		}
		head = head->next;
	}
	putchar('\n');
}

char findMostCommon(List head, int off){
	if(off >= 12)raiseError("Offset is too high\n");
	int tot = 0, one = 0;
	while(head != NULL){
		tot++;
		if(head->value[off] == '1')one++;
		head = head->next;
	}
	if(one >= (tot/2))return'1';//if is == to tot/2 will return '1' in the oxigen case which are the one to keep and '1' in the co2 case which is the one to remove
	return '0';
}

int size(List head){
	int tot = 0;
	while(head != NULL){
	tot++;
	head = head->next;
	}
	return tot;
}

List copy(List head){
	List newHead = newNode(head->value);
	List curr = newHead;
	head = head->next;
	while(head != NULL){
		curr->next = newNode(head->value);
		head = head->next;
		curr = curr->next;
	}
	return newHead;
}

int filterForOxigen(List head){
	head = copy(head);
	List curr, tmp, last;
	int off = 0, out;
	char mostCommon;
	//debug info
	while(size(head) > 1){
		curr = head;
		last = NULL;
		mostCommon = findMostCommon(head, off);
		//filter
		while(curr != NULL){
			if(curr->value[off] != mostCommon){
				if(last == NULL){
					//element is head
					head = curr->next;
					free(curr);
					curr = head;
				}else{
					//element is not head
					last->next = curr->next;
					free(curr);
					curr = last->next;
				}
			}else{
				last = curr;
				curr = curr->next;
			}
		}
		//prepare for next iteration
		off++;
	}
	out = convert(head->value);
	free(head);
	return out;
}

int filterForCO2(List head){
	head = copy(head);
	List curr, tmp, last;
	int off = 0, out;
	char mostCommon;
	//debug info
	while(size(head) > 1){
		curr = head;
		last = NULL;
		mostCommon = findMostCommon(head, off);
		//filter
		while(curr != NULL){
			if(curr->value[off] == mostCommon){
				if(last == NULL){
					//element is head
					head = curr->next;
					free(curr);
					curr = head;
				}else{
					//element is not head
					last->next = curr->next;
					free(curr);
					curr = last->next;
				}
			}else{
				last = curr;
				curr = curr->next;
			}
		}
		//prepare for next iteration
		off++;
	}
	out = convert(head->value);
	free(head);
	return out;
}

int main(){
	List values = readFile("input.txt");
	int oxigen = filterForOxigen(values);
	printf("O[%d]\n", oxigen);
	int co2 = filterForCO2(values);
	printf("Solution is CO2[%d]\n", co2);
	printf("Solution is [%d]\n", oxigen*co2);

	freeList(values);
	return 0;
}
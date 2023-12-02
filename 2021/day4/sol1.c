#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>

typedef struct BingoCell{
    short value;
    bool marked;
} BingoCell;

typedef BingoCell BingoCard[5][5];

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

void readRandos(FILE* f, int** r, int* s){
    char c = '\0';
    int value;
    int size = 0;
    int* randos;

    while(c != '\n'){
        if(c == ',')
            size++;
        c = getc(f);
	}
    size++;//count last element

    randos = malloc(sizeof(int) * size);
    fseek(f, 0, SEEK_SET);//reset to start of file

    for (int i = 0; i < size ; i++)
    {
        fscanf(f, "%d%c", &value , &c);
        randos[i] = value;
    }
    *r = randos;
    *s = size;
}

void readCards(FILE* f, BingoCard** ca, int* s){
    char c = '\0', prevc = '\0';
    long start_pos;
    int value;
    int size = 0;
    BingoCard* cards;

    getc(f);//ignore first \n
    start_pos = ftell(f);//save start  pos of first card

    while(!feof(f)){
        if(prevc == '\n' && c == '\n')size++;
        prevc = c;
        c = getc(f);
	}

    cards = malloc(sizeof(BingoCard) * size);
    fseek(f, start_pos, SEEK_SET);//reset to start of file

    for (int i = 0; i < size ; i++)
    {
        for (int row = 0; row < 5; row++)
        {
            for (int col = 0; col < 5; col++)
            {
                fscanf(f, "%d%c", &value, &c);
                cards[i][row][col].value = value;
                cards[i][row][col].marked = false;
            }
        }
        getc(f);//discard \n at end of card
    }
    *ca = cards;
    *s = size;
}

void getInput(const char* filename, int** randos, int* randos_size, BingoCard** cards, int* cards_size){
	FILE* f = fopen(filename, "r");
	if(f == NULL)raiseError("Cannot open file");
    readRandos(f, randos, randos_size);
    readCards(f, cards, cards_size);
	if(ferror(f))raiseError("Error while reading file");
	fclose(f);
}

bool isWinningCard(BingoCard* card){//this is very ugly but i like it
    //rows
    if((*card)[0][0].marked && (*card)[0][1].marked && (*card)[0][2].marked && (*card)[0][3].marked && (*card)[0][4].marked)return true;
    if((*card)[1][0].marked && (*card)[1][1].marked && (*card)[1][2].marked && (*card)[1][3].marked && (*card)[1][4].marked)return true;
    if((*card)[2][0].marked && (*card)[2][1].marked && (*card)[2][2].marked && (*card)[2][3].marked && (*card)[2][4].marked)return true;
    if((*card)[3][0].marked && (*card)[3][1].marked && (*card)[3][2].marked && (*card)[3][3].marked && (*card)[3][4].marked)return true;
    if((*card)[4][0].marked && (*card)[4][1].marked && (*card)[4][2].marked && (*card)[4][3].marked && (*card)[4][4].marked)return true;
    //cols
    if((*card)[0][0].marked && (*card)[1][0].marked && (*card)[2][0].marked && (*card)[3][0].marked && (*card)[4][0].marked)return true;
    if((*card)[0][1].marked && (*card)[1][1].marked && (*card)[2][1].marked && (*card)[3][1].marked && (*card)[4][1].marked)return true;
    if((*card)[0][2].marked && (*card)[1][2].marked && (*card)[2][2].marked && (*card)[3][2].marked && (*card)[4][2].marked)return true;
    if((*card)[0][3].marked && (*card)[1][3].marked && (*card)[2][3].marked && (*card)[3][3].marked && (*card)[4][3].marked)return true;
    if((*card)[0][4].marked && (*card)[1][4].marked && (*card)[2][4].marked && (*card)[3][4].marked && (*card)[4][4].marked)return true;
    return false;
}


int solve(int* randos, int randos_size, BingoCard* cards, int cards_size){
    int card_n = 0, i = 0;
    bool found = false;
    int usum = 0;
    for (;(i < randos_size) && !found; i++)
    {
        card_n = 0;
        for (; (card_n < cards_size) && !found; card_n++)
        {
            for (int row = 0; row < 5; row++)
                for (int col = 0; col < 5; col++)
                    if(cards[card_n][row][col].value == randos[i])cards[card_n][row][col].marked = true;
            
            if(isWinningCard(cards + card_n))found = true;
        }
    }

    if(!found)raiseError("No winning card found!");
    card_n--;//in second for loop card_n increses before checking flag 'found'
    i--;//last number called

    for (int row = 0; row < 5; row++){
        for (int col = 0; col < 5; col++){
            if(!cards[card_n][row][col].marked){
                usum += cards[card_n][row][col].value;
            }
        }
    }

    return randos[i]*usum;
}

int main(){
	BingoCard* cards;
    int* randos;
    int csize, rsize, sol;

    getInput("input.txt", &randos, &rsize, &cards, &csize);
    sol = solve(randos, rsize, cards, csize);
    printf("Solution is :%d\n", sol);

    free(cards);
    free(randos);
	return 0;
}
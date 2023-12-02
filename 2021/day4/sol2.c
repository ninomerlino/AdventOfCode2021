#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>

typedef struct BingoCell{
    short value;
    bool marked;
} BingoCell;

typedef struct BingoCard{
    BingoCell card[5][5];
    bool won;
} BingoCard;

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

void printCard(const BingoCard* c){
    printf("----Bingo!----\n");
    for (int i = 0; i < 5; i++)
    {
        for (int j = 0; j < 5; j++)
            printf("%02d ", c->card[i][j].value);
        putchar('\n');
    }
    printf("--------------\n");
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
    size++;

    cards = malloc(sizeof(BingoCard) * size);
    fseek(f, start_pos, SEEK_SET);//reset to start of file

    for (int i = 0; i < size ; i++)
    {
        for (int row = 0; row < 5; row++)
        {
            for (int col = 0; col < 5; col++)
            {
                fscanf(f, "%d%c", &value, &c);
                cards[i].card[row][col].value = value;
                cards[i].card[row][col].marked = false;
            }
        }
        cards[i].won = false;
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
    if(card->card[0][0].marked && card->card[0][1].marked && card->card[0][2].marked && card->card[0][3].marked && card->card[0][4].marked)return true;
    if(card->card[1][0].marked && card->card[1][1].marked && card->card[1][2].marked && card->card[1][3].marked && card->card[1][4].marked)return true;
    if(card->card[2][0].marked && card->card[2][1].marked && card->card[2][2].marked && card->card[2][3].marked && card->card[2][4].marked)return true;
    if(card->card[3][0].marked && card->card[3][1].marked && card->card[3][2].marked && card->card[3][3].marked && card->card[3][4].marked)return true;
    if(card->card[4][0].marked && card->card[4][1].marked && card->card[4][2].marked && card->card[4][3].marked && card->card[4][4].marked)return true;
    //cols
    if(card->card[0][0].marked && card->card[1][0].marked && card->card[2][0].marked && card->card[3][0].marked && card->card[4][0].marked)return true;
    if(card->card[0][1].marked && card->card[1][1].marked && card->card[2][1].marked && card->card[3][1].marked && card->card[4][1].marked)return true;
    if(card->card[0][2].marked && card->card[1][2].marked && card->card[2][2].marked && card->card[3][2].marked && card->card[4][2].marked)return true;
    if(card->card[0][3].marked && card->card[1][3].marked && card->card[2][3].marked && card->card[3][3].marked && card->card[4][3].marked)return true;
    if(card->card[0][4].marked && card->card[1][4].marked && card->card[2][4].marked && card->card[3][4].marked && card->card[4][4].marked)return true;
    return false;
}


int solve(int* randos,const int randos_size, BingoCard* cards,const int cards_size){
    int i = 0;
    int winners = 0, lastwinner = -1;
    int usum = 0;
    for (;i < randos_size; i++)
    {
        winners = 0;
        for (int card_n = 0; (card_n < cards_size); card_n++)
        {
            if(cards[card_n].won){
                winners++;
                continue;
            }
            for (int row = 0; row < 5; row++)
                for (int col = 0; col < 5; col++)
                    if(cards[card_n].card[row][col].value == randos[i])
                        cards[card_n].card[row][col].marked = true;
            
            if(isWinningCard(cards + card_n)){
                cards[card_n].won = true;
                winners++;
            }
        }
        if(winners == cards_size-1 && lastwinner == -1){//one has yet to win find who
            for (int card_n = 0; card_n < cards_size; card_n++)
            {
                if(!cards[card_n].won){
                    lastwinner = card_n;
                    printf("Last winner %d\n", card_n);
                    break;
                }//find the only one who hasn't won yet and set it as last winner
            }
        }else if(winners == cards_size){//all card won save index of last winning number
            printf("Last %d\n", randos[i]);
            break;
        }
    }

    if(winners != cards_size || lastwinner == -1)raiseError("No last winning card found!");//ended for without all card winning

    for (int row = 0; row < 5; row++){
        for (int col = 0; col < 5; col++){
            if(!cards[lastwinner].card[row][col].marked){
                usum += cards[lastwinner].card[row][col].value;
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
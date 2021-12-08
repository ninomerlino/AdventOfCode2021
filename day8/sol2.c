#include<stdio.h>
#include<stdlib.h>
#include<stdbool.h>
#include<string.h>

typedef enum{ invalid=-1, a, b, c, d, e, f, g} Segments;
typedef Segments Connection[7];
typedef char StrSignal[8];
/*
A Connection is composed by seven segments and connection[a] = f tells that the wire a should be interpreted as f
the signal tells which wires are on so signal[a] and connection[a] = f means that a is active and should be interpreted as f segment
*/

void raiseError(const char* msg){
    fprintf(stderr ,"%s\n", msg);
    exit(1);
}

//converts chart to Segment value(used to interpred the string)
int charToEnum(char letter){
    switch (letter){
    case 'a':
        return a;
    case 'b':
        return b;
    case 'c':
        return c;
    case 'd':
        return d;
    case 'e':
        return e;
    case 'f':
        return f;
    case 'g':
        return g;
    default:
        printf("->%c<-",letter);
        raiseError("Invalid char detected!");
    }
}
//converts str to int with the given connection
int interpretSignal(StrSignal str, Connection con){
    bool realSignal[7] = {0};
    for (int i = 0; i < strlen(str); i++)
    {
        realSignal[con[charToEnum(str[i])]] = true;
    }
    if(realSignal[a] && realSignal[b] && realSignal[f] && realSignal[g]){
        if(realSignal[c] && realSignal[d] && realSignal[e])return 8;
        if(realSignal[c] && realSignal[e])return 0;
        if(realSignal[c] && realSignal[d])return 9;
        if(realSignal[d] && realSignal[e])return 6;
        if(realSignal[d])return 5;
    }
    if(realSignal[a] && realSignal[c] && realSignal[d] && realSignal[g]){
        if(realSignal[e])return 2;
        if(realSignal[f])return 3;
    }
    if(realSignal[c] && realSignal[f]){
        if(realSignal[d] && realSignal[b])return 4;
        if(realSignal[a])return 7;
        return 1;
    }
    return -1;
}
//checks with the given input if the connection is valid
bool validConnection(StrSignal inputs[10], Connection con){
    int lenght, decoded;
    for (int i = 0; i < 10; i++)
    {
        lenght = strlen(inputs[i]);
        decoded = interpretSignal(inputs[i], con);
        if(lenght == 0)raiseError("Error empty string");
        if(lenght == 2 && decoded != 1)return false;
        if(lenght == 3 && decoded != 7)return false;
        if(lenght == 4 && decoded != 4)return false;
        if(lenght == 7 && decoded != 8)return false;
        if(lenght == 5 && !(decoded == 2 || decoded == 5 || decoded == 3))return false;
        if(lenght == 6 && !(decoded == 6 || decoded == 9 || decoded == 0))return false;
    }
    return true;   
}
//really ugly but checks all possible connections to find the valid one
bool findConnection(StrSignal inputs[10], Connection con){
    int try = 0;
    for (int pos_a = 0; pos_a < 7; pos_a++)
        for (int pos_b = 0; pos_b < 7; pos_b++)
            if(pos_b != pos_a)
                for (int pos_c = 0; pos_c < 7; pos_c++)
                    if(pos_c != pos_b && pos_c != pos_a)
                        for (int pos_d = 0; pos_d < 7; pos_d++)
                            if(pos_d != pos_a && pos_d != pos_b && pos_d != pos_c)
                                for (int pos_e = 0; pos_e < 7; pos_e++)
                                    if(pos_e != pos_a && pos_e != pos_b && pos_e != pos_c && pos_e != pos_d) 
                                        for (int pos_f = 0; pos_f < 7; pos_f++)
                                            if(pos_f != pos_a && pos_f != pos_b && pos_f != pos_c && pos_f != pos_d && pos_f != pos_e)
                                                for (int pos_g = 0; pos_g < 7; pos_g++)
                                                    if(pos_g != pos_a && pos_g != pos_b && pos_g != pos_c && pos_g != pos_d && pos_g != pos_e && pos_g != pos_f){
                                                        con[a] = pos_a;
                                                        con[b] = pos_b;
                                                        con[c] = pos_c;
                                                        con[d] = pos_d;
                                                        con[e] = pos_e;
                                                        con[f] = pos_f;
                                                        con[g] = pos_g;
                                                        if(validConnection(inputs, con))return true;//try all combinations
                                                        try++;
                                                    }
    printf("Tried %d combinations\n", try);
    return false;
}
//reads the file parses the 10 inputs than finds a connection with them and interpred the last 4 output with the correct connection
int readfile(const char* filename){
    FILE* file = fopen(filename, "r");
    bool output = false;
    int sum = 0, index_output = 0, index_input = 0, digits[4];
    char tmp = '\0', str[8];
    char inputs[10][8];
    Connection con;

    if(file == NULL)raiseError("Cannot open file");

    memset(con, invalid, sizeof(int)*7);
    memset(inputs, 0, 80);

    while (fscanf(file, "%7s", str) != EOF)
    {
        if(strcmp(str, "|") == 0){
            printf("Read all 10 inputs->");
            output = true;
            index_input = 0;
            if(!findConnection(inputs, con))
                raiseError("Cannot find connection");
            printf("Valid connection found->");
            continue;
        }
        if(index_output == 4){
            printf("Asembling value->");
            index_output = 0;
            sum += 1000*digits[0] + 100*digits[1] + 10*digits[2] + digits[3];
            printf("%d%d%d%d\n", digits[0],digits[1],digits[2],digits[3]);
            output = false;
            memset(con, invalid, sizeof(int)*7);
            memset(inputs, 0, 80);
        }
        if(!output){
            strcpy(inputs[index_input], str);
            index_input++;
        }else{
            digits[index_output] = interpretSignal(str, con);
            index_output++;
        }
    }
    //values are assembled on the next line start so last value has to be asembled outside of the while
    printf("Asembling value->");
    sum += 1000*digits[0] + 100*digits[1] + 10*digits[2] + digits[3];
    printf("%d%d%d%d\n", digits[0],digits[1],digits[2],digits[3]);
    if(ferror(file))raiseError("Error while reading file");
	fclose(file);
    return sum;
}

int main(){
    int sol = readfile("input.txt");
    printf("Solution = %d\n", sol);
    return 0;
}
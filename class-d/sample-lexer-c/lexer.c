#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char operation[] = "5 + 3*(2-5)/4 +3", *p = operation;
    char *array[20];
    
    while (*p) { 
        if ( isdigit(*p) || ((*p=='+') && isdigit(*(p+1))) ) {
            long val = strtol(p, &p, 10); 
            printf("number(%ld)\n", val); 
        } else {
            p++;
        }
    }

    return 0;
}

// gcc lexer.c
// ./a.out
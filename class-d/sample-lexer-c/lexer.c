#include <stdio.h>
#include <ctype.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
    char *operation = "5 + 3*(2-5)/4 +3", *p = operation;

    while (*p) { 
        if ( isdigit(*p) || ((*p=='-'||*p=='+') && isdigit(*(p+1))) ) {
            long val = strtol(p, &p, 10); 
            printf("number(%ld)\n", val); 
        } else {
            p++;
        }
    }

    char buf[] ="5 + 3*(2-5)/4 +3";
    int i = 0;
    char *space = strtok (buf, " ");
    char *array[3];

    while (space != NULL) {
        array[i++] = space;
        space = strtok (NULL, "/");
    }

    for (i = 0; i < 3; ++i) 
        printf("(%s)\n", array[i]);

    return 0;
}

// gcc lexer.c
// ./a.out
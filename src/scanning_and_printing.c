#include <stdio.h>

int scan_and_print() {
    int num;
    printf("Enter an integer: ");
    scanf("%d", &num);
    printf("You entered: %d\n", num);
    return num;
}
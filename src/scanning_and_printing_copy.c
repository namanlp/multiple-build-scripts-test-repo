#include <stdio.h>

int scan_and_print() {
    int num;
    printf("OK! Enter the copy integer: ");
    scanf("%d", &num);
    printf("You copied: %d\n", num);
    return num;
}
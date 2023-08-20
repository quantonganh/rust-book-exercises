#include <stdio.h>

int main() {
    int y;
    int x = (y = 6);
    printf("x: %d, y: %d\n", x, y);
}

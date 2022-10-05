#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int x;
    int y;
} position;

int main(void)
{
    position papa, son;
    int K, L;
    int speed, xn, yn, xdiff, ydiff;

    scanf("%d %d %d", &papa.x, &papa.y, &K);
    scanf("%d %d %d", &son.x, &son.y, &L);

    if ((K - L) > (L - 1))
    {
        speed = K - L;
    }
    else
    {
        speed = L - 1;
    }

    // calculate the x coordinate catch up time
    xdiff = abs(son.x - papa.x);
    if (xdiff % speed == 0) {
        xn = xdiff / speed;
    } else {
        xn = xdiff / speed + 1;
    }

    // calculate the y coordinate catch up time
    ydiff = abs(son.y - papa.y);
    if (ydiff % speed == 0) {
        yn = ydiff / speed;
    } else {
        yn = ydiff / speed + 1;
    }
    printf("%d\n", xn + yn);
}
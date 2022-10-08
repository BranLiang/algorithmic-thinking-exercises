#include <stdio.h>

int find_n(int a, int b, int c)
{
    return a * b + c % a;
}

int main(void)
{
    int q;
    scanf("%d", &q);
    for (int i = 0; i < q; i++)
    {
        int a, b, c;
        scanf("%d %d %d", &a, &b, &c);
        printf("%d\n", find_n(a, b, c));
    }
}
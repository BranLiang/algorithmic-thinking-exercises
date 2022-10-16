#include <stdio.h>

long find_n(long a, long b, long c)
{
    return a * b + c % a;
}

int main(void)
{
    int q;
    scanf("%d", &q);
    for (int i = 0; i < q; i++)
    {
        long a, b, c;
        scanf("%ld %ld %ld", &a, &b, &c);
        printf("%ld\n", find_n(a, b, c));
    }
}
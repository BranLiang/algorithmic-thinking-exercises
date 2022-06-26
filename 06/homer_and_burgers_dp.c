#include <stdio.h>

#define SIZE 100000
#define MAX(x, y) ((x)>(y)?(x):(y))

void solve(int m, int n, int t)
{
    int result, first, second;
    int dp[SIZE];
    dp[0] = 0;
    for (int i = 1; i <= t; i++)
    {
        if (i >= m)
        {
            first = dp[i - m];
        }
        else
        {
            first = -1;
        }

        if (i >= n)
        {
            second = dp[i - n];
        }
        else
        {
            second = -1;
        }

        if (first == -1 && second == -1)
        {
            dp[i] = -1;
        }
        else
        {
            dp[i] = MAX(first, second) + 1;
        }
    }
    result = dp[t];
    if (result >= 0)
    {
        printf("%d\n", result);
    }
    else
    {
        int j = t;
        do
        {
            j--;
            result = dp[j];
        } while (result == -1);
        printf("%d %d\n", result, t - j);
    }
}

int main(void)
{
    int m, n, t;
    while (scanf("%d %d %d", &m, &n, &t) != EOF)
    {
        solve(m, n, t);
    }
}
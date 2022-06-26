#include <stdlib.h>
#include <stdio.h>

#define MEMO_SIZE 10000

int solve_t(int m, int n, int t, int memo[])
{
    int first, second;
    if (memo[t] != -2)
    {
        return memo[t];
    }

    if (t == 0)
    {
        memo[t] = 0;
        return 0;
    }

    if (t >= m)
    {
        first = solve_t(m, n, t - m, memo);
    }
    else
    {
        first = -1;
    }
    
    if (t >= n)
    {
        second = solve_t(m, n, t - n, memo);
    }
    else
    {
        second = -1;
    }
    
    if (first == -1 && second == -1)
    {
        memo[t] = -1;
        return -1;
    }
    
    if (first > second)
    {
        memo[t] = first + 1;
    }
    else
    {
        memo[t] = second + 1;
    }
    return memo[t];
}

void solve(int m, int n, int t)
{
    int memo[MEMO_SIZE];
    for (int j = 0; j < MEMO_SIZE; j++)
    {
        memo[j] = -2;
    }

    int result = solve_t(m, n, t, memo);
    if (result > 0)
    {
        printf("%d\n", result);
    }
    else
    {
        int i = t;
        do
        {
            i--;
            result = solve_t(m, n, i, memo);
        } while (result == -1);
        printf("%d %d\n", result, t - i);
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
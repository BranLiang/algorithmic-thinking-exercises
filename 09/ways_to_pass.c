#include <stdio.h>
#define SIZE 70

int solve(int n, int t, int p, int memo[SIZE + 1][SIZE + 1])
{
    int total = 0;
    int last_mark;

    if (memo[n][t] != -1)
    {
        return memo[n][t];
    }

    if (n == 0 && t == 0)
    {
        return 1;
    }
    if (n == 0)
    {
        return 0;
    }
    
    for (last_mark = p; last_mark <= t; last_mark++)
    {
        total += solve(n - 1, t - last_mark, p, memo);
    }
    memo[n][t] = total;
    return total;
}

int main(void)
{
    int case_num, n, t, p, i, j, k;
    int memo[SIZE + 1][SIZE + 1];
    scanf("%d ", &case_num);
    for (k = 0; k < case_num; k++)
    {
        scanf("%d %d %d", &n, &t, &p);
        for (i = 0; i <= SIZE; i++)
        {
            for (j = 0; j <= SIZE; j++)
            {
                memo[i][j] = -1;
            }
        }
        printf("%d\n", solve(n, t, p, memo));
    }
    return 0;
}
#include <stdio.h>

#define MAX_MATCH_NUM 1000

int max(int x, int y)
{
    if (x > y)
    {
        return x;
    }
    else
    {
        return y;
    }
}

int solve(char outcome_a[], char outcome_b[], int goals_a[], int goals_b[], int a, int b, int memo[MAX_MATCH_NUM + 1][MAX_MATCH_NUM + 1])
{
    int first, second, third, fourth, result;
    if (memo[a][b] != -1)
    {
        return memo[a][b];
    }
    
    if (a == 0 || b == 0)
    {
        memo[a][b] = 0;
        return 0;
    }
    // Option 1: the last match of both a, b matters
    if (
        (outcome_a[a] == 'W' && outcome_b[b] == 'L' && goals_a[a] > goals_b[b]) || 
        (outcome_a[a] == 'L' && outcome_b[b] == 'W' && goals_a[a] < goals_b[b])
    )
    {
        first = solve(outcome_a, outcome_b, goals_a, goals_b, a-1, b-1, memo) + goals_a[a] + goals_b[b];
    }
    else
    {
        first = 0;
    }
    // Option 2: the last match of both a, b doesn't matter
    second = solve(outcome_a, outcome_b, goals_a, goals_b, a-1, b-1, memo);
    // Option 3: only last match of a doesn't matters
    third = solve(outcome_a, outcome_b, goals_a, goals_b, a-1, b, memo);
    // Option 4: only last match of b doesn't matters
    fourth = solve(outcome_a, outcome_b, goals_a, goals_b, a, b-1, memo);

    result = max(max(max(first, second), third), fourth);
    memo[a][b] = result;
    return result;
}

int main(void)
{
    int match_num;
    char outcome_a[MAX_MATCH_NUM + 1], outcome_b[MAX_MATCH_NUM + 1];
    int goals_a[MAX_MATCH_NUM + 1], goals_b[MAX_MATCH_NUM + 1];
    int memo[MAX_MATCH_NUM + 1][MAX_MATCH_NUM + 1];

    scanf("%d ", &match_num);
    for (int i = 1; i <= match_num; i++)
    {
        scanf("%c", &outcome_a[i]);
    }
    for (int i = 1; i <= match_num; i++)
    {
        scanf("%d ", &goals_a[i]);
    }
    for (int i = 1; i <= match_num; i++)
    {
        scanf("%c", &outcome_b[i]);
    }
    for (int i = 1; i <= match_num; i++)
    {
        scanf("%d ", &goals_b[i]);
    }

    for (int i = 0; i <= match_num; i++)
    {
        for (int j = 0; j <= match_num; j++)
        {
            memo[i][j] = -1;
        }
    }
    printf("%d\n", solve(outcome_a, outcome_b, goals_a, goals_b, match_num, match_num, memo));
    return 0;
}
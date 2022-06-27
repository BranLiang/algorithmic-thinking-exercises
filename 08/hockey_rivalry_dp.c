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

int solve(char outcome_a[], char outcome_b[], int goals_a[], int goals_b[], int n)
{
    int first, second, third, fourth;
    int previous[MAX_MATCH_NUM + 1], current[MAX_MATCH_NUM + 1];

    for (int i = 0; i <= n; i++)
    {
        previous[i] = 0;
    }
    
    for (int a = 1; a <= n; a++)
    {
        for (int b = 1; b <= n; b++)
        {
            // Option 1: the last match of both a, b matters
            if (
                (outcome_a[a] == 'W' && outcome_b[b] == 'L' && goals_a[a] > goals_b[b]) || 
                (outcome_a[a] == 'L' && outcome_b[b] == 'W' && goals_a[a] < goals_b[b])
            )
            {
                first = previous[b-1] + goals_a[a] + goals_b[b];
            }
            else
            {
                first = 0;
            }
            
            // Option 2: the last match of both a, b doesn't matter
            second = previous[b-1];
            // Option 3: only last match of a doesn't matters
            third = previous[b];
            // Option 4: only last match of b doesn't matters
            fourth = current[b-1];
            current[b] = max(max(max(first, second), third), fourth);
        }
        for (int k = 0; k <= MAX_MATCH_NUM; k++)
        {
            previous[k] = current[k];
        }
    }
    return current[n];
}

int main(void)
{
    int match_num;
    char outcome_a[MAX_MATCH_NUM + 1], outcome_b[MAX_MATCH_NUM + 1];
    int goals_a[MAX_MATCH_NUM + 1], goals_b[MAX_MATCH_NUM + 1];

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

    printf("%d\n", solve(outcome_a, outcome_b, goals_a, goals_b, match_num));
    return 0;
}
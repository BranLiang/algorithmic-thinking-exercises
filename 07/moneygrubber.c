#include <stdio.h>

#define MAX_K 200
#define MAX_NUM_SCHEMA 20
#define MAX_NUM_ITEMS 100

double min(double x, double y)
{
    if (x < y)
    {
        return x;
    }
    else
    {
        return y;
    }
}

double solve_k(int num[], double price[], int num_schemas, double unit_price, int num_items, double memo[])
{
    double best, result;
    int i;
    if (memo[num_items] != -1)
    {
        return memo[num_items];
    }
    
    if (num_items == 0)
    {
        memo[0] = 0;
        return 0;
    }
    best = solve_k(num, price, num_schemas, unit_price, num_items - 1, memo) + unit_price;
    for (i = 0; i < num_schemas; i++)
    {
        if (num_items - num[i] >= 0)
        {
            result = solve_k(num, price, num_schemas, unit_price, num_items - num[i], memo) + price[i];
            best = min(best, result);
        }
    }
    memo[num_items] = best;
    return best;
}

double solve(int num[], double price[], int num_schemas, double unit_price, int num_items, double memo[])
{
    int k;
    double best, result;
    best = solve_k(num, price, num_schemas, unit_price, num_items, memo);
    for (k = num_items + 1; k < MAX_K; k++)
    {
        result = solve_k(num, price, num_schemas, unit_price, k, memo);
        best = min(best, result);
    }
    return best;
}

int get_number(int *num)
{
    int ch;
    int ret = 0;
    ch = getchar();
    if (ch == ' ')
    {
        return get_number(num);
    }
    
    while (ch != ' ' && ch != '\n')
    {
        ret = 10 * ret + ch - '0';
        ch = getchar();
    }
    *num = ret;
    return ch != '\n';
}

int handle_case(int case_num)
{
    double unit_price, result;
    int i, j;
    int num_schemas, num_items, more;
    int num[MAX_NUM_SCHEMA];
    double price[MAX_NUM_SCHEMA];
    double memo[MAX_K];
    for (i = 0; i < MAX_K; i++)
    {
        memo[i] = -1;
    }
    if (scanf("%lf%d", &unit_price, &num_schemas) == EOF)
    {
        return 0;
    }
    else
    {
        printf("Case %d:\n", case_num);
    }
    
    for (i = 0; i < num_schemas; i++)
    {
        scanf("%d %lf", &num[i], &price[i]);
    }
    scanf(" ");
    do
    {
        more = get_number(&num_items);
        result = solve(num, price, num_schemas, unit_price, num_items, memo);
        printf("Buy %d for $%.2f\n", num_items, result);
    } while (more);
    return 1;
}

int main(void)
{   
    int case_num = 1;
    while (handle_case(case_num))
    {
        case_num += 1;
    }
    return 0;
}
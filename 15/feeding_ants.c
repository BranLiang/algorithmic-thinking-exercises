#define MAX_NODES 1000
#define MAX_LIQUID 2000000000

#include <stdio.h>
#include <stdlib.h>

typedef struct edge
{
    int to_node, percentage, superpipe;
    struct edge *next;
} edge;

int can_feed(int node, double liquid, edge *adj_list[], int liquid_needed[])
{
    int ok;
    edge *e;
    double provided_liquid;
    if (liquid_needed[node] != -1)
    {
        return liquid >= liquid_needed[node];
    }
    ok = 1;
    e = adj_list[node];
    while (e && ok)
    {
        provided_liquid = liquid * e->percentage / 100;
        if (e->superpipe)
        {
            provided_liquid = provided_liquid * provided_liquid;
        }
        if (!can_feed(e->to_node, provided_liquid, adj_list, liquid_needed))
        {
            ok = 0;
        }
        e = e->next;
    }
    return ok;
}

int main(void)
{
    int from_node, node_num;
    edge *e;
    edge *adj_list[MAX_NODES + 1] = {NULL};
    int liquid_needed[MAX_NODES + 1];
    double low, mid, high;

    scanf("%d", &node_num);

    for (int i = 0; i < node_num - 1; i++)
    {
        e = malloc(sizeof(edge));
        if (e == NULL)
        {
            printf("malloc error\n");
            exit(1);
        }
        
        scanf("%d %d %d %d", &from_node, &e->to_node, &e->percentage, &e->superpipe);

        e->next = adj_list[from_node];
        adj_list[from_node] = e;
    }

    for (int i = 1; i <= node_num; i++)
    {
        scanf("%d", &liquid_needed[i]);
    }

    low = 0;
    high = MAX_LIQUID;
    while (high - low > 0.001)
    {
        mid = (high + low) / 2;
        if (can_feed(1, mid, adj_list, liquid_needed))
        {
            high = mid;
        }
        else
        {
            low = mid;
        }
    }
    printf("%.3f\n", high);
}
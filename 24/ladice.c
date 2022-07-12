#include <stdio.h>

#define MAX_N 300000

int find(int parent[], int drawer)
{
    int set = drawer, temp;
    while (parent[set] != set)
    {
        set = parent[set];
    }
    // path compression
    while (parent[drawer] != set)
    {
        temp = parent[drawer];
        parent[drawer] = set;
        drawer = temp;
    }
    return set;
}

void union_sets(int parent[], int drawer_a, int drawer_b)
{
    int set1, set2;
    set1 = find(parent, drawer_a);
    set2 = find(parent, drawer_b);
    // fold set1 into set2
    parent[set1] = set2;
    if (set1 == set2)
    {
        parent[set2] = 0;
    }
    puts("LADICA");
}

int main(void)
{
    int parent[MAX_N];
    int n, d, drawer_a, drawer_b;
    scanf("%d %d", &n, &d);

    for (int i = 0; i <= d; i++)
    {
        parent[i] = i;
    }

    for (int i = 1; i <= n; i++)
    {
        scanf("%d %d", &drawer_a, &drawer_b);
        if (find(parent, drawer_a) == drawer_a)
        {
            union_sets(parent, drawer_a, drawer_b);
        }
        else if (find(parent, drawer_b) == drawer_b)
        {
            union_sets(parent, drawer_b, drawer_a);
        }
        else if (find(parent, drawer_a) > 0)
        {
            union_sets(parent, drawer_a, drawer_b);
        }
        else if (find(parent, drawer_b) > 0)
        {
            union_sets(parent, drawer_b, drawer_a);
        }
        else
        {
            puts("SMECE");
        }
    }
}

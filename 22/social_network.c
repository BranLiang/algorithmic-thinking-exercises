#include <stdio.h>

#define MAX_NUM 100000

int find_community(int community_of[], int x)
{
    int community = x, temp;
    while (community != community_of[community])
    {
        community = community_of[community];
    }

    // Path compression
    while (x != community)
    {
        temp = community_of[x];
        community_of[x] = community;
        x = temp;
    }

    return community;
}

void union_communities(int community_of[], int x, int y, int size[], int max_size)
{
    int community_a, community_b;
    community_a = find_community(community_of, x);
    community_b = find_community(community_of, y);

    if (community_a == community_b)
    {
        return;
    }
    if (size[community_a] + size[community_b] > max_size)
    {
        return;
    }
    
    if (size[community_a] > size[community_b])
    {
        community_of[community_b] = community_a;
        size[community_a] += size[community_b];
    }
    else
    {
        community_of[community_a] = community_b;
        size[community_b] += size[community_a];
    }
}

int main(void)
{
    int n, m, q;
    int i;
    char command;
    int x, y;
    int size[MAX_NUM + 1];
    int community_of[MAX_NUM + 1];

    scanf("%d %d", &n, &m);
    for (i = 1; i <= n; i++)
    {
        size[i] = 1;
        community_of[i] = i;
    }

    scanf("%d ", &q);
    for (i = 0; i < q; i++)
    {
        scanf("%c", &command);
        if (command == 'A')
        {
            scanf("%d %d ", &x, &y);
            union_communities(community_of, x, y, size, m);
        }
        else if (command == 'E')
        {
            scanf("%d %d ", &x, &y);
            if (find_community(community_of, x) == find_community(community_of, y))
            {
                puts("Yes");
            }
            else
            {
                puts("No");
            }
        }
        else if (command == 'S')
        {
            scanf("%d ", &x);
            printf("%d\n", size[find_community(community_of, x)]);
        }
    }

}
#include <stdio.h>
#include <stdlib.h>

#define MAX_N 100001

typedef struct node {
    int weight;
    int to;
    struct node *next;
} node;

void add_node(int from, int to, int weight, node *tree[MAX_N])
{
    node *new_node = malloc(sizeof(node));
    new_node->weight = weight;
    new_node->to = to;
    new_node->next = tree[from];
    tree[from] = new_node;    
}

void find_longest_path(int start, node *tree[MAX_N], int *longest, int *second_longest)
{
    int current_size, new_size;
    int current_locations[MAX_N], new_locations[MAX_N];
    int distances[MAX_N] = {[0 ... MAX_N - 1] = -1};

    distances[start] = 0;
    current_size = 1;
    new_size = 0;
    current_locations[0] = start;

    while (current_size > 0)
    {
        for (int i = 0; i < current_size; i++)
        {
            node *location = tree[current_locations[i]];
            while (location != NULL)
            {
                int to = location->to;
                if (distances[to] == -1)
                {
                    int new_distance = distances[current_locations[i]] + location->weight;
                    if (new_distance >= *longest && to > start)
                    {
                        *second_longest = *longest;
                        *longest = new_distance;
                    }
                    else if (new_distance > *second_longest && to > start)
                    {
                        *second_longest = new_distance;
                    }
                    
                    distances[to] = new_distance;
                    new_locations[new_size] = to;
                    new_size++;
                }
                location = location->next;
            }
        }
        current_size = new_size;
        new_size = 0;
        for (int j = 0; j < current_size; j++)
        {
            current_locations[j] = new_locations[j];
        }
    }
}

int main(void)
{
    int longest, second_longest;
    longest = 0;
    second_longest = 0;

    int n, a, b, weight;
    scanf("%d", &n);
    static node *tree[MAX_N] = {NULL};

    for (int i = 0; i < n - 1; i++)
    {
        scanf("%d %d %d", &a, &b, &weight);

        add_node(a, b, weight, tree);
        add_node(b, a, weight, tree);
    }

    for (int i = 1; i <= n; i++)
    {
        find_longest_path(i, tree, &longest, &second_longest);
    }

    printf("%d\n", second_longest);
}
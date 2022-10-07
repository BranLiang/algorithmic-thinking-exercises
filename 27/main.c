#include <stdio.h>
#include <stdlib.h>

#define MAX_N 100001

typedef struct {
    int from;
    int to;
    int length;
} path;

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

int not_the_same_path(int from, int to, path *path)
{
    if (from == path->to && to == path->from)
    {
        return 0;
    }
    return 1;
}

void find_longest_path(int start, node *tree[MAX_N], path *longest, path *second_longest)
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
                    if (new_distance >= longest->length && not_the_same_path(start, to, longest))
                    {
                        second_longest->from = start;
                        second_longest->to = longest->to;
                        second_longest->length = longest->length;
                        longest->from = start;
                        longest->to = to;
                        longest->length = new_distance;
                    }
                    else if (new_distance > second_longest->length && not_the_same_path(start, to, second_longest) && not_the_same_path(start, to, longest))
                    {
                        second_longest->from = start;
                        second_longest->to = to;
                        second_longest->length = new_distance;
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
    path longest, second_longest;
    longest.length = 0;
    longest.from = -1;
    longest.to = -1;
    second_longest.length = 0;
    second_longest.from = -1;
    second_longest.to = -1;

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
    // printf("1 From: %d\n", longest.from);
    // printf("1 To: %d\n", longest.to);
    // printf("1 Length: %d\n", longest.length);
    // printf("2 From: %d\n", second_longest.from);
    // printf("2 To: %d\n", second_longest.to);
    // printf("2 Length: %d\n", second_longest.length);
    printf("%d\n", second_longest.length);
}
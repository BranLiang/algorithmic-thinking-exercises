#include <stdio.h>
#include <stdlib.h>

#define MAX_Q 200000
#define MAX_N 1000

typedef struct node {
    int to;
    struct node *next;
} node;

int find_distance(int start, int dest, node *hallways[MAX_N + 1])
{
    int distances[MAX_N + 1] = {[0 ... MAX_N] = -1};
    int current_size, new_size;
    int current_locations[MAX_N + 1], new_locations[MAX_N + 1];

    // Only distance known is to the start room, which is 0
    distances[start] = 0;
    current_locations[0] = start;
    current_size = 1;
    new_size = 0;

    while (current_size > 0)
    {
        for (int i = 0; i < current_size; i++)
        {
            int location = current_locations[i];
            if (location == dest)
            {
                return distances[dest];
            }

            node *hallway = hallways[location];
            while (hallway != NULL)
            {
                int to = hallway->to;
                if (distances[to] == -1)
                {
                    distances[to] = distances[location] + 1;
                    new_locations[new_size] = to;
                    new_size++;
                }
                hallway = hallway->next;
            }
        }
        current_size = new_size;
        new_size = 0;
        for (int i = 0; i < current_size; i++)
        {
            current_locations[i] = new_locations[i];
        }
    }
    return distances[dest];
}

int main(void) {
    node *hallways[MAX_N + 1] = {NULL};
    int n, m, t, q;
    int a, b;
    scanf("%d %d %d", &n, &m, &t);

    // read hallways
    for (int i = 0; i < m; i++)
    {
        scanf("%d %d", &a, &b);
        node *hallway = malloc(sizeof(node));
        hallway->to = b;
        hallway->next = hallways[a];
        hallways[a] = hallway;
    }
    
    scanf("%d", &q);
    for (int i = 0; i < q; i++)
    {
        scanf("%d %d", &a, &b);
        int distance = find_distance(a, b, hallways);
        if (distance == -1)
        {
            printf("Not enough hallways!\n");
        }
        else
        {
            printf("%d\n", distance * t);
        }
    }
    return 0;
}
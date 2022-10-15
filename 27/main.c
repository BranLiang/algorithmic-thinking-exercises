#include <stdio.h>
#include <stdlib.h>

#define MAX_N 100001

typedef struct node {
    int weight;
    int to;
    struct node *next;
} node;

typedef struct {
    int from;
    int to;
    int distance;
} route;

void add_node(int from, int to, int weight, node *tree[MAX_N])
{
    node *new_node = malloc(sizeof(node));
    new_node->weight = weight;
    new_node->to = to;
    new_node->next = tree[from];
    tree[from] = new_node;
}

void remove_node(node *tree[MAX_N], int from, node *target)
{
    node *prev = NULL;
    node *cur = tree[from];
    while (cur != NULL) {
        if (cur == target) {
            if (prev == NULL) {
                tree[from] = cur->next;
            } else {
                prev->next = cur->next;
            }
            return;
        }
        prev = cur;
        cur = cur->next;
    }
}

route max_route_single_direction(int from, node *tree[MAX_N])
{
    route max;
    max.distance = 0;
    max.from = from;
    max.to = from;

    node *current = tree[from];
    while (current != NULL) {
        route current_route = max_route_single_direction(current->to, tree);
        int distance = current->weight + current_route.distance;
        if (distance > max.distance) {
            max.distance = distance;
            max.to = current_route.to;
        }
        current = current->next;
    }
    return max;
}

route max_route(int through, node *tree[MAX_N])
{
    route first;
    first.distance = 0;
    first.from = through;
    first.to = through;

    route second;
    second.distance = 0;
    second.from = through;
    second.to = through;

    node *current = tree[through];
    while (current != NULL) {
        route current_route = max_route_single_direction(current->to, tree);
        int distance = current->weight + current_route.distance;
        if (distance > first.distance) {
            second.distance = first.distance;
            second.to = first.to;
            first.distance = distance;
            first.to = current_route.to;
        } else if (distance > second.distance) {
            second.distance = distance;
            second.to = current_route.to;
        }
        current = current->next;
    }

    route max;
    max.distance = first.distance + second.distance;
    max.from = second.to;
    max.to = first.to;

    return max;
}

int include_node(int from, int to, node *tree[MAX_N])
{
    node *current = tree[from];
    while (current != NULL) {
        if (current->to == to || include_node(current->to, to, tree)) {
            return 1;
        }
        current = current->next;
    }
    return 0;
}

int find_edge(node *tree[MAX_N], int from, int to)
{
    node *current = tree[from];
    while (current != NULL) {
        if (current->to == to || include_node(current->to, to, tree)) {
            return current->to;
        }
        current = current->next;
    }
    return 0;
}

route find_max_route(node *tree[MAX_N])
{
    route max;

    for (int i = 1; i < MAX_N; i++)
    {
        if (tree[i] != NULL) {
            route current = max_route(i, tree);
            if (current.distance > max.distance) {
                max = current;
            }
        }
    }
    return max;
}

void solve(node *tree[MAX_N], int reverse_tree[MAX_N])
{
    route max = find_max_route(tree);

    printf("from: %d, to: %d, distance: %d\n", max.from, max.to, max.distance);

    // int to = find_edge(tree, max.from, max.to);

    // printf("node, from: %d, to: %d", max.from, to);

    // node *node1, *node2;
    // route max1, max2;
    // int from;

    // if (reverse_tree[max.from] == 0)
    // {
    //     node1 = find_start(tree, max);
    //     remove_node(tree, max.from, node1);
    //     max1 = find_max_route(tree);
    //     add_node(max.from, node1->to, node1->weight, tree);
    // } else {
    //     from = reverse_tree[max.from];
    //     node1 = tree[from];
    //     remove_node(tree, from, node1);
    //     max1 = find_max_route(tree);
    //     add_node(from, node1->to, node1->weight, tree);
    // }

    // from = reverse_tree[max.to];
    // node2 = tree[from];
    // remove_node(tree, from, node2);
    // max2 = find_max_route(tree);

    // if (max1.distance > max2.distance) {
    //     printf("%d", max1.distance);
    // } else {
    //     printf("%d", max2.distance);
    // }
}

int main(void)
{
    node *tree[MAX_N] = {NULL};
    int reverse_tree[MAX_N];

    for (int i = 0; i < MAX_N; i++)
    {
        reverse_tree[i] = 0;
    }

    int n;
    scanf("%d", &n);

    for (int i = 1; i < n; i++)
    {
        int from, to, weight;
        scanf("%d %d %d", &from, &to, &weight);
        reverse_tree[to] = from;
        add_node(from, to, weight, tree);
    }

    solve(tree, reverse_tree);
}
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NODES 50000

typedef struct
{
    char *label;
    int priority;
} treap_node;

char *read_label(void)
{
    int size = 16;
    char *label = malloc(size);
    char ch = getchar();
    int index = 0;
    while (ch != '/' && ch != '\n' && ch != EOF)
    {
        label[index] = ch;
        index++;
        ch = getchar();
        if (index == size - 1)
        {
            size = size * 2;
            label = realloc(label, size);
            if (label != NULL)
            {
                printf("realloc error\n");
                exit(1);
            }
        }
    }
    label[index] = '\0';
    return label;
}

int comparer(const void *v1, const void *v2)
{
    const treap_node *n1 = v1;
    const treap_node *n2 = v2;
    return strcmp(n1->label, n2->label);
}

int max_priority_index(treap_node treap_nodes[], int left, int right)
{
    int i;
    int max_index = left;
    for (i = left + 1; i <= right; i++)
    {
        if (treap_nodes[i].priority > treap_nodes[max_index].priority)
        {
            max_index = i;
        }
    }
    return max_index;
}

void solve(treap_node treap_nodes[], int left, int right)
{
    int root_index;
    treap_node root;
    if (left > right)
    {
        return;
    }
    root_index = max_priority_index(treap_nodes, left, right);
    root = treap_nodes[root_index];
    printf("(");
    solve(treap_nodes, left, root_index - 1);
    printf("%s/%d", root.label, root.priority);
    solve(treap_nodes, root_index + 1, right);
    printf(")");
}

int main(void)
{
    static treap_node treap_nodes[MAX_NODES];
    int node_num, priority;
    char *label;
    scanf("%d ", &node_num);
    while (node_num > 0)
    {
        for (int i = 0; i < node_num; i++)
        {
            treap_nodes[i].label = read_label();
            scanf("%d ", &treap_nodes[i].priority);
        }
        qsort(treap_nodes, node_num, sizeof(treap_node), comparer);
        solve(treap_nodes, 0, node_num - 1);
        printf("\n");
        scanf("%d ", &node_num);
    }
}

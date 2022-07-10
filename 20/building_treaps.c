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

typedef struct
{
    int left, right;
    int max_index;
} segtree_node;


void init_segtree(segtree_node segtree[], int node, int left, int right)
{
    int mid;
    segtree[node].left = left;
    segtree[node].right = right;
    if (left == right)
    {
        return;
    }
    mid = (left + right) / 2;
    init_segtree(segtree, node * 2, left, mid);
    init_segtree(segtree, node * 2 + 1, mid + 1, right);
}

int fill_segtree(segtree_node segtree[], int node, treap_node treap_nodes[])
{
    int left_max, right_max;
    if (segtree[node].left == segtree[node].right)
    {
        segtree[node].max_index = segtree[node].left;
        return segtree[node].max_index;
    }
    left_max = fill_segtree(segtree, node * 2, treap_nodes);
    right_max = fill_segtree(segtree, node * 2 + 1, treap_nodes);
    if (treap_nodes[left_max].priority > treap_nodes[right_max].priority)
    {
        segtree[node].max_index = left_max;
    }
    else
    {
        segtree[node].max_index = right_max;
    }
    return segtree[node].max_index;
}

int query_segtree(segtree_node segtree[], int node, treap_node treap_nodes[], int left, int right)
{
    int left_max, right_max;
    if (right < segtree[node].left || left > segtree[node].right)
    {
        return -1;
    }
    if (right >= segtree[node].right && left <= segtree[node].left)
    {
        return segtree[node].max_index;
    }
    left_max = query_segtree(segtree, node * 2, treap_nodes, left, right);
    right_max = query_segtree(segtree, node * 2 + 1, treap_nodes, left, right);
    if (left_max == -1)
    {
        return right_max;
    }

    if (right_max == -1)
    {
        return left_max;
    }

    if (treap_nodes[left_max].priority > treap_nodes[right_max].priority)
    {
        return left_max;
    }
    else
    {
        return right_max;
    }
}

void solve(segtree_node segtree[], treap_node treap_nodes[], int left, int right)
{
    int root_index;
    treap_node root;
    if (left > right)
    {
        return;
    }
    root_index = query_segtree(segtree, 1, treap_nodes, left, right);
    root = treap_nodes[root_index];
    printf("(");
    solve(segtree, treap_nodes, left, root_index - 1);
    printf("%s/%d", root.label, root.priority);
    solve(segtree, treap_nodes, root_index + 1, right);
    printf(")");
}

int main(void)
{
    static treap_node treap_nodes[MAX_NODES];
    static segtree_node segtree[MAX_NODES * 4 + 1];
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
        init_segtree(segtree, 1, 0, node_num - 1);
        fill_segtree(segtree, 1, treap_nodes);
        solve(segtree, treap_nodes, 0, node_num - 1);
        printf("\n");
        scanf("%d ", &node_num);
    }
}

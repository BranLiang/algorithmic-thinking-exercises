#define MAX_N 100000

#include <stdio.h>
#include <stdlib.h>

int max(int a, int b)
{
    if (a > b)
    {
        return a;
    }
    else
    {
        return b;
    }
}

typedef struct
{
    int left, right;
    int max_sum, max_element;
} segtree_node;

typedef struct
{
    int max_sum, max_element;
} node_info;

void init_segtree(segtree_node segtree[], int node, int left, int right)
{
    int mid;
    segtree[node].left = left;
    segtree[node].right = right;
    if (left == right)
    {
        return;
    }
    mid = (left + right) /  2;
    init_segtree(segtree, node * 2, left, mid);
    init_segtree(segtree, node * 2 + 1, mid + 1, right);
}

node_info fill_segtree(segtree_node segtree[], int node, int seq[])
{
    node_info left_info, right_info;
    if (segtree[node].left == segtree[node].right)
    {
        segtree[node].max_element = seq[segtree[node].left];
        segtree[node].max_sum = -1;
        return (node_info){ segtree[node].max_sum, segtree[node].max_element };
    }
    left_info = fill_segtree(segtree, node * 2, seq);
    right_info = fill_segtree(segtree, node * 2 + 1, seq);

    segtree[node].max_element = max(
        left_info.max_element,
        right_info.max_element
    );

    if (left_info.max_sum == -1 && right_info.max_sum == -1)
    {
        segtree[node].max_sum = left_info.max_element + right_info.max_element;
    }
    else if (left_info.max_sum == -1)
    {
        segtree[node].max_sum = max(
            left_info.max_element + right_info.max_element,
            right_info.max_sum
        );
    }
    else if (right_info.max_sum == -1)
    {
        segtree[node].max_sum = max(
            left_info.max_element + right_info.max_element,
            left_info.max_sum
        );
    }
    else
    {
        segtree[node].max_sum = max(
            left_info.max_element + right_info.max_element,
            max(
                left_info.max_sum,
                right_info.max_sum
            )
        );
    }
    return (node_info){ segtree[node].max_sum, segtree[node].max_element };
}

node_info query_segtree(segtree_node segtree[], int node, int seq[], int left, int right)
{
    node_info left_info, right_info, ret_info;
    if (left > segtree[node].right || right < segtree[node].left)
    {
        ret_info.max_element = -1;
        ret_info.max_sum = -1;
        return ret_info;
    }
    if (left <= segtree[node].left && right >= segtree[node].right)
    {
        ret_info.max_element = segtree[node].max_element;
        ret_info.max_sum = segtree[node].max_sum;
        return ret_info;
    }
    
    left_info = query_segtree(segtree, node * 2, seq, left, right);
    right_info = query_segtree(segtree, node * 2 + 1, seq, left, right);

    if (left_info.max_element == -1)
    {
        return right_info;
    }

    if (right_info.max_element == -1)
    {
        return left_info;
    }

    ret_info.max_element = max(left_info.max_element, right_info.max_element);
    if (left_info.max_sum == -1 && right_info.max_sum == -1)
    {
        ret_info.max_sum = left_info.max_element + right_info.max_element;
    }
    else if (left_info.max_sum == -1)
    {
        ret_info.max_sum = max(
            left_info.max_element + right_info.max_element,
            right_info.max_sum
        );
    }
    else if (right_info.max_sum == -1)
    {
        ret_info.max_sum = max(
            left_info.max_element + right_info.max_element,
            left_info.max_sum
        );
    }
    else
    {
        ret_info.max_sum = max(
            left_info.max_element + right_info.max_element,
            max(left_info.max_sum, right_info.max_sum)
        );
    }
    
    return ret_info;
}

node_info update_segtree(segtree_node segtree[], int node, int seq[], int index)
{
    node_info left_info, right_info;
    segtree_node left_node, right_node;
    if (segtree[node].left == segtree[node].right)
    {
        segtree[node].max_element = seq[index];
        return (node_info){ segtree[node].max_sum, segtree[node].max_element };
    }
    left_node = segtree[node * 2];
    right_node = segtree[node * 2 + 1];
    if (index <= left_node.right)
    {
        left_info = update_segtree(segtree, node * 2, seq, index);
        right_info = (node_info){ right_node.max_sum, right_node.max_element };
    }
    else
    {
        left_info = (node_info){ left_node.max_sum, left_node.max_element };
        right_info = update_segtree(segtree, node * 2 + 1, seq, index);
    }
    segtree[node].max_element = max(left_info.max_element, right_info.max_element);

    if (left_info.max_sum == -1 && right_info.max_sum == -1)
    {
        segtree[node].max_sum = left_info.max_element + right_info.max_element;
    }
    else if (left_info.max_sum == -1)
    {
        segtree[node].max_sum = max(
            left_info.max_element + right_info.max_element,
            right_info.max_sum
        );
    }
    else if (right_info.max_sum == -1)
    {
        segtree[node].max_sum = max(
            left_info.max_element + right_info.max_element,
            left_info.max_sum
        );
    }
    else
    {
        segtree[node].max_sum = max(
            left_info.max_element + right_info.max_element,
            max(
                left_info.max_sum,
                right_info.max_sum
            )
        );
    }
    
    return (node_info){ segtree[node].max_sum, segtree[node].max_element };
}

int main(void)
{
    int n, q, i;
    char command;
    int a, b;
    segtree_node segtree[MAX_N * 4 + 1];
    node_info result;
    int seq[MAX_N + 1];
    scanf("%d", &n);
    for (i = 1; i <= n; i++)
    {
        scanf("%d", &seq[i]);
    }
    init_segtree(segtree, 1, 1, n);
    fill_segtree(segtree, 1, seq);

    scanf("%d ", &q);
    for (i = 0; i < q; i++)
    {
        scanf("%c%d%d ", &command, &a, &b);
        if (command == 'U')
        {
            seq[a] = b;
            update_segtree(segtree, 1, seq, a);
        }
        else
        {
            result = query_segtree(segtree, 1, seq, a, b);
            printf("%d\n", result.max_sum);
        }
    }
}
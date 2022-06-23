#define SIZE 255
#define TEST_CASES 5

#include <stdlib.h>
#include <stdio.h>

typedef struct node
{
    int candy;
    struct node *left;
    struct node *right; 
} node;

node *new_nonhouse(node *left, node *right)
{
    node *nonhouse = malloc(sizeof(node));
    nonhouse->left = left;
    nonhouse->right = right;
    return nonhouse;
}

node *new_house(int candy)
{
    node *house = malloc(sizeof(node));
    house->candy = candy;
    house->left = NULL;
    house->right = NULL;
    return house;
}

typedef struct stack
{
    node *values[SIZE];
    int highest_used;
} stack;

stack *new_stack()
{
    stack *s = malloc(sizeof(stack));
    s->highest_used = -1;
    return s;
}

void push_stack(stack *s, node *n)
{
    s->highest_used++;
    s->values[s->highest_used] = n;
}

node *pop_stack(stack *s)
{
    node *ret = s->values[s->highest_used];
    s->highest_used--;
    return ret;
}

int is_stack_empty(stack *s)
{
    return s->highest_used == -1;
}

int tree_candy_with_stack(node *tree)
{
    int total = 0;
    stack *s = new_stack();
    while (tree != NULL)
    {
        if (tree->left == NULL && tree->right == NULL)
        {
            total += tree->candy;
            if (is_stack_empty(s))
            {
                tree = NULL;
            }
            else
            {
                tree = pop_stack(s);
            }
        }
        else
        {
            push_stack(s, tree->left);
            tree = tree->right;
        }
    }
    return total;
}

int tree_candy(node *tree)
{
    if (tree->left == NULL && tree->right == NULL)
    {
        return tree->candy;
    }
    else
    {
        return tree_candy(tree->left) + tree_candy(tree->right);
    }
}

int tree_streets(node *tree)
{
    if (tree->left == NULL && tree->right == NULL)
    {
        return 0;
    }
    else
    {
        return tree_streets(tree->left) + tree_streets(tree->right) + 4;
    }
}

int tree_height(node *tree)
{
    if (tree->left == NULL && tree->right == NULL)
    {
        return 0;
    }
    else
    {
        if (tree_height(tree->left) > tree_height(tree->right))
        {
            return tree_height(tree->left) + 1;
        }
        else
        {
            return tree_height(tree->right) + 1;
        }
    }
}

void tree_solve(node *tree)
{
    int candy = tree_candy(tree);
    int height = tree_height(tree);
    int street_num = tree_streets(tree) - height;
    printf("%d %d\n", street_num, candy);
}

node *read_tree_helper(char line[], int *pos)
{
    node *tree;
    tree = malloc(sizeof(node));
    if (tree == NULL)
    {
        fprintf(stderr, "malloc error.\n");
        exit(1);
    }
    if (line[*pos] == '(')
    {
        (*pos)++;
        tree->left = read_tree_helper(line, pos);
        (*pos)++;
        tree->right = read_tree_helper(line, pos);
        (*pos)++;
    }
    else
    {
        tree->left = NULL;
        tree->right = NULL;
        tree->candy = line[*pos] - '0';
        (*pos)++;
        char next = line[*pos];
        if (next != ')' && next != ' ' && next != '\0')
        {
            tree->candy = 10 * tree->candy + line[*pos] - '0';
            (*pos)++;
        }
    }
    return tree;
}

node *read_tree(char line[])
{
    int pos = 0;
    return read_tree_helper(line, &pos);
}

int main(void)
{
    char line[SIZE + 1];
    node *tree;
    for (int i = 0; i < TEST_CASES; i++)
    {
        fgets(line, sizeof(line), stdin);
        tree = read_tree(line);
        tree_solve(tree);
    }
}




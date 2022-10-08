#include <stdlib.h>
#include <stdio.h>

typedef struct node {
    int value;
    struct node *next;
} node;

node *merge(node *a, node *b) {
    node *head = NULL;
    node **tail = &head;
    while (a && b) {
        if (a->value < b->value) {
            *tail = a;
            a = a->next;
        } else {
            *tail = b;
            b = b->next;
        }
        tail = &(*tail)->next;
    }
    *tail = a ? a : b;
    return head;
}

int get_number(int *num)
{
    int ch;
    int ret = 0;
    ch = getchar();
    if (ch == ' ')
    {
        return get_number(num);
    }
    
    while (ch != ' ' && ch != '\n')
    {
        ret = 10 * ret + ch - '0';
        ch = getchar();
    }
    *num = ret;
    return ch != '\n';
}

node *read_line() {
    node *head = NULL;
    node **tail = &head;
    int num;
    while (get_number(&num)) {
        *tail = malloc(sizeof(node));
        (*tail)->value = num;
        tail = &(*tail)->next;
    }
    *tail = NULL;
    return head;
}

int main(void)
{
    int n;
    node *list1 = read_line();
    node *list2 = read_line();
    
    node *head = merge(list1, list2);
    while (head) {
        printf("%d ", head->value);
        head = head->next;
    }
    printf("\n");
}


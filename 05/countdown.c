#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#define hashsize(n) ((unsigned long)1 << (n))
#define hashmask(n) (hashsize(n) - 1)
#define NUM_BITS 12
#define MAX_NAME 10
#define MAX_PEOPLE 1000

unsigned long oaat(char *key, unsigned long len, unsigned long bits) {
    unsigned long hash, i;
    hash = 0;
    for (i = 0; i < len; i++)
    {
        hash += key[i];
        hash += (hash << 10);
        hash ^= (hash >> 6);
    }
    hash += (hash << 3);
    hash ^= (hash >> 11);
    hash += (hash << 15);
    return hash & hashmask(bits);
}

typedef struct mankind
{
    char *name;
    struct mankind **children;
    int num_children;
} mankind;

typedef struct person_node
{
    struct mankind *person;
    struct person_node *next;
} person_node;

mankind *find_person(person_node *people[], char *name)
{
    unsigned long id = oaat(name, strlen(name), NUM_BITS);
    person_node *node = people[id];
    while (node != NULL)
    {
        if (strcmp(node->person->name, name) == 0)
        {
            return node->person;
        }
        else
        {
            node = node->next;
        }
    }
    return NULL;
}

mankind *new_person(char *name)
{
    mankind *person = malloc(sizeof(mankind));
    person->name = name;
    person->num_children = 0;
    return person;
}

mankind *insert_person(person_node *people[], char *name)
{
    mankind *person = new_person(name);
    person_node *new_node = malloc(sizeof(person_node));
    new_node->person = person;

    unsigned long id = oaat(name, strlen(name), NUM_BITS);
    person_node *node = people[id];
    if (node != NULL)
    {
        new_node->next = node;
    }
    else
    {
        new_node->next = NULL;
    }
    people[id] = new_node;
    return new_node->person;
}

mankind *read_line(person_node *people[])
{
    char *parent_name = malloc(MAX_NAME + 1);
    int num_children;
    mankind *parent;
    scanf("%s", parent_name);
    scanf("%d", &num_children);
    parent = find_person(people, parent_name);
    if (parent == NULL)
    {
        parent = insert_person(people, parent_name);
    }
    else
    {
        free(parent_name);
    }
    parent->num_children = num_children;
    parent->children = malloc(sizeof(mankind) * num_children);
    for (int i = 0; i < num_children; i++)
    {
        char *child_name = malloc(MAX_NAME + 1);
        scanf("%s", child_name);
        mankind *child = find_person(people, child_name);
        if (child == NULL)
        {
            child = insert_person(people, child_name);
        }
        else
        {
            free(child_name);
        }
        parent->children[i] = child;
    }
    return parent;
}

int score(mankind *person, int distance)
{
    if (distance == 1)
    {
        return person->num_children;
    }
    else
    {
        int total = 0;
        for (int i = 0; i < person->num_children; i++)
        {
            mankind *child = person->children[i];
            total += score(child, distance - 1);
        }
        return total;
    }
}

typedef struct item
{
    mankind *person;
    int score;
} item;

int item_cmp(const void* elem0, const void* elem1)
{
    const item *item_a = elem0;
    const item *item_b = elem1;
    return item_b->score - item_a->score;
}

void read_case()
{
    item *list[MAX_PEOPLE] = {NULL};
    person_node *people[1 << NUM_BITS] = {NULL};
    int num_lines, distance;
    scanf("%d %d", &num_lines, &distance);
    for (int i = 0; i < num_lines; i++)
    {
        item *new_item = malloc(sizeof(item));
        new_item->person = read_line(people);
        new_item->score = 0;
        list[i] = new_item;
    }
    for (int i = 0; i < num_lines; i++)
    {
        item *item = list[i];
        item->score = score(item->person, distance);
    }
    qsort(list, num_lines, sizeof(item), item_cmp);
    for (int i = 0; i < num_lines; i++)
    {
        item *p = list[i];
        printf("%s %d\n", p->person->name, p->score);
    }
}

int main(void)
{
    int case_num;
    scanf("%d", &case_num);
    for (int i = 0; i < case_num; i++)
    {
        printf("Tree: %d\n", i + 1);
        read_case();
        printf("\n");
    }
}
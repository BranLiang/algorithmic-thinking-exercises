#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_SIZE 1000000

typedef struct node
{
    char *key;
    char *value;
    int timestamp;
    struct node *next;
} node;

typedef struct
{
    node *data[MAX_SIZE];
} time_map;

int hash(char *key)
{
    int hash = 0;
    for (int i = 0; i < strlen(key); i++)
    {
        hash += key[i];
    }
    return hash % MAX_SIZE;
}

void get(time_map *obj, char *key, int timestamp, char *value)
{
    int index = hash(key);

    node *temp = obj->data[index];
    while (temp != NULL)
    {
        if (strcmp(temp->key, key) == 0 && temp->timestamp <= timestamp)
        {
            strcpy(value, temp->value);
        }
        temp = temp->next;
    }
}

void set(time_map *obj, char *key, char *value, int timestamp)
{
    int index = hash(key);

    node *new_node = malloc(sizeof(node));
    new_node->key = malloc(strlen(key) + 1);
    strcpy(new_node->key, key);

    new_node->value = malloc(strlen(value) + 1);
    strcpy(new_node->value, value);

    new_node->timestamp = timestamp;

    if (obj->data[index] == NULL)
    {
        obj->data[index] = new_node;
        return;
    }
    else
    {
        node *current = obj->data[index];
        while (current->next != NULL)
        {
            current = current->next;
        }
        current->next = new_node;
    }
}

int main(void)
{
    int sets, outputs;
    time_map *obj = malloc(sizeof(time_map));
    scanf("%d", &sets);
    for (int i = 0; i < sets; i++)
    {
        char key[100], value[100];
        int timestamp;
        scanf("%s %s %d", key, value, &timestamp);
        set(obj, key, value, timestamp);
    }

    scanf("%d", &outputs);
    for (int i = 0; i < outputs; i++)
    {
        char value[100];
        char key[100];
        int timestamp;
        scanf("%s %d", key, &timestamp);
        get(obj, key, timestamp, value);
        printf("%s\n", value);
    }
}

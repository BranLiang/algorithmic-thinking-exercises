#define SIZE 100000
#include <stdio.h>
#include <assert.h>
#include <stdlib.h>

int identical_right(int snow1[], int snow2[], int start)
{
    int offset, snow2_index;
    for (offset = 0; offset < 6; offset++)
    {
        snow2_index = (start + offset) % 6;
        if (snow1[offset] != snow2[snow2_index])
        {
            return 0;
        }
    }
    return 1;
}

int identical_left(int snow1[], int snow2[], int start)
{
    int offset, snow2_index;
    for (offset = 0; offset < 6; offset++)
    {
        snow2_index = start - offset;
        if (snow2_index < 0)
        {
            snow2_index = snow2_index + 6;
        }
        if (snow1[offset] != snow2[snow2_index])
        {
            return 0;
        }
    }
    return 1;
}

int are_identical(int snow1[], int snow2[])
{
    for (int start = 0; start < 6; start++)
    {
        if (identical_right(snow1, snow2, start))
        {
            return 1;
        }
        if (identical_left(snow1, snow2, start))
        {
            return 1;
        }
    }
    return 0;
}

typedef struct snowflake_node {
    int snowflake[6];
    struct snowflake_node *next;
} snowflake_node;

void identify_identical(snowflake_node *snowflakes[])
{
    snowflake_node *node1, *node2;
    for (int i = 0; i < SIZE; i++)
    {
        node1 = snowflakes[i];
        while (node1 != NULL)
        {
            node2 = node1->next;
            while (node2 != NULL)
            {
                if (are_identical(node1->snowflake, node2->snowflake))
                {
                    printf("Twin snowflakes found.\n");
                    return;
                }
                node2 = node2->next;   
            }
            node1 = node1->next;
        }
    }
    printf("No two snowflakes are alike.\n");
}

int code(int snow[])
{
    int sum = 0;
    for (int i = 0; i < 6; i++)
    {
        sum += snow[i];
    }
    return sum % SIZE;
}

int main(void)
{
    static snowflake_node *snowflakes[SIZE] = {NULL};
    snowflake_node *snow;
    int n, snowflake_code;
    scanf("%d", &n);
    for (int i = 0; i < n; i++)
    {
        snow = malloc(sizeof(snowflake_node));
        if (snow == NULL)
        {
            fprintf(stderr, "malloc error\n");
            exit(1);
        }
        for (int j = 0; j < 6; j++)
        {
            scanf("%d", &snow->snowflake[j]);
        }
        snowflake_code = code(snow->snowflake);
        snow->next = snowflakes[snowflake_code];
        snowflakes[snowflake_code] = snow;
    }
    identify_identical(snowflakes);
    return 0;
}
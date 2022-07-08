#include <stdio.h>
#include <stdlib.h>
#define MAX_RECEIPT_NUM 1000000

typedef struct
{
    int receipt_index, cost;
} heap_element;

void max_heap_insert(heap_element heap[], int *num_heap, int receipt_index, int cost)
{
    int i;
    heap_element temp;
    (*num_heap)++;
    heap[*num_heap] = (heap_element){receipt_index, cost};

    i = *num_heap;
    while (i > 1 && cost > heap[i / 2].cost)
    {
        temp = heap[i];
        heap[i] = heap[i / 2];
        heap[i / 2] = temp;
        i = i / 2;
    }
}

heap_element max_heap_extract(heap_element heap[], int *num_heap)
{
    int i,child;
    heap_element result, temp;
    result = heap[1];
    heap[1] = heap[*num_heap];
    (*num_heap)--;

    i = 1;
    while (i * 2 <= *num_heap)
    {
        child = i * 2;
        if (child < *num_heap && heap[child].cost < heap[child + 1].cost)
        {
            child += 1;
        }
        if (heap[i].cost < heap[child].cost)
        {
            temp = heap[i];
            heap[i] = heap[child];
            heap[child] = temp;
            i = child;
        }
        else
        {
            break;
        }
    }
    return result;
}

void min_heap_insert(heap_element heap[], int *num_heap, int receipt_index, int cost)
{
    int i;
    heap_element temp;
    (*num_heap)++;
    heap[*num_heap] = (heap_element){receipt_index, cost};

    i = *num_heap;
    while (i > 1 && cost < heap[i / 2].cost)
    {
        temp = heap[i];
        heap[i] = heap[i / 2];
        heap[i / 2] = temp;
        i = i / 2;
    }
}

heap_element min_heap_extract(heap_element heap[], int *num_heap)
{
    int i,child;
    heap_element result, temp;
    result = heap[1];
    heap[1] = heap[*num_heap];
    (*num_heap)--;

    i = 1;
    while (i * 2 <= *num_heap)
    {
        child = i * 2;
        if (child < *num_heap && heap[child].cost > heap[child + 1].cost)
        {
            child += 1;
        }
        if (heap[i].cost > heap[child].cost)
        {
            temp = heap[i];
            heap[i] = heap[child];
            heap[child] = temp;
            i = child;
        }
        else
        {
            break;
        }
    }
    return result;
}

int main(void)
{
    int day_num, receipt_num, cost;
    int receipt_index = 0;
    long long total = 0;

    // Used tracks if a specific receipt has been used
    static int used[MAX_RECEIPT_NUM + 1] = {0};

    static heap_element max_heap[MAX_RECEIPT_NUM + 1];
    static heap_element min_heap[MAX_RECEIPT_NUM + 1];
    int max_num_heap = 0;
    int min_num_heap = 0;

    heap_element max_heap_element;
    heap_element min_heap_element;

    scanf("%d", &day_num);
    for (int i = 0; i < day_num; i++)
    {
        scanf("%d", &receipt_num);
        for (int j = 0; j < receipt_num; j++)
        {
            scanf("%d", &cost);
            max_heap_insert(max_heap, &max_num_heap, receipt_index, cost);
            min_heap_insert(min_heap, &min_num_heap, receipt_index, cost);
            receipt_index++;
        }
        max_heap_element = max_heap_extract(max_heap, &max_num_heap);
        while (used[max_heap_element.receipt_index])
        {
            max_heap_element = max_heap_extract(max_heap, &max_num_heap);
        }
        used[max_heap_element.receipt_index] = 1;
        min_heap_element = min_heap_extract(min_heap, &min_num_heap);
        while (used[min_heap_element.receipt_index])
        {
            min_heap_element = min_heap_extract(min_heap, &min_num_heap);
        }
        used[min_heap_element.receipt_index] = 1;
        total += max_heap_element.cost - min_heap_element.cost;
    }
    printf("%lld\n", total);
    return 0;
}
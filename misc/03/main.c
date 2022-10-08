#include <stdio.h>

#define MAX_SIZE 1000000

typedef struct
{
    int counter[MAX_SIZE];
    int size;
} stream;

void init(stream *s)
{
    for (int i = 0; i < MAX_SIZE; i++)
    {
        s->counter[i] = 0;
    }
    s->size = 0;
}

void add_num(stream *obj, int number)
{
    obj->counter[number]++;
    obj->size++;
}

float find_medium(stream *obj)
{
    if (obj->size % 2 == 0)
    {
        int count = 0;
        int i = 0;
        while (count < obj->size / 2)
        {
            i++;
            count += obj->counter[i];
        }
        int j = i;
        while (count < obj->size / 2 + 1)
        {
            j++;
            count += obj->counter[j];
        }
        return (i + j) / 2.0;
    }
    else
    {
        int count = 0;
        int i = 0;
        while (count < obj->size / 2 + 1)
        {
            count += obj->counter[i];
            i++;
        }
        return i - 1;
    }    
}

int main(void)
{
    int n;
    stream s;
    init(&s);
    scanf("%d", &n);
    for (int i = 0; i < n; i++)
    {
        int num;
        scanf("%d", &num);
        add_num(&s, num);
        printf("%.1f\n", find_medium(&s));
    }
}
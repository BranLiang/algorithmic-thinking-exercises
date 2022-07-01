#include <stdio.h>
#include <stdlib.h>

#define MAX_CELLS 101

typedef struct passage
{
    int to_cell;
    int time;
    struct passage *next;
} passage;

int find_min_time(int from_cell, int exit_cell, passage *cells[], int cells_num)
{
    static int fixed[MAX_CELLS] = {0};
    static int min_times[MAX_CELLS];
    int i, j, min_time, min_time_index, found;
    passage *e;
    for (i = 1; i <= cells_num; i++)
    {
        fixed[i] = 0;
        min_times[i] = -1;
    }

    min_times[from_cell] = 0;

    for (i = 1; i <= cells_num; i++)
    {
        min_time = -1;
        found = 0;
        for (j = 1; j <= cells_num; j++)
        {
            if (!fixed[j] && min_times[j] >= 0)
            {
                if (min_time == -1 || min_times[j] < min_time)
                {
                    min_time = min_times[j];
                    min_time_index = j;
                    found = 1;
                }
            }
        }

        if (!found)
        {
            break;
        }

        fixed[min_time_index] = 1;
        e = cells[min_time_index];
        while (e)
        {
            if (
                min_times[e->to_cell] == -1 ||
                (
                    min_times[e->to_cell] > min_time + e->time
                )
            )
            {
                min_times[e->to_cell] = min_time + e->time;
            }
            e = e->next;
        }
    }
    return min_times[exit_cell];
}

int main(void)
{
    int i, j, cases_num, cells_num, exit_cell, time_limit, passages_num;
    int from_cell, total, min_time;
    passage *cells[MAX_CELLS];
    passage *e;
    scanf("%d", &cases_num);
    for (i = 1; i <= cases_num; i++)
    {
        scanf("%d %d %d %d", &cells_num, &exit_cell, &time_limit, &passages_num);

        for (j = 1; j <= cells_num; j++)
        {
            cells[j] = NULL;
        }

        for (j = 0; j < passages_num; j++)
        {
            e = malloc(sizeof(passage));
            scanf("%d %d %d", &from_cell, &e->to_cell, &e->time);
            e->next = cells[from_cell];
            cells[from_cell] = e;
        }

        total = 0;
        for (j = 1; j <= cells_num; j++)
        {
            min_time = find_min_time(j, exit_cell, cells, cells_num);
            if (min_time >= 0 && min_time <= time_limit)
            {
                total += 1;
            }
        }
        printf("%d\n", total);
        if (i < cases_num)
        {
            puts("");
        }
    }
}
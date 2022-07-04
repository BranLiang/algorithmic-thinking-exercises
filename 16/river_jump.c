#include <stdio.h>
#include <stdlib.h>

#define MAX_ROCK_NUM 50000

int comparator(const void *p1, const void *p2)
{
    int n1 = *(const int *)p1;
    int n2 = *(const int *)p2;
    return n1 - n2;
}

int can_make_min_distance(int distance, int rocks[], int rock_num, int length, int remove_num)
{
    int i, removed = 0, prev_rock_loc = 0, cur_rock_loc;
    for (i = 0; i < rock_num; i++)
    {
        cur_rock_loc = rocks[i];
        if (cur_rock_loc - prev_rock_loc < distance)
        {
            removed += 1;
            if (removed > remove_num)
            {
                return 0;
            }
        }
        else
        {
            prev_rock_loc = cur_rock_loc;
        }
    }
    if (length - prev_rock_loc < distance)
    {
        removed += 1;
        if (removed > remove_num)
        {
            return 0;
        }
    }
    return 1;
}

void solve(int rocks[], int rock_num, int length, int remove_num)
{
    int low, mid, high;
    low = 0;
    high = length + 1;
    while (high - low > 1)
    {
        mid = (high + low) / 2;
        if (can_make_min_distance(mid, rocks, rock_num, length, remove_num))
        {
            low = mid;
        }
        else
        {
            high = mid;
        }
    }
    printf("%d\n", low);
}

int main(void)
{
    int rocks[MAX_ROCK_NUM], i;
    int length, rock_num, remove_num;
    scanf("%d %d %d", &length, &rock_num, &remove_num);
    for (i = 0; i < rock_num; i++)
    {
        scanf("%d", &rocks[i]);
    }
    qsort(rocks, rock_num, sizeof(int), comparator);
    solve(rocks, rock_num, length, remove_num);
}
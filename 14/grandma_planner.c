#include <stdio.h>
#include <stdlib.h>

#define MAX_TOWN_NUM 700

typedef struct path
{
    int to_town, length;
    struct path *next;
} path;

enum {
    WITHOUT_COOKIE,
    WITH_COOKIE
};

void solve(int stores[], int town_num, path *paths[])
{
    static int min_lengths[MAX_TOWN_NUM + 1][2];
    static int min_path_nums[MAX_TOWN_NUM + 1][2];
    static int done[MAX_TOWN_NUM + 1][2];
    int i, j, found, min_length, min_length_town, min_length_state, old_length, new_length;
    int state;
    path *e;

    // Initialize min_lengths and min_path_nums
    for (i = 1; i <= town_num; i++)
    {
        for (state = 0; state <= 1; state++)
        {
            min_lengths[i][state] = -1;
            min_path_nums[i][state] = 0;
            done[i][state] = 0;
        }
    }
    min_lengths[1][WITHOUT_COOKIE] = 0;
    min_path_nums[1][WITHOUT_COOKIE] = 1;

    for (i = 1; i <= town_num * 2; i++)
    {
        found = 0;
        min_length = -1;
        for (state = 0; state <= 1; state++)
        {
            for (j = 1; j <= town_num; j++)
            {
                if (!done[j][state] && min_lengths[j][state] >= 0)
                {
                    if (min_length == -1 || min_lengths[j][state] < min_length)
                    {   
                        min_length_town = j;
                        min_length = min_lengths[j][state];
                        min_length_state = state;
                        found = 1;
                    }
                }
            }

            if (!found)
            {
                break;
            }
            
            done[min_length_town][min_length_state] = 1;
            
            if (min_length_state == WITHOUT_COOKIE && stores[min_length_town])
            {
                old_length = min_lengths[min_length_town][WITH_COOKIE];
                if (old_length == -1 || old_length >= min_length)
                {
                    min_lengths[min_length_town][WITH_COOKIE] = min_length;
                    if (old_length == min_length)
                    {
                        min_path_nums[min_length_town][WITH_COOKIE] += min_path_nums[min_length_town][WITHOUT_COOKIE];
                    }
                    else
                    {
                        min_path_nums[min_length_town][WITH_COOKIE] = min_path_nums[min_length_town][WITHOUT_COOKIE];
                    }
                    min_path_nums[min_length_town][WITH_COOKIE] %= 1000000;
                }   
            }
            else
            {
                e = paths[min_length_town];
                while (e)
                {
                    old_length = min_lengths[e->to_town][min_length_state];
                    new_length = min_length + e->length;
                    if (old_length == -1 || old_length >= new_length)
                    {
                        min_lengths[e->to_town][min_length_state] = new_length;
                        if (old_length == new_length)
                        {
                            min_path_nums[e->to_town][min_length_state] += min_path_nums[min_length_town][min_length_state];
                        }
                        else
                        {
                            min_path_nums[e->to_town][min_length_state] = min_path_nums[min_length_town][min_length_state];
                        }
                        min_path_nums[e->to_town][min_length_state] %= 1000000;
                    }
                    e = e->next;
                }
            }
        }
    }
    printf("%d %d\n", min_lengths[town_num][WITH_COOKIE], min_path_nums[town_num][WITH_COOKIE]);
}

int main(void)
{
    int store_num, town_num, from_town, to_town, store_index, length;
    
    int stores[MAX_TOWN_NUM + 1] = {0};
    path *paths[MAX_TOWN_NUM + 1] = {NULL};

    scanf("%d", &town_num);

    // Link all paths
    for (from_town = 1; from_town <= town_num; from_town++)
    {
        for (to_town = 1; to_town <= town_num; to_town++)
        {
            scanf("%d", &length);
            if (from_town != to_town)
            {
                path *new_path = malloc(sizeof(path));
                new_path->length = length;
                new_path->to_town = to_town;
                new_path->next = paths[from_town];
                paths[from_town] = new_path;
            }
        }
    }

    // Initialize stores
    scanf("%d", &store_num);
    for (int i = 0; i < store_num; i++)
    {
        scanf("%d", &store_index);
        stores[store_index] = 1;
    }

    solve(stores, town_num, paths);
}
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define SetBit(A,k)     ( A[(k/32)] |= (1 << (k%32)) )
#define TestBit(A,k)    ( A[(k/32)] & (1 << (k%32)) )

#define SIZE 1000000

typedef enum state {
    UP,
    DOWN
} state;

typedef struct position
{
    int height;
    state state;
} position;


void find_moves(int h, int j, int32_t *itchings, int min_moves[SIZE * 2][2])
{
    static position cur_positions[SIZE * 4];
    int cur_positions_num = 1;

    static position new_positions[SIZE * 4];
    int new_positions_num;

    int from_height, dest_height, distance;
    state from_state;

    cur_positions[0] = (position){0, UP};

    while (cur_positions_num > 0)
    {
        new_positions_num = 0;
        for (int i = 0; i < cur_positions_num; i++)
        {
            from_height = cur_positions[i].height;
            from_state = cur_positions[i].state;

            if (from_state == UP)
            {
                // Add up position to the next iterator
                dest_height = from_height + j;
                distance = 1 + min_moves[from_height][UP];
                if (
                    dest_height < (h + j) &&
                    !TestBit(itchings, dest_height) &&
                    (
                        min_moves[dest_height][UP] == -1 ||
                        min_moves[dest_height][UP] > distance
                    )
                )
                {
                    min_moves[dest_height][UP] = distance;
                    new_positions[new_positions_num] = (position){ dest_height, UP };
                    new_positions_num += 1;
                }

                // Add down positions to the next iterator
                dest_height = from_height;
                distance = 1 + min_moves[from_height][UP];
                if (
                    min_moves[dest_height][DOWN] == -1 ||
                    min_moves[dest_height][DOWN] > distance
                )
                {
                    min_moves[dest_height][DOWN] = distance;
                    new_positions[new_positions_num] = (position) { dest_height, DOWN };
                    new_positions_num += 1;
                }
            }
            else
            {
                // For those the distance remains the same, it is added to the current iterator
                // Add down positions to the current iterator
                dest_height = from_height - 1;
                distance = min_moves[from_height][DOWN];
                if (
                    dest_height >= 0 &&
                    (
                        min_moves[dest_height][DOWN] == -1 ||
                        min_moves[dest_height][DOWN] > distance
                    )
                )
                {
                    min_moves[dest_height][DOWN] = distance;
                    cur_positions[cur_positions_num] = (position) { dest_height, DOWN };
                    cur_positions_num += 1;
                }
                
                // Add up position to the current iterator
                dest_height = from_height;
                distance = min_moves[from_height][DOWN];
                if (
                    !TestBit(itchings, dest_height) &&
                    (
                        min_moves[dest_height][UP] == -1 ||
                        min_moves[dest_height][UP] > distance
                    )
                )
                {
                    min_moves[dest_height][UP] = distance;
                    cur_positions[cur_positions_num] = (position) { dest_height, UP };
                    cur_positions_num += 1;
                }   
            }
        }
        

        for (int i = 0; i < new_positions_num; i++)
        {
            cur_positions[i] = new_positions[i];
        }
        cur_positions_num = new_positions_num;
    }
}

int main(void)
{
    int h, j, n, segment_start, segment_end, min_move;
    int i, k;
    // Intialize the result with -1, which means it can't climb to the target height
    int result = -1;
    scanf("%d %d %d", &h, &j, &n);

    // Initialize the min_moves all to be -1, which means we don't have any knowledge of min moves yet
    static int min_moves[SIZE * 2][2];
    for (i = 0; i < SIZE * 2; i++)
    {
        min_moves[i][UP] = -1;
        min_moves[i][DOWN] = -1;
    }
    min_moves[0][UP] = 0;
    
    // Intialize the itching segments
    int32_t *itchings = malloc(sizeof(int32_t) * (h / 32 + 1 + j / 32 + 1));
    for (i = 0; i < n; i++)
    {
        scanf("%d %d", &segment_start, &segment_end);
        for (k = segment_start; k <= segment_end; k++)
        {
            SetBit(itchings, k);
        }
    }
    // Fill min_moves using BFS strategy
    find_moves(h, j, itchings, min_moves);

    // Find the minimum moves among all the possibilities
    for (i = h; i < (h + j); i++)
    {
        min_move = min_moves[i][UP];
        if (min_move != -1 && (min_move < result || result == -1))
        {
            result = min_move;
        }
    }
    
    printf("%d\n", result);
}
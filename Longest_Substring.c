#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#define MAX_NUM_CHARS 26

// O(n^2) implementation

// int lengthOfLongestSubstring(char *s)
// {
//     char temp_substring[256] = "";
//     char longest_substring[256] = "";
//     char current_char[2] = "";
//     int size = strlen(s);

//     for (int index = 0; index < size; index++)
//     {
//         current_char[0] = s[index];

//         if (strchr(temp_substring, current_char[0]))
//         {
//             if (strlen(temp_substring) > strlen(longest_substring))
//             {
//                 strcpy(longest_substring, temp_substring);
//             }
//              // Find the first occurrence of the current character in temp_substring
//             char *first_occurance = strchr(temp_substring, current_char[0]);
//             // Copy all characters from the first occurrence to the end of temp_substring to the new temp_substring
//             strcpy(temp_substring, first_occurance + 1);
//         }
//         strcat(temp_substring, current_char);
//     }
//     return strlen(longest_substring);
// }

// O(n) implementation using a hastable sliding window

int lengthOfLongestSubstring(char *s)
{
    int start = 0;
    int end = 0;
    int max_length = 0;
    bool char_set[128] = {false};
    int size = strlen(s);

    while (end < size)
    {
        if (!char_set[s[end]])
        {
            char_set[s[end]] = true;
            end++;
            max_length = (end - start > max_length) ? end - start : max_length;
        }
        else
        {
            char_set[s[start]] = false;
            start++;
        }
    }
    return max_length;
}

int main()
{
    char *s = "abcabcbb";
    printf("%d\n", lengthOfLongestSubstring(s));
    return 0;
}

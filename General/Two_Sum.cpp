// Thought Process
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target

// First idea:
// Loop through the array adding the nums and compare with target
#include <iostream>
#include <vector>
#include <unordered_map>
using namespace std;

// O(n^2)

// class Solution
// {
// public:
//     vector<int> twoSum(vector<int> &nums, int target)
//     {

//         int size = nums.size();

//         for (int index = 0; index < size; index++)
//         {
//             for (int index_2 = index + 1; index_2 < size; index_2++)
//             {
//                 int sum = nums[index] + nums[index_2];
//                 if (sum == target)
//                 {
//                     return {index, index_2};
//                 }
//             }
//         }
//         return {};
//     }
// };

// O(n) time complexity
// Second idea store the array in a hashmap then loop through the array once and check if
// the complement i.e: target - index is in the array

class Solution
{
public:
    vector<int> twoSum(vector<int> &nums, int target)
    {
        int size = nums.size();
        unordered_map<int, int> hashmap;
        for (int index = 0; index < size; index++)
        {
            int complement = target - nums[index];
            auto found = hashmap.find(complement);
            if (found != hashmap.end())
            {
                return {found->second, index};
            }
            hashmap[nums[index]] = index;
        }
        return {};
    }
};

int main()
{
    Solution sol;
    std::vector<int> nums = {2, 7, 11, 15};
    int target = 9;
    std::vector<int> result = sol.twoSum(nums, target);
    if (result == std::vector<int>{0, 1})
    {
        std::cout << "Test passed!" << std::endl;
    }
    else
    {
        std::cout << "Test failed!" << std::endl;
        std::cout << "Result: [";
        for (size_t i = 0; i < result.size(); ++i)
        {
            std::cout << result[i];
            if (i != result.size() - 1)
            {
                std::cout << ", ";
            }
        }
        std::cout << "]" << std::endl;
    }
    return 0;
}
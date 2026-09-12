#include <ranges>

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
       unordered_map<int, int> complements; 

       for(int i = 0; i < nums.size(); ++i) {
            int complement = target - nums[i];
            if(complements.contains(complement)) {
                return { complements[complement], i };
            }

            complements.insert({nums[i], i});
       }

       return { -1, -1 };
    }
};

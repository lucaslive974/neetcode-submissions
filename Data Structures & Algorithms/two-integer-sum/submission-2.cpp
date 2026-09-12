#include <ranges>

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
       unordered_map<int, int> complements; 

       for(auto [ idx,  num ] : std::ranges::views::enumerate(nums)) {
            int complement = target - num;
            if(complements.contains(complement)) {
                return { complements[complement], static_cast<int>(idx) };
            }

            complements.insert({num, idx});
       }

       return { -1, -1 };
    }
};

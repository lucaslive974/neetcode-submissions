class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        std::unordered_map<int, int> complements;    

        int idx = 0;
        for(auto num : nums) {
            int complement = target - num;
            if(complements.contains(complement)) {
                return { complements[complement], idx };
            }

            complements.insert({num, idx});
            ++idx;
        }

        return { -1, -1 };
    }
};

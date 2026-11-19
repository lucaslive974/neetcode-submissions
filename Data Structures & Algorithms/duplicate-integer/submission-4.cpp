class Solution {
public:
    bool hasDuplicate(vector<int>& nums) {
        std::unordered_set<int> duplicates;

        for(auto num : nums) {
            if(duplicates.contains(num))
                return true;

            duplicates.insert(num);
        }


        return false;
    }
};
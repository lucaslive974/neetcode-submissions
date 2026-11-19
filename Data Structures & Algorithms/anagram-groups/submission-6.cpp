class Solution {
public:
    vector<vector<string>> groupAnagrams(vector<string>& strs) {
        unordered_map<string, vector<string>> groups;

        for(auto &str : strs) {
            auto cstr = str; 
            ranges::sort(cstr);
            groups[cstr].push_back(str);
        }

        vector<vector<string>> result;
        for(auto &[key, group] : groups) 
            result.push_back(std::move(group));
       
        return result;
    }
};

class Solution {
public:
    bool isAnagram(string s, string t) {
        if(s.size() != t.size())
            return false;

        std::unordered_map<char, unsigned int> sm;
        for(char &c : s) {
            sm[c]++;
        }

        std::unordered_map<char, unsigned int> tm;
        for(char &c : t) {
            tm[c]++;
        }


        for(auto &[key, vals] : sm) {
            auto valt = tm[key];
            if(vals != valt)
                return false;
        }

        return true;
    }
};

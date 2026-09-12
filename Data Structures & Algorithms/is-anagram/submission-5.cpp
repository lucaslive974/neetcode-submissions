class Solution {
public:
    bool isAnagram(string s, string t) {
       unordered_map<int, int> s1;
       unordered_map<int, int> s2;

       for(char &c : s) {
         s1[c]++;
       } 

       for(char &c : t) {
        s2[c]++;
       }

       if(s1.size() != s2.size())
        return false;

       for(auto [ ch, freq ] : s1) {
            if(s2[ch] != freq)
                return false;
       }

       return true;
    }
};

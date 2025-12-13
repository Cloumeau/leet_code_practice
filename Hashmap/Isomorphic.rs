use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_to_t: HashMap<char, char> = HashMap::new();
        let mut t_to_s: HashMap<char, char> = HashMap::new();

        for (cs, ct) in s.chars().zip(t.chars()) {
            // s → t mapping must stay consistent
            if let Some(&mapped) = s_to_t.get(&cs) {
                if mapped != ct {
                    return false;
                }
            } else {
                s_to_t.insert(cs, ct);
            }

            // t → s mapping must also stay consistent
            if let Some(&mapped) = t_to_s.get(&ct) {
                if mapped != cs {
                    return false;
                }
            } else {
                t_to_s.insert(ct, cs);
            }
        }

        true
    }
}

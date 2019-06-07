use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Eq, PartialEq)]
struct Skill {
    /// List of parents of this skill
    parents: Vec<SkillParentRelation>,
    /// How much experience (directly or through children) the skill has gained
    exp_progress: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct SkillParentRelation {
    /// Index of the parent in the tree
    index: usize,
    /// The strength of the parent relation; each xp gained to the skill will apply strength
    /// experience to the parent as well (and recursively)
    strength: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SkillTree {
    skills: Vec<Skill>,
    lookup: HashMap<String, usize>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ExpGain<'a> {
    exp: usize,
    skill_name: &'a str,
}

// TODO: error handling
impl SkillTree {
    pub fn new() -> SkillTree {
        SkillTree {
            skills: Vec::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn with_skill(mut self, addition: SkillAddition) -> Result<SkillTree, ()> {
        if self.lookup.contains_key(&addition.name) {
            return Err(());
        }

        let next_index = self.skills.len();
        self.lookup.insert(addition.name, next_index);

        let mut new_skill = Skill {
            parents: Vec::with_capacity(addition.parents.len()),
            exp_progress: 0,
        };

        for parent in &addition.parents {
            if let Some(&index) = self.lookup.get(&parent.name) {
                new_skill.parents.push({
                    SkillParentRelation {
                        index,
                        strength: parent.strength,
                    }
                });
            } else {
                return Err(());
            }
        }

        self.skills.push(new_skill);

        Ok(self)
    }

    fn gain_experience_ind(&mut self, index: usize, exp_gain: usize) {
        let mut skill = self.skills.get_mut(index).expect("indices should be valid!");

        skill.exp_progress += exp_gain;

        // We need to swap out the parents vec so we can release the borrw on skill/self ...
        let parents = std::mem::replace(&mut skill.parents, Vec::new());

        for &reln in &parents {
            self.gain_experience_ind(reln.index, exp_gain * reln.strength);
        }

        // ... but now we need to grab skill back ...
        let skill = self.skills.get_mut(index).expect("indices should be valid!");

        // ... and restore its parents
        std::mem::replace(&mut skill.parents, parents);

        // borrow checker makes this weird but idw to clone that vec if I can avoid it
    }

    pub fn gain_experience<'a>(&mut self, exp_gain: ExpGain<'a>) -> Result<(), ()> {
        if let Some(&skill_index) = self.lookup.get(exp_gain.skill_name) {
            self.gain_experience_ind(skill_index, exp_gain.exp);

            Ok(())
        } else {
            Err(())
        }
    }

    pub fn check_experience<'a>(&self, skill_name: &'a str) -> Result<usize, ()> {
        if let Some(&ind) = self.lookup.get(skill_name) {
            Ok(self.skills[ind].exp_progress)
        } else {
            Err(())
        }
    }

    pub fn from_ron(ron: &str) -> Result<SkillTree, ()> {
        let additions: Vec<SkillAddition> = ron::de::from_str(ron).map_err(|_e| {})?;

        let mut tree = SkillTree::new();

        for addition in additions {
            tree = tree.with_skill(addition)?;
        }

        Ok(tree)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SkillAddition {
    pub name: String,
    #[serde(default)]
    pub parents: Vec<SkillParentRelationAddition>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SkillParentRelationAddition {
    pub name: String,
    #[serde(default = "one")]
    pub strength: usize,
}

#[inline(always)]
fn one() -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        let ron = "[]";

        let actual = SkillTree::from_ron(ron).expect("Should deserialize");
        let expected = SkillTree::new();

        assert_eq!(expected, actual);
    }

    #[test]
    fn one_test() {
        let ron = r#"[(name: "root", parents: [])]"#;

        let actual = SkillTree::from_ron(ron).expect("Should deserialize");

        let mut lookup = HashMap::new();
        lookup.insert("root".to_string(), 0);

        let expected = SkillTree {
            skills: vec![Skill {
                parents: vec![],
                exp_progress: 0,
            }],
            lookup,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn two_root_test() {
        let ron = r#"[
            (name: "root", parents: []), 
            (name: "root2", parents: [])
        ]"#;

        let actual = SkillTree::from_ron(ron).expect("Should deserialize");

        let mut lookup = HashMap::new();
        lookup.insert("root".to_string(), 0);
        lookup.insert("root2".to_string(), 1);

        let expected = SkillTree {
            skills: vec![
                Skill {
                    parents: vec![],
                    exp_progress: 0,
                },
                Skill {
                    parents: vec![],
                    exp_progress: 0,
                },
            ],
            lookup,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn non_trivial_test() {
        let ron = r#"[
            (name: "root", parents: []),
            (name: "r_c", parents: [(name: "root", strength: 2)]),
            (name: "root2", parents: []),
            (name: "r_c2", parents: [(name: "root2", strength: 21)]),
            (
                name: "r_c12",
                parents: [
                    (name: "root2"),
                    (name: "root", strength: 12)
                ]
            )
        ]"#;

        let actual = SkillTree::from_ron(ron).expect("Should deserialize");

        let mut lookup = HashMap::new();
        lookup.insert("root".to_string(), 0);
        lookup.insert("r_c".to_string(), 1);
        lookup.insert("root2".to_string(), 2);
        lookup.insert("r_c2".to_string(), 3);
        lookup.insert("r_c12".to_string(), 4);

        let expected = SkillTree {
            skills: vec![
                Skill {
                    parents: vec![],
                    exp_progress: 0,
                },
                Skill {
                    parents: vec![SkillParentRelation { index: 0, strength: 2 }],
                    exp_progress: 0,
                },
                Skill {
                    parents: vec![],
                    exp_progress: 0,
                },
                Skill {
                    parents: vec![SkillParentRelation { index: 2, strength: 21 }],
                    exp_progress: 0,
                },
                Skill {
                    parents: vec![
                        SkillParentRelation { index: 2, strength: 1 },
                        SkillParentRelation { index: 0, strength: 12 },
                    ],
                    exp_progress: 0,
                },
            ],
            lookup,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn non_trivial_test_exp_gain() {
        let ron = r#"[
            (name: "root", parents: []),
            (name: "r_c", parents: [(name: "root", strength: 2)]),
            (name: "root2", parents: []),
            (name: "r_c2", parents: [(name: "root2", strength: 21)]),
            (name: "r_c12", parents: [
                (name: "root2", strength: 2),
                (name: "root", strength: 12)
            ])
        ]"#;

        let mut actual = SkillTree::from_ron(ron).expect("Should deserialize");
        actual
            .gain_experience(ExpGain {
                exp: 5,
                skill_name: "root",
            })
            .expect("name is valid so this should work");

        actual
            .gain_experience(ExpGain {
                exp: 13,
                skill_name: "r_c12",
            })
            .expect("name is valid so this should work");

        actual
            .gain_experience(ExpGain {
                exp: 18,
                skill_name: "bogus",
            })
            .expect_err("name is not valid so this should not do anything");

        let mut lookup = HashMap::new();
        lookup.insert("root".to_string(), 0);
        lookup.insert("r_c".to_string(), 1);
        lookup.insert("root2".to_string(), 2);
        lookup.insert("r_c2".to_string(), 3);
        lookup.insert("r_c12".to_string(), 4);

        let expected = SkillTree {
            skills: vec![
                Skill {
                    parents: vec![],
                    exp_progress: 5 + 12 * 13,
                },
                Skill {
                    parents: vec![SkillParentRelation { index: 0, strength: 2 }],
                    exp_progress: 0,
                },
                Skill {
                    parents: vec![],
                    exp_progress: 26,
                },
                Skill {
                    parents: vec![SkillParentRelation { index: 2, strength: 21 }],
                    exp_progress: 0,
                },
                Skill {
                    parents: vec![
                        SkillParentRelation { index: 2, strength: 2 },
                        SkillParentRelation { index: 0, strength: 12 },
                    ],
                    exp_progress: 13,
                },
            ],
            lookup,
        };

        assert_eq!(expected, actual);

        assert_eq!(actual.check_experience("root"), Ok(5 + 12 * 13));
        assert_eq!(actual.check_experience("r_c12"), Ok(13));
        assert_eq!(actual.check_experience("bonus_name"), Err(()));
    }
}

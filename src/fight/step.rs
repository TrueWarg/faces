use bevy::utils::HashMap;

use crate::rpg::{DirectionalAttack, RangedProp, TargetProps};

#[derive(Default, Debug, Eq, PartialEq)]
pub struct StepDecision {
    target_id: usize,
    attack_id: usize,
}

pub fn decide_next_step(
    attacks: &Vec<DirectionalAttack>,
    targets: &HashMap<usize, TargetProps>,
) -> StepDecision {
    let mut min_health = f32::MAX;
    let mut target_id = usize::MAX;
    let mut attack_id = 0;

    for (id, target) in targets.iter() {
        for (pos, attack) in attacks.iter().enumerate() {
            let m_e = mathematical_expectation(attack, target);
            if m_e < min_health {
                min_health = m_e;
                target_id = id.clone();
                attack_id = pos;
            }
        }
    }

    if target_id == usize::MAX {
        panic!("Target not selected")
    }

    return StepDecision { target_id, attack_id };
}

fn mathematical_expectation(attack: &DirectionalAttack, target: &TargetProps) -> f32 {
    if target.evasion == 100 {
        return f32::MAX;
    }
    let damage = attack.damage();
    let reduced_damage = if target.armor == 0 {
        damage as f32
    } else {
        ((target.armor as f32) / 100.0) * damage as f32
    };
    let health = target.health.current as f32;
    let potential_result = health - reduced_damage;
    let miss_p = target.evasion as f32 / 100.0;

    return (1.0 - miss_p) * potential_result + miss_p * health;
}

#[test]
fn decide_next_step_test_1() {
    let attacks = vec![
        DirectionalAttack::Punch {
            damage: 23
        },
        DirectionalAttack::Punch {
            damage: 12
        },
    ];
    let mut targets = HashMap::new();
    targets.insert(0, TargetProps {
        health: RangedProp {
            min: 0,
            current: 120,
            max: 120,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 0,
    });
    targets.insert(1, TargetProps {
        health: RangedProp {
            min: 0,
            current: 25,
            max: 100,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 0,
    });
    targets.insert(2, TargetProps {
        health: RangedProp {
            min: 0,
            current: 30,
            max: 300,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 0,
    });

    let expected = StepDecision {
        target_id: 1,
        attack_id: 0,
    };

    assert_eq!(decide_next_step(&attacks, &targets), expected);
}

#[test]
fn decide_next_step_test_2() {
    let attacks = vec![
        DirectionalAttack::Punch {
            damage: 23
        },
        DirectionalAttack::Punch {
            damage: 100
        },
    ];
    let mut targets = HashMap::new();
    targets.insert(0, TargetProps {
        health: RangedProp {
            min: 0,
            current: 110,
            max: 110,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 0,
    });
    targets.insert(1, TargetProps {
        health: RangedProp {
            min: 0,
            current: 65,
            max: 110,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 50,
        evasion: 0,
    });

    let expected = StepDecision {
        target_id: 0,
        attack_id: 1,
    };

    assert_eq!(decide_next_step(&attacks, &targets), expected);
}

#[test]
fn decide_next_step_test_3() {
    let attacks = vec![
        DirectionalAttack::Punch {
            damage: 20
        },
        DirectionalAttack::Punch {
            damage: 100
        },
    ];
    let mut targets = HashMap::new();
    targets.insert(0, TargetProps {
        health: RangedProp {
            min: 0,
            current: 1,
            max: 110,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 100,
    });
    targets.insert(1, TargetProps {
        health: RangedProp {
            min: 0,
            current: 65,
            max: 110,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 80,
    });

    targets.insert(2, TargetProps {
        health: RangedProp {
            min: 0,
            current: 110,
            max: 110,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 0,
    });

    let expected = StepDecision {
        target_id: 2,
        attack_id: 1,
    };

    assert_eq!(decide_next_step(&attacks, &targets), expected);
}

#[test]
fn decide_next_step_test_4() {
    let attacks = vec![
        DirectionalAttack::Punch {
            damage: 20
        },
        DirectionalAttack::Punch {
            damage: 100
        },
    ];
    let mut targets = HashMap::new();
    targets.insert(0, TargetProps {
        health: RangedProp {
            min: 0,
            current: 1,
            max: 110,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 50,
    });
    targets.insert(1, TargetProps {
        health: RangedProp {
            min: 0,
            current: 65,
            max: 110,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 80,
    });

    targets.insert(2, TargetProps {
        health: RangedProp {
            min: 0,
            current: 110,
            max: 110,
        },
        energy: RangedProp {
            min: 0,
            current: 0,
            max: 0,
        },
        armor: 0,
        evasion: 0,
    });

    let expected = StepDecision {
        target_id: 0,
        attack_id: 1,
    };

    assert_eq!(decide_next_step(&attacks, &targets), expected);
}
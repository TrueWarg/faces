use bevy::utils::HashMap;

pub struct DialogStick {
    pub id: usize,
    replicas: Vec<Replica>,
    branching: Option<Branching>,
}


impl Default for DialogStick {
    fn default() -> Self {
        return DialogStick {
            id: 0,
            replicas: vec![],
            branching: None,
        };
    }
}

impl DialogStick {
    pub fn first_replica(&self) -> &Replica {
        return self.get_replica_at(0);
    }

    pub fn last_replica(&self) -> &Replica {
        return self.get_replica_at(self.replicas.len() - 1);
    }

    pub fn get_replica_at(&self, idx: usize) -> &Replica {
        return &self.replicas[idx];
    }

    fn get_branching(&self) -> &Option<Branching> {
        return &self.branching;
    }
}

pub struct Replica {
    pub text: String,
    pub effect: Option<DialogEffect>,
}

impl Replica {
    pub fn from_text(text: String) -> Replica {
        return Replica {
            text,
            effect: None,
        };
    }

    pub fn create(text: String, effect: DialogEffect) -> Replica {
        return Replica {
            text,
            effect: Some(effect),
        };
    }
}

pub struct Branching {
    pub id: usize,
    pub variants: Vec<Variant>,
}

impl Default for Branching {
    fn default() -> Self {
        return Branching {
            id: 0,
            variants: vec![],
        }
    }
}

pub struct Variant {
    pub label: String,
    pub stick: DialogStick,
}

impl Variant{
    fn create(label: String, stick: DialogStick) -> Self {
        return Variant {
            label,
            stick,
        }
    }
}

pub enum DialogEffect {
    ReplaceDialog,
    EndDialog,
}


fn dialog_run_str(dialog: DialogStick, answers: Vec<usize>) -> String {
    let mut result = "".to_string();
    let mut branch_stack = vec![dialog];
    let mut current_stick = 0;
    loop {
        if current_stick >= answers.len() && branch_stack.last().expect("").branching.is_some() {
            break;
        }

        if branch_stack.is_empty() {
            break;
        }
    }

    return result;
}

//         start
//         *
//         |
//         *
//    -->^ ^ ^
//    |  | | |
//    ---* * *
//         end
#[test]
fn test_run() {
    let mut expected = "Hello, mu***r!\n\n".to_string();
    expected += "What are you want?\n\n";
    expected += "-------------------------\n";
    expected += "1. What is your name?\n";
    expected += "2. I will turn your neck!\n";
    expected += "3. I will go.\n";
    expected += "-------------------------\n\n";
    expected += "What is your name?\n\n";
    expected += "My name is RED SPIKE BOY!\n\n";
    expected += "-------------------------\n";
    expected += "1. What is your name?\n";
    expected += "2. I will turn your neck!\n";
    expected += "3. I will go.\n";
    expected += "-------------------------\n\n";
    expected += "I will turn your neck!\n\n";
    expected += "Not well smelt Formidable!\n\n";
}

pub const INVALID_NODE_ID: usize = 0;